use criterion::*;
use std::hint::black_box;

use bitris::prelude::*;

fn set_at(dy: i32) {
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

fn bench_boards(c: &mut Criterion) {
    let mut group = c.benchmark_group("boards");

    for dy in [1, 2, 4, 8] {
        group.bench_function(BenchmarkId::new("set_at", dy), |b| {
            b.iter(|| black_box(set_at(black_box(dy))))
        });
    }

    group.finish()
}

criterion_group!(benches, bench_boards);
criterion_main!(benches);
