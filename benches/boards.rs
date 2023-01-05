use criterion::*;

use bitris::*;

fn set_at(c: &mut Criterion) {
    c.bench_function("set_at", |b| {
        b.iter(|| {
            for dy in 1..30 {
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
        })
    });
}

criterion_group!(benches, set_at);
criterion_main!(benches);
