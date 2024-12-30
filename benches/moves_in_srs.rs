use std::str::FromStr;

use criterion::*;
use std::hint::black_box;

use bitris::prelude::*;

#[derive(Debug)]
struct MovesBenchmarkData {
    name: String,
    board: Board64,
    spawn: CcPosition,
    expected_all_moves: Vec<(Shape, usize)>,
    expected_minimized_moves: Vec<(Shape, usize)>,
}

fn all_moves(board: Board64, shape: Shape, spawn: CcPosition, expected: usize) {
    let spawn = shape.with(Orientation::North).with(spawn);
    let moves = srs::generate_all_moves(AllowMove::Softdrop, board, spawn.into());
    assert_eq!(moves.len(), expected);
}

fn minimized_moves(board: Board64, shape: Shape, spawn: CcPosition, expected: usize) {
    let spawn = shape.with(Orientation::North).with(spawn);
    let moves = srs::generate_minimized_moves(AllowMove::Softdrop, board, spawn.into());
    assert_eq!(moves.len(), expected);
}

fn bench_moves_in_srs(c: &mut Criterion) {
    use Shape::*;
    let mirror = |board: Board64| {
        let mut freeze = board;
        freeze.mirror();
        freeze
    };

    let benchmarks = vec![
        MovesBenchmarkData {
            name: "empty".to_string(),
            board: Board64::default(),
            spawn: cc(4, 20),
            expected_all_moves: vec![
                (T, 34),
                (I, 34),
                (L, 34),
                (J, 34),
                (S, 34),
                (Z, 34),
                (O, 36),
            ],
            expected_minimized_moves: vec![
                (T, 34),
                (I, 17),
                (L, 34),
                (J, 34),
                (S, 17),
                (Z, 17),
                (O, 9),
            ],
        },
        MovesBenchmarkData {
            name: "pco".to_string(),
            board: Board64::from_str(
                "\
                ##.....###\
                ##....####\
                ##...#####\
                ##....####\
            ",
            )
            .unwrap(),
            spawn: cc(4, 20),
            expected_all_moves: vec![
                (T, 35),
                (I, 35),
                (L, 35),
                (J, 35),
                (S, 34),
                (Z, 36),
                (O, 36),
            ],
            expected_minimized_moves: vec![
                (T, 35),
                (I, 18),
                (L, 35),
                (J, 35),
                (S, 17),
                (Z, 18),
                (O, 9),
            ],
        },
        MovesBenchmarkData {
            name: "mirrored-pco".to_string(),
            board: Board64::from_str(
                "\
                ##.....###\
                ##....####\
                ##...#####\
                ##....####\
            ",
            )
            .map(mirror)
            .unwrap(),
            spawn: cc(4, 20),
            expected_all_moves: vec![
                (T, 35),
                (I, 35),
                (L, 35),
                (J, 35),
                (S, 36),
                (Z, 34),
                (O, 36),
            ],
            expected_minimized_moves: vec![
                (T, 35),
                (I, 18),
                (L, 35),
                (J, 35),
                (S, 18),
                (Z, 17),
                (O, 9),
            ],
        },
        MovesBenchmarkData {
            name: "loop".to_string(),
            board: Board64::from_str(
                "\
                ......####\
                .........#\
                ########.#\
                #........#\
                #.######.#\
                #.######.#\
                #........#\
                #.########\
                #........#\
                #.######.#\
                #.######.#\
                #........#\
                ########.#\
                ########.#\
                #........#\
                #.######.#\
                #.######.#\
                #........#\
                .#########\
            ",
            )
            .unwrap(),
            spawn: cc(4, 20),
            expected_all_moves: vec![
                (T, 35),
                (I, 108),
                (L, 35),
                (J, 36),
                (S, 34),
                (Z, 36),
                (O, 36),
            ],
            expected_minimized_moves: vec![
                (T, 35),
                (I, 54),
                (L, 35),
                (J, 36),
                (S, 17),
                (Z, 18),
                (O, 9),
            ],
        },
        MovesBenchmarkData {
            name: "mirrored-loop".to_string(),
            board: Board64::from_str(
                "\
                ......####\
                .........#\
                ########.#\
                #........#\
                #.######.#\
                #.######.#\
                #........#\
                #.########\
                #........#\
                #.######.#\
                #.######.#\
                #........#\
                ########.#\
                ########.#\
                #........#\
                #.######.#\
                #.######.#\
                #........#\
                .#########\
            ",
            )
            .map(mirror)
            .unwrap(),
            spawn: cc(4, 20),
            expected_all_moves: vec![
                (T, 35),
                (I, 108),
                (L, 36),
                (J, 35),
                (S, 36),
                (Z, 34),
                (O, 36),
            ],
            expected_minimized_moves: vec![
                (T, 35),
                (I, 54),
                (L, 36),
                (J, 35),
                (S, 18),
                (Z, 17),
                (O, 9),
            ],
        },
        MovesBenchmarkData {
            name: "hatetris".to_string(),
            board: Board64::from_str(
                "\
                ##........\
                ##....####\
                ##..######\
                #.....####\
                ###....###\
                ####..####\
                ###.....##\
                #........#\
                #.......##\
                ##.....###\
                ####..####\
                ##....####\
                ###..#####\
                .##.#.###.\
            ",
            )
            .unwrap(),
            spawn: cc(4, 20),
            expected_all_moves: vec![
                (T, 72),
                (I, 58),
                (L, 65),
                (J, 65),
                (S, 67),
                (Z, 66),
                (O, 36),
            ],
            expected_minimized_moves: vec![
                (T, 72),
                (I, 30),
                (L, 65),
                (J, 65),
                (S, 34),
                (Z, 33),
                (O, 9),
            ],
        },
        MovesBenchmarkData {
            name: "mirrored-hatetris".to_string(),
            board: Board64::from_str(
                "\
                ##........\
                ##....####\
                ##..######\
                #.....####\
                ###....###\
                ####..####\
                ###.....##\
                #........#\
                #.......##\
                ##.....###\
                ####..####\
                ##....####\
                ###..#####\
                .##.#.###.\
            ",
            )
            .map(mirror)
            .unwrap(),
            spawn: cc(4, 20),
            expected_all_moves: vec![
                (T, 72),
                (I, 58),
                (L, 65),
                (J, 65),
                (S, 66),
                (Z, 67),
                (O, 36),
            ],
            expected_minimized_moves: vec![
                (T, 72),
                (I, 30),
                (L, 65),
                (J, 65),
                (S, 33),
                (Z, 34),
                (O, 9),
            ],
        },
        MovesBenchmarkData {
            name: "side-4-wide".to_string(),
            board: Board64::from_str(
                "\
                ....######\
                ....######\
                ....######\
                ....######\
                ....######\
                ....######\
                ....######\
                ....######\
                ....######\
                ....######\
                ....######\
                ....######\
                ....######\
                ....######\
                ....######\
                ....######\
                ....######\
                ....######\
                ....######\
                .#########\
            ",
            )
            .unwrap(),
            spawn: cc(4, 20),
            expected_all_moves: vec![
                (T, 34),
                (I, 34),
                (L, 34),
                (J, 34),
                (S, 34),
                (Z, 34),
                (O, 36),
            ],
            expected_minimized_moves: vec![
                (T, 34),
                (I, 17),
                (L, 34),
                (J, 34),
                (S, 17),
                (Z, 17),
                (O, 9),
            ],
        },
        MovesBenchmarkData {
            name: "mirrored-side-4-wide".to_string(),
            board: Board64::from_str(
                "\
                ....######\
                ....######\
                ....######\
                ....######\
                ....######\
                ....######\
                ....######\
                ....######\
                ....######\
                ....######\
                ....######\
                ....######\
                ....######\
                ....######\
                ....######\
                ....######\
                ....######\
                ....######\
                ....######\
                .#########\
            ",
            )
            .map(mirror)
            .unwrap(),
            spawn: cc(4, 20),
            expected_all_moves: vec![
                (T, 34),
                (I, 34),
                (L, 34),
                (J, 34),
                (S, 34),
                (Z, 34),
                (O, 36),
            ],
            expected_minimized_moves: vec![
                (T, 34),
                (I, 17),
                (L, 34),
                (J, 34),
                (S, 17),
                (Z, 17),
                (O, 9),
            ],
        },
        MovesBenchmarkData {
            name: "center-4-wide".to_string(),
            board: Board64::from_str(
                "\
                ###....###\
                ###....###\
                ###....###\
                ###....###\
                ###....###\
                ###....###\
                ###....###\
                ###....###\
                ###....###\
                ###....###\
                ###....###\
                ###....###\
                ###....###\
                ###....###\
                ###....###\
                ###....###\
                ###....###\
                ###....###\
                ###....###\
                ###.######\
            ",
            )
            .unwrap(),
            spawn: cc(4, 20),
            expected_all_moves: vec![
                (T, 34),
                (I, 34),
                (L, 34),
                (J, 34),
                (S, 34),
                (Z, 34),
                (O, 36),
            ],
            expected_minimized_moves: vec![
                (T, 34),
                (I, 17),
                (L, 34),
                (J, 34),
                (S, 17),
                (Z, 17),
                (O, 9),
            ],
        },
        MovesBenchmarkData {
            name: "mirrored-center-4-wide".to_string(),
            board: Board64::from_str(
                "\
                ###....###\
                ###....###\
                ###....###\
                ###....###\
                ###....###\
                ###....###\
                ###....###\
                ###....###\
                ###....###\
                ###....###\
                ###....###\
                ###....###\
                ###....###\
                ###....###\
                ###....###\
                ###....###\
                ###....###\
                ###....###\
                ###....###\
                ###.######\
            ",
            )
            .map(mirror)
            .unwrap(),
            spawn: cc(4, 20),
            expected_all_moves: vec![
                (T, 34),
                (I, 34),
                (L, 34),
                (J, 34),
                (S, 34),
                (Z, 34),
                (O, 36),
            ],
            expected_minimized_moves: vec![
                (T, 34),
                (I, 17),
                (L, 34),
                (J, 34),
                (S, 17),
                (Z, 17),
                (O, 9),
            ],
        },
        MovesBenchmarkData {
            name: "dt".to_string(),
            board: Board64::from_str(
                "\
                .....#....\
                ..##.##...\
                ...####..#\
                ##.#######\
                #..#######\
                #...######\
                ##.#######\
                ##.#######\
            ",
            )
            .unwrap(),
            spawn: cc(4, 20),
            expected_all_moves: vec![
                (T, 39),
                (I, 34),
                (L, 35),
                (J, 37),
                (S, 34),
                (Z, 40),
                (O, 36),
            ],
            expected_minimized_moves: vec![
                (T, 39),
                (I, 17),
                (L, 35),
                (J, 37),
                (S, 17),
                (Z, 20),
                (O, 9),
            ],
        },
        MovesBenchmarkData {
            name: "mirrored-dt".to_string(),
            board: Board64::from_str(
                "\
                .....#....\
                ..##.##...\
                ...####..#\
                ##.#######\
                #..#######\
                #...######\
                ##.#######\
                ##.#######\
            ",
            )
            .map(mirror)
            .unwrap(),
            spawn: cc(4, 20),
            expected_all_moves: vec![
                (T, 39),
                (I, 34),
                (L, 37),
                (J, 35),
                (S, 40),
                (Z, 34),
                (O, 36),
            ],
            expected_minimized_moves: vec![
                (T, 39),
                (I, 17),
                (L, 37),
                (J, 35),
                (S, 20),
                (Z, 17),
                (O, 9),
            ],
        },
        MovesBenchmarkData {
            name: "rotating-early".to_string(),
            board: Board64::from_str(
                "\
                .#........\
                ........##\
                ...#.....#\
                #.######.#\
                .........#\
                .........#\
                .........#\
                .........#\
                .........#\
                .........#\
                .........#\
                .........#\
                .........#\
                .........#\
                .........#\
                .........#\
                .........#\
                .........#\
                .........#\
                .........#\
            ",
            )
            .unwrap(),
            spawn: cc(4, 20),
            expected_all_moves: vec![
                (T, 73),
                (I, 70),
                (L, 72),
                (J, 69),
                (S, 70),
                (Z, 70),
                (O, 36),
            ],
            expected_minimized_moves: vec![
                (T, 73),
                (I, 35),
                (L, 72),
                (J, 69),
                (S, 35),
                (Z, 35),
                (O, 9),
            ],
        },
        MovesBenchmarkData {
            name: "mirrored-rotating-early".to_string(),
            board: Board64::from_str(
                "\
                .#........\
                ........##\
                ...#.....#\
                #.######.#\
                .........#\
                .........#\
                .........#\
                .........#\
                .........#\
                .........#\
                .........#\
                .........#\
                .........#\
                .........#\
                .........#\
                .........#\
                .........#\
                .........#\
                .........#\
                .........#\
            ",
            )
            .map(mirror)
            .unwrap(),
            spawn: cc(4, 20),
            expected_all_moves: vec![
                (T, 73),
                (I, 70),
                (L, 69),
                (J, 72),
                (S, 70),
                (Z, 70),
                (O, 36),
            ],
            expected_minimized_moves: vec![
                (T, 73),
                (I, 35),
                (L, 69),
                (J, 72),
                (S, 35),
                (Z, 35),
                (O, 9),
            ],
        },
        MovesBenchmarkData {
            name: "randomize".to_string(),
            board: Board64::from_str(
                "\
                ..........\
                ....#.....\
                #.......#.\
                ..#....#..\
                .....#...#\
                .#..#.#...\
                .#.......#\
                ..#.....##\
                .....#....\
                .......#..\
                ..#.......\
                #....##...\
                ..........\
                ...#....#.\
                .#......#.\
                ....#.....\
                ..#....#..\
                .........#\
                .#..#.#.#.\
                ..........\
            ",
            )
            .unwrap(),
            spawn: cc(4, 20),
            expected_all_moves: vec![
                (T, 134),
                (I, 138),
                (L, 137),
                (J, 151),
                (S, 132),
                (Z, 151),
                (O, 44),
            ],
            expected_minimized_moves: vec![
                (T, 134),
                (I, 73),
                (L, 137),
                (J, 151),
                (S, 68),
                (Z, 78),
                (O, 11),
            ],
        },
        MovesBenchmarkData {
            name: "mirrored-randomize".to_string(),
            board: Board64::from_str(
                "\
                ..........\
                ....#.....\
                #.......#.\
                ..#....#..\
                .....#...#\
                .#..#.#...\
                .#.......#\
                ..#.....##\
                .....#....\
                .......#..\
                ..#.......\
                #....##...\
                ..........\
                ...#....#.\
                .#......#.\
                ....#.....\
                ..#....#..\
                .........#\
                .#..#.#.#.\
                ..........\
            ",
            )
            .map(mirror)
            .unwrap(),
            spawn: cc(4, 20),
            expected_all_moves: vec![
                (T, 134),
                (I, 139),
                (L, 151),
                (J, 137),
                (S, 151),
                (Z, 132),
                (O, 44),
            ],
            expected_minimized_moves: vec![
                (T, 134),
                (I, 74),
                (L, 151),
                (J, 137),
                (S, 78),
                (Z, 68),
                (O, 11),
            ],
        },
    ];

    benchmarks.iter().for_each(|benchmark| {
        let mut group = c.benchmark_group(format!("moves_{}", benchmark.name));

        // all moves
        for (shape, expected) in &benchmark.expected_all_moves {
            group.bench_function(BenchmarkId::new("all_moves", shape), |b| {
                b.iter(|| {
                    all_moves(benchmark.board, *shape, benchmark.spawn, *expected);
                    black_box(());
                })
            });
        }

        // minimized moves
        for (shape, expected) in &benchmark.expected_minimized_moves {
            group.bench_function(BenchmarkId::new("minimized_moves", shape), |b| {
                b.iter(|| {
                    minimized_moves(benchmark.board, *shape, benchmark.spawn, *expected);
                    black_box(());
                })
            });
        }

        group.finish()
    });
}

criterion_group!(benches, bench_moves_in_srs);
criterion_main!(benches);
