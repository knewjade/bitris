use criterion::*;
use std::hint::black_box;

use bitris::prelude::*;
use bitris::set_at2;

fn call_set_at(dy: i32) {
    let mut board = Board64::default();
    let ceiling = board.ceiling() as i32;
    let mut y = 0;
    while y < ceiling {
        for x in 0..10 {
            board.set_at(xy(x, y));
        }
        y += dy;
    }
    board.clear_lines();
    assert_eq!(board.count_blocks(), 0);
}

fn call_set_at2(dy: i32) {
    assert_eq!(set_at2(dy), 0);
}

fn bench_boards(c: &mut Criterion) {
    let mut group = c.benchmark_group("boards");

    for dy in [1, 2] {
        group.bench_function(BenchmarkId::new("set_at", dy), |b| {
            b.iter(|| {
                call_set_at(dy);
                black_box(())
            })
        });
        group.bench_function(BenchmarkId::new("set_at2", dy), |b| {
            b.iter(|| {
                call_set_at2(dy);
                black_box(())
            })
        });
    }

    group.finish()
}

criterion_group!(
    name = benches;
    config = Criterion::default();
    targets = bench_boards);
criterion_main!(benches);
