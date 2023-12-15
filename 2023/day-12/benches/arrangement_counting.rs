use criterion::{black_box, criterion_group, criterion_main, Criterion};

use day_12::count_valid_arrangements;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function(
        "Counting arrangements 1", 
        |b| b.iter(|| 
            count_valid_arrangements(
                black_box("????.######..#####.?????.######..#####.?????.######..#####.?????.######..#####.?????.######..#####."),
                black_box(&vec![1,6,5,1,6,5,1,6,5,1,6,5,1,6,5])
        ))
    );

    c.bench_function(
        "Counting arrangements 2", 
        |b| b.iter(|| 
            count_valid_arrangements(
                black_box("?###??????????###??????????###??????????###??????????###????????"),
                black_box(&vec![3,2,1,3,2,1,3,2,1,3,2,1,3,2,1])
        ))
    );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
