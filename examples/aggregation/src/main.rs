//! Aggregate signatures from multiple contributors over the BN254 curve.
//!
//! # Usage (3 of 4 Threshold)
//!
//! _To run this example, you must first install [Rust](https://www.rust-lang.org/tools/install) and [protoc](https://grpc.io/docs/protoc-installation)._
//!
//! ## Orchestrator
//! ```bash
//! cargo run --release -- --me 0@3000 --participants 0,1,2,3,4 --contributors 1,2,3,4
//! ```
//!
//! ## Contributor 1
//! ```bash
//! cargo run --release -- --bootstrappers 0@127.0.0.1:3000 --me 1@3001 --participants 0,1,2,3,4  --orchestrator 0 --contributors 1,2,3,4
//! ```
//!
//! ## Contributor 2
//! ```bash
//! cargo run --release -- --bootstrappers 0@127.0.0.1:3000 --me 2@3002 --participants 0,1,2,3,4  --orchestrator 0 --contributors 1,2,3,4
//! ```
//!
//! ## Contributor 3
//! ```bash
//! cargo run --release -- --bootstrappers 0@127.0.0.1:3000 --me 3@3003 --participants 0,1,2,3,4  --orchestrator 0 --contributors 1,2,3,4
//! ```
//!
//! ## Contributor 4
//!
//! ```bash
//! cargo run --release -- --bootstrappers 0@127.0.0.1:3000 --me 4@3004 --participants 0,1,2,3,4 --orchestrator 0 --contributors 1,2,3,4
//! ```

mod bn254;
mod handlers;
mod bindings;

use ark_ff::{Fp, PrimeField};
use ark_bn254::Fr;
use bn254::{Bn254, PrivateKey};
use clap::{value_parser, Arg, Command};
use commonware_cryptography::Scheme;
use commonware_p2p::authenticated::{self, Network};
use commonware_runtime::{
    tokio::{self, Executor},
    Runner, Spawner,
};
use commonware_utils::quorum;
use governor::Quota;
use prometheus_client::registry::Registry;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    num::NonZeroU32,
};
use std::{str::FromStr, time::Duration};
use tracing::info;
use eigen_crypto_bls::{convert_to_g1_point, convert_to_g2_point};
use alloy_primitives::{address, hex_literal::hex};
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
struct KeyConfig {
    private_key: String,
}

fn load_key_from_file(path: &str) -> String {
    let contents = fs::read_to_string(path).expect("Could not read key file");
    let config: KeyConfig = serde_yaml::from_str(&contents).expect("Could not parse key file");
    config.private_key
}

fn get_signer(key: &str) -> Bn254 {
    let fr = Fr::from_str(key).expect("Invalid decimal string for private key");
    let key = PrivateKey::from(fr);
    <Bn254 as Scheme>::from(key).expect("Failed to create signer")
}

// Unique namespace to avoid message replay attacks.
const APPLICATION_NAMESPACE: &[u8] = b"_COMMONWARE_AGGREGATION_";

fn main() {
    // Initialize runtime
    let runtime_cfg = tokio::Config::default();
    let (executor, runtime) = Executor::init(runtime_cfg.clone());

    // Parse arguments
    let matches = Command::new("commonware-aggregation")
        .about("generate and verify BN254 Multi-Signatures")
        .arg(
            Arg::new("bootstrappers")
                .long("bootstrappers")
                .required(false)
                .value_delimiter(',')
                .value_parser(value_parser!(String)),
        )
        .arg(
            Arg::new("key-file")
                .long("key-file")
                .required(true)
                .help("Path to the YAML file containing the private key")
        )
        .arg(
            Arg::new("port")
                .long("port")
                .required(true)
                .help("Port to run the service on")
        )
        .arg(
            Arg::new("participants")
                .long("participants")
                .required(true)
                .value_delimiter(',')
                .value_parser(value_parser!(String))
                .help("File paths for all participant key files"),
        )
        .arg(
            Arg::new("orchestrator")
                .long("orchestrator")
                .required(false)
                .help("Path to orchestrator key file"),
        )
        .arg(
            Arg::new("contributors")
                .long("contributors")
                .required(true)
                .value_delimiter(',')
                .value_parser(value_parser!(String))
                .help("File paths for contributor key files"),
        )
        .get_matches();

    // Create logger
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    // Configure my identity
    let key_file = matches.get_one::<String>("key-file").expect("Please provide key file");
    let port = matches.get_one::<String>("port").expect("Please provide port");
    let key = load_key_from_file(key_file);
    let me = format!("{}@{}", key, port);
    let parts = me.split('@').collect::<Vec<&str>>();
    if parts.len() != 2 {
        panic!("Identity not well-formed");
    }
    let key = parts[0];
    let signer = get_signer(key);
    tracing::info!(key = ?signer.public_key(), "loaded signer");
    let public_key = signer.public_g1();
    let (apk, _, _) = bn254::get_points(&[public_key], &[signer.public_key()], &[]).unwrap();
    let g1_point = convert_to_g1_point(apk).unwrap();
    println!("public key G1 coordinates: ({}, {})", g1_point.X, g1_point.Y);
    println!("key: {}", key);
    println!("parts: {:?}", parts);
    println!("me: {:?}", me);
    // std::process::exit(0);

    // Configure my port
    let port = parts[1].parse::<u16>().expect("Port not well-formed");
    tracing::info!(port, "loaded port");

    // Configure allowed peers
    let mut recipients = Vec::new();
    let participants = matches
        .get_many::<String>("participants")
        .expect("Please provide allowed keys");
    if participants.len() == 0 {
        panic!("Please provide at least one participant");
    }
    for peer_file in participants {
        let peer_key = load_key_from_file(peer_file);
        let verifier = get_signer(&peer_key).public_key();
        tracing::info!(key = ?verifier, "registered authorized key",);
        recipients.push(verifier);
    }

    // Configure bootstrappers (if provided)
    let bootstrappers = matches.get_many::<String>("bootstrappers");
    let mut bootstrapper_identities = Vec::new();
    if let Some(bootstrappers) = bootstrappers {
        for bootstrapper in bootstrappers {
            let parts = bootstrapper.split('@').collect::<Vec<&str>>();
            let verifier = get_signer(parts[0]).public_key();
            let bootstrapper_address =
                SocketAddr::from_str(parts[1]).expect("Bootstrapper address not well-formed");
            bootstrapper_identities.push((verifier, bootstrapper_address));
        }
    }

    // Configure network
    const MAX_MESSAGE_SIZE: usize = 1024 * 1024; // 1 MB
    let p2p_cfg = authenticated::Config::aggressive(
        signer.clone(),
        APPLICATION_NAMESPACE,
        Arc::new(Mutex::new(Registry::default())),
        SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), port),
        bootstrapper_identities.clone(),
        MAX_MESSAGE_SIZE,
    );

    // Start runtime
    executor.start(async move {
        let (mut network, mut oracle) = Network::new(runtime.clone(), p2p_cfg);

        // Provide authorized peers
        //
        // In a real-world scenario, this would be updated as new peer sets are created (like when
        // the composition of a validator set changes).
        oracle.register(0, recipients).await;

        // Parse contributors
        let mut contributors = Vec::new();
        let mut contributors_map = HashMap::new();
        let participants = matches
            .get_many::<String>("contributors")
            .expect("Please provide contributors");
        if participants.len() == 0 {
            panic!("Please provide at least one contributor");
        }
        for peer_file in participants {
            let peer_key = load_key_from_file(peer_file);
            let signer = get_signer(&peer_key);
            let verifier = signer.public_key();
            let verifier_g1 = signer.public_g1();
            tracing::info!(key = ?verifier, "registered contributor",);
            contributors.push(verifier.clone());
            contributors_map.insert(verifier, verifier_g1);
        }

        // Infer threshold
        let threshold = 3; //hardcoded for now

        // Check if I am the orchestrator
        const DEFAULT_MESSAGE_BACKLOG: usize = 256;
        const COMPRESSION_LEVEL: Option<i32> = Some(3);
        const AGGREGATION_FREQUENCY: Duration = Duration::from_secs(10);
        if let Some(orchestrator_file) = matches.get_one::<String>("orchestrator") {
            // Create contributor
            let (sender, receiver) = network.register(
                0,
                Quota::per_second(NonZeroU32::new(10).unwrap()),
                DEFAULT_MESSAGE_BACKLOG,
                COMPRESSION_LEVEL,
            );
            let orchestrator_key = load_key_from_file(orchestrator_file);
            let orchestrator = get_signer(&orchestrator_key).public_key();
            let contributor =
                handlers::Contributor::new(orchestrator, signer, contributors, threshold as usize);
            runtime.spawn("contributor", contributor.run(sender, receiver));
        } else {
            let (sender, receiver) = network.register(
                0,
                Quota::per_second(NonZeroU32::new(10).unwrap()),
                DEFAULT_MESSAGE_BACKLOG,
                COMPRESSION_LEVEL,
            );
            let orchestrator = handlers::Orchestrator::new(
                runtime.clone(),
                signer,
                AGGREGATION_FREQUENCY,
                contributors,
                contributors_map,
                threshold as usize,
            );
            runtime.spawn("orchestrator", orchestrator.run(sender, receiver));
        }
        network.run().await;
    });
}
