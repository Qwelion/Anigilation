// benches/benchmark.rs (AI, Gemini 3.1 pro)
use {
    annihilation::step_1,
    criterion::{criterion_group, criterion_main, BatchSize, Criterion},
    std::{fs, path::Path},
};

fn bench_wipe(c: &mut Criterion) {
    let mut group = c.benchmark_group("wipe_group");
    group.sample_size(10); //10 reps
    
    group.bench_function("wipe_100_files", |b| b.iter_batched(
        || {
            let _ = fs::create_dir_all("bench_dir");
            (0..100).for_each(|i| { let _ = fs::write(format!("bench_dir/{i}"), [0u8; 1024]); });
            "bench_dir"
        },
        |dir| { let _ = step_1(Path::new(dir)); },
        BatchSize::SmallInput,
    ));
    
    group.finish();
    let _ = fs::remove_dir_all("bench_dir");
}

criterion_group!(benches, bench_wipe);
criterion_main!(benches);