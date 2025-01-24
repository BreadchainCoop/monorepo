use commonware_cryptography::{Hasher, Sha256};
use criterion::{criterion_group, Criterion};
use rand::{thread_rng, Rng};

fn benchmark_hash_message(c: &mut Criterion) {
    for message_length in [100, 1000, 10000].into_iter() {
        let mut msg = vec![0u8; message_length];
        thread_rng().fill(msg.as_mut_slice());
        let msg = msg.as_slice();
        c.bench_function(&format!("{}/msg_len={}", module_path!(), msg.len()), |b| {
            b.iter(|| {
                let mut hasher = Sha256::new();
                hasher.update(msg);
                hasher.finalize();
            });
        });
    }
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(10);
    targets = benchmark_hash_message
}
