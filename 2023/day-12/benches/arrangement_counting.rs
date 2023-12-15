use criterion::{black_box, criterion_group, criterion_main, Criterion};

use day_12::count_valid_arrangements;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function(
        "Counting arrangements", 
        |b| b.iter(|| 
            count_valid_arrangements(
                black_box("????.######..#####.?????.######..#####.?????.######..#####.?????.######..#####.?????.######..#####."),
                black_box(&vec![1,6,5,1,6,5,1,6,5,1,6,5,1,6,5])
        ))
    );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
