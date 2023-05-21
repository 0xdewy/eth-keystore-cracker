use criterion::{criterion_group, criterion_main, Criterion};
use keystore_cracker::decrypt_and_create_keystore;
use std::path::PathBuf;
use std::sync::Arc;

fn benchmark_decrypt_and_create_keystore(c: &mut Criterion) {
    let input_dir = Arc::new(PathBuf::from("./test")); // replace with your path
    let output_dir = Arc::new(PathBuf::from("./output")); // replace with your path
    let passwords_file = Arc::new(PathBuf::from("./test/test_passwords")); // replace with your path
    let new_password = String::from("insecure");

    c.bench_function("decrypt_and_create_keystore", |b| {
        b.iter(|| {
            decrypt_and_create_keystore(
                &input_dir,
                &output_dir,
                &passwords_file,
                new_password.clone(),
            )
            .expect("Failed to decrypt and create keystore");
        })
    });
}

criterion_group!(benches, benchmark_decrypt_and_create_keystore);
criterion_main!(benches);
