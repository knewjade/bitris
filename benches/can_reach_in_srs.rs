use std::str::FromStr;

use bitris::prelude::*;
use criterion::*;
use std::hint::black_box;

fn mirror(board: &Board64) -> Board64 {
    let mut freeze = *board;
    freeze.mirror();
    freeze
}

#[derive(Debug)]
struct CanReachBenchmarkData {
    name: String,
    board: Board64,
    spawn: CcPlacement,
    cases: Vec<CanReachCase>,
}

#[derive(Debug)]
struct CanReachCase {
    allow_move: AllowMove,
    goal: CcPlacement,
    expected_fuzzy: bool,
    expected_strictly: bool,
}

fn can_reach(
    allow_move: AllowMove,
    goal: CcPlacement,
    board: Board64,
    spawn: CcPlacement,
    expected: bool,
) {
    assert_eq!(
        srs::can_reach(allow_move, goal.into(), board, spawn.into()),
        expected
    );
}

fn can_reach_strictly(
    allow_move: AllowMove,
    goal: CcPlacement,
    board: Board64,
    spawn: CcPlacement,
    expected: bool,
) {
    assert_eq!(
        srs::can_reach_strictly(allow_move, goal.into(), board, spawn.into()),
        expected
    );
}

fn bench_can_reach_in_srs(c: &mut Criterion) {
    use Shape::*;

    let benchmarks = [
        CanReachBenchmarkData {
            name: "z_kick1".to_string(),
            board: Board64::from_str(
                "\
                ..........\
                ...#......\
                #####.####\
                ####..####\
                ####.#####\
                ",
            )
            .unwrap(),
            spawn: Z.with(Orientation::North).with(cc(4, 20)),
            cases: vec![
                CanReachCase {
                    allow_move: AllowMove::Softdrop,
                    goal: Z.with(Orientation::East).with(cc(4, 1)),
                    expected_fuzzy: true,
                    expected_strictly: true,
                },
                CanReachCase {
                    allow_move: AllowMove::Softdrop,
                    goal: Z.with(Orientation::West).with(cc(5, 1)),
                    expected_fuzzy: true,
                    expected_strictly: false,
                },
                CanReachCase {
                    allow_move: AllowMove::Harddrop,
                    goal: Z.with(Orientation::West).with(cc(5, 1)),
                    expected_fuzzy: false,
                    expected_strictly: false,
                },
            ],
        },
        CanReachBenchmarkData {
            name: "i_kick1".to_string(),
            board: Board64::from_str(
                "\
                ####.#####\
                ####.#####\
                #........#\
                ####.#####\
                ####.#####\
                ####.#####\
                ",
            )
            .unwrap(),
            spawn: I.with(Orientation::North).with(cc(4, 20)),
            cases: vec![
                CanReachCase {
                    allow_move: AllowMove::Softdrop,
                    goal: I.with(Orientation::North).with(cc(2, 3)),
                    expected_fuzzy: true,
                    expected_strictly: true,
                },
                CanReachCase {
                    allow_move: AllowMove::Softdrop,
                    goal: I.with(Orientation::South).with(cc(7, 3)),
                    expected_fuzzy: true,
                    expected_strictly: true,
                },
                CanReachCase {
                    allow_move: AllowMove::Harddrop,
                    goal: I.with(Orientation::South).with(cc(7, 3)),
                    expected_fuzzy: false,
                    expected_strictly: false,
                },
                CanReachCase {
                    allow_move: AllowMove::Harddrop,
                    goal: I.with(Orientation::West).with(cc(4, 3)),
                    expected_fuzzy: true,
                    expected_strictly: true,
                },
            ],
        },
        CanReachBenchmarkData {
            name: "i_kick2".to_string(),
            board: Board64::from_str(
                "\
                .....#####\
                .....#####\
                ##...#####\
                #....#####\
                ",
            )
            .unwrap(),
            spawn: I.with(Orientation::North).with(cc(4, 20)),
            cases: vec![
                CanReachCase {
                    allow_move: AllowMove::Softdrop,
                    goal: I.with(Orientation::South).with(cc(3, 0)),
                    expected_fuzzy: true,
                    expected_strictly: true,
                },
                CanReachCase {
                    allow_move: AllowMove::Softdrop,
                    goal: I.with(Orientation::North).with(cc(2, 0)),
                    expected_fuzzy: true,
                    expected_strictly: false,
                },
            ],
        },
        CanReachBenchmarkData {
            name: "o1".to_string(),
            board: Board64::from_str(
                "\
                ..........\
                ##........\
                #.........\
                #.........\
                ",
            )
            .unwrap(),
            spawn: O.with(Orientation::North).with(cc(4, 20)),
            cases: vec![
                CanReachCase {
                    allow_move: AllowMove::Softdrop,
                    goal: O.with(Orientation::North).with(cc(1, 0)),
                    expected_fuzzy: true,
                    expected_strictly: true,
                },
                CanReachCase {
                    allow_move: AllowMove::Harddrop,
                    goal: O.with(Orientation::North).with(cc(1, 0)),
                    expected_fuzzy: false,
                    expected_strictly: false,
                },
            ],
        },
    ];

    let mut group = c.benchmark_group("can_reach".to_string());
    for allow_move in [AllowMove::Softdrop, AllowMove::Harddrop] {
        benchmarks.iter().for_each(|benchmark| {
            benchmark
                .cases
                .iter()
                .filter(|case| case.allow_move == allow_move)
                .for_each(|case| {
                    group.bench_function(
                        BenchmarkId::new(
                            allow_move.to_string().to_lowercase(),
                            format!(
                                "fuzzy_{}_{}_{}",
                                benchmark.name, case.goal.piece, case.goal.position
                            )
                            .to_string()
                            .replace(" ", "")
                            .to_string()
                            .replace("-", "_")
                            .to_lowercase(),
                        ),
                        |b| {
                            b.iter(|| {
                                can_reach(
                                    allow_move,
                                    case.goal,
                                    benchmark.board,
                                    benchmark.spawn,
                                    case.expected_fuzzy,
                                );
                                black_box(());
                            })
                        },
                    );
                    group.bench_function(
                        BenchmarkId::new(
                            allow_move.to_string().to_lowercase(),
                            format!(
                                "strictly_{}_{}_{}",
                                benchmark.name, case.goal.piece, case.goal.position
                            )
                            .to_string()
                            .replace(" ", "")
                            .to_string()
                            .replace("-", "_")
                            .to_lowercase(),
                        ),
                        |b| {
                            b.iter(|| {
                                can_reach_strictly(
                                    allow_move,
                                    case.goal,
                                    benchmark.board,
                                    benchmark.spawn,
                                    case.expected_strictly,
                                );
                                black_box(());
                            })
                        },
                    );
                })
        });
    }
}

criterion_group!(benches, bench_can_reach_in_srs);
criterion_main!(benches);
