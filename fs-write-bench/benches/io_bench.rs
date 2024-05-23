use std::io::Write;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use once_cell::sync::Lazy;
use rand::{thread_rng, RngCore};

static KB_8: Lazy<Vec<u8>> = Lazy::new(|| {
    let mut rng = thread_rng();
    let mut bs = vec![0_u8; 8192];
    rng.fill_bytes(&mut bs);
    bs
});

static KB_64: Lazy<Vec<u8>> = Lazy::new(|| {
    let mut rng = thread_rng();
    let mut bs = vec![0_u8; 8192];
    rng.fill_bytes(&mut bs);
    bs
});

static KB_1024: Lazy<Vec<u8>> = Lazy::new(|| {
    let mut rng = thread_rng();
    let mut bs = vec![0_u8; 8192];
    rng.fill_bytes(&mut bs);
    bs
});

fn fs_write(bs: &[u8]) {
    let mut f = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .append(false)
        .open("tmp.bin")
        .unwrap();
    f.write_all(&bs).unwrap();
}

async fn async_write(bs: &[u8]) {
    use tokio::io::AsyncWriteExt;
    let mut f = tokio::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .append(false)
        .open("tmp.bin")
        .await
        .unwrap();
    f.write_all(bs).await.unwrap();
}

fn sync_benchmark_8kb(c: &mut Criterion) {
    c.bench_function("fs write 8KB", |b| b.iter(|| fs_write(black_box(&KB_8))));
}

fn async_fs_benchmark_8kb(c: &mut Criterion) {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("build tokio runtime");
    c.bench_function("async fs write 8KB", |b| {
        b.iter(|| {
            rt.block_on(async {
                async_write(black_box(&KB_8)).await;
            })
        })
    });
}

fn sync_benchmark_64kb(c: &mut Criterion) {
    c.bench_function("fs write 64KB", |b| b.iter(|| fs_write(black_box(&KB_64))));
}

fn async_fs_benchmark_64kb(c: &mut Criterion) {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("build tokio runtime");
    c.bench_function("async fs write 64KB", |b| {
        b.iter(|| {
            rt.block_on(async {
                async_write(black_box(&KB_64)).await;
            })
        })
    });
}

fn sync_benchmark_1024kb(c: &mut Criterion) {
    c.bench_function("fs write 1024KB", |b| {
        b.iter(|| fs_write(black_box(&KB_1024)))
    });
}

fn async_fs_benchmark_1024kb(c: &mut Criterion) {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("build tokio runtime");
    c.bench_function("async fs write 1024KB", |b| {
        b.iter(|| {
            rt.block_on(async {
                async_write(black_box(&KB_1024)).await;
            })
        })
    });
}

criterion_group!(benches_8kb, sync_benchmark_8kb, async_fs_benchmark_8kb);
criterion_group!(benches_64kb, sync_benchmark_64kb, async_fs_benchmark_64kb);
criterion_group!(
    benches_1024kb,
    sync_benchmark_1024kb,
    async_fs_benchmark_1024kb
);
criterion_main!(benches_8kb, benches_64kb, benches_1024kb);
