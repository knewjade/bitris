use std::str::FromStr;

use criterion::*;

use bitris::prelude::*;

#[derive(Debug)]
struct MovesBenchmarkData {
    id: String,
    board: Board64,
    spawn: CcPosition,
    expected: Vec<(Shape, usize)>,
}


#[inline(always)]
fn all_moves(data: &MovesBenchmarkData) {
    let board = Board64::from(data.board);
    for (shape, expected_size) in &data.expected {
        let spawn = shape.with(Orientation::North).with(data.spawn);
        let moves = srs::generate_all_moves(AllowMove::Softdrop, board, spawn.into());
        assert_eq!(moves.len(), *expected_size);
    }
}

#[inline(always)]
fn minimized_moves(data: &MovesBenchmarkData) {
    let board = Board64::from(data.board);
    for (shape, expected_size) in &data.expected {
        let spawn = shape.with(Orientation::North).with(data.spawn);
        let moves = srs::generate_minimized_moves(AllowMove::Softdrop, board, spawn.into());
        assert_eq!(moves.len(), *expected_size);
    }
}


fn bench_all_moves(c: &mut Criterion) {
    use Shape::*;
    let mirror = |board: Board64| {
        let mut freeze = board.clone();
        freeze.mirror();
        freeze
    };

    let benchmarks = vec![
        MovesBenchmarkData {
            id: format!("empty"),
            board: Board64::default(),
            spawn: cc(4, 20),
            expected: vec![(T, 34), (I, 34), (L, 34), (J, 34), (S, 34), (Z, 34), (O, 36)],
        },
        MovesBenchmarkData {
            id: format!("pco"),
            board: Board64::from_str(
                "\
                ##.....###\
                ##....####\
                ##...#####\
                ##....####\
            ").unwrap(),
            spawn: cc(4, 20),
            expected: vec![(T, 35), (I, 35), (L, 35), (J, 35), (S, 34), (Z, 36), (O, 36)],
        },
        MovesBenchmarkData {
            id: format!("mirrored-pco"),
            board: Board64::from_str(
                "\
                ##.....###\
                ##....####\
                ##...#####\
                ##....####\
            ").map(mirror).unwrap(),
            spawn: cc(4, 20),
            expected: vec![(T, 35), (I, 35), (L, 35), (J, 35), (S, 36), (Z, 34), (O, 36)],
        },
        MovesBenchmarkData {
            id: format!("loop"),
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
            ").unwrap(),
            spawn: cc(4, 20),
            expected: vec![(T, 35), (I, 108), (L, 35), (J, 36), (S, 34), (Z, 36), (O, 36)],
        },
        MovesBenchmarkData {
            id: format!("mirrored-loop"),
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
            ").map(mirror).unwrap(),
            spawn: cc(4, 20),
            expected: vec![(T, 35), (I, 108), (L, 36), (J, 35), (S, 36), (Z, 34), (O, 36)],
        },
        MovesBenchmarkData {
            id: format!("hatetris"),
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
            ").unwrap(),
            spawn: cc(4, 20),
            expected: vec![(T, 72), (I, 58), (L, 65), (J, 65), (S, 67), (Z, 66), (O, 36)],
        },
        MovesBenchmarkData {
            id: format!("mirrored-hatetris"),
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
            ").map(mirror).unwrap(),
            spawn: cc(4, 20),
            expected: vec![(T, 72), (I, 58), (L, 65), (J, 65), (S, 66), (Z, 67), (O, 36)],
        },
        MovesBenchmarkData {
            id: format!("side-4-wide"),
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
            ").unwrap(),
            spawn: cc(4, 20),
            expected: vec![(T, 34), (I, 34), (L, 34), (J, 34), (S, 34), (Z, 34), (O, 36)],
        },
        MovesBenchmarkData {
            id: format!("mirrored-side-4-wide"),
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
            ").map(mirror).unwrap(),
            spawn: cc(4, 20),
            expected: vec![(T, 34), (I, 34), (L, 34), (J, 34), (S, 34), (Z, 34), (O, 36)],
        },
        MovesBenchmarkData {
            id: format!("center-4-wide"),
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
            ").unwrap(),
            spawn: cc(4, 20),
            expected: vec![(T, 34), (I, 34), (L, 34), (J, 34), (S, 34), (Z, 34), (O, 36)],
        },
        MovesBenchmarkData {
            id: format!("mirrored-center-4-wide"),
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
            ").map(mirror).unwrap(),
            spawn: cc(4, 20),
            expected: vec![(T, 34), (I, 34), (L, 34), (J, 34), (S, 34), (Z, 34), (O, 36)],
        },
        MovesBenchmarkData {
            id: format!("dt"),
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
            ").unwrap(),
            spawn: cc(4, 20),
            expected: vec![(T, 39), (I, 34), (L, 35), (J, 37), (S, 34), (Z, 40), (O, 36)],
        },
        MovesBenchmarkData {
            id: format!("mirrored-dt"),
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
            ").map(mirror).unwrap(),
            spawn: cc(4, 20),
            expected: vec![(T, 39), (I, 34), (L, 37), (J, 35), (S, 40), (Z, 34), (O, 36)],
        },
        MovesBenchmarkData {
            id: format!("rotating-early"),
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
            ").unwrap(),
            spawn: cc(4, 20),
            expected: vec![(T, 73), (I, 70), (L, 72), (J, 69), (S, 70), (Z, 70), (O, 36)],
        },
        MovesBenchmarkData {
            id: format!("mirrored-rotating-early"),
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
            ").map(mirror).unwrap(),
            spawn: cc(4, 20),
            expected: vec![(T, 73), (I, 70), (L, 69), (J, 72), (S, 70), (Z, 70), (O, 36)],
        },
        MovesBenchmarkData {
            id: format!("randomize"),
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
            ").unwrap(),
            spawn: cc(4, 20),
            expected: vec![(T, 134), (I, 138), (L, 137), (J, 151), (S, 132), (Z, 151), (O, 44)],
        },
        MovesBenchmarkData {
            id: format!("mirrored-randomize"),
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
            ").map(mirror).unwrap(),
            spawn: cc(4, 20),
            expected: vec![(T, 134), (I, 139), (L, 151), (J, 137), (S, 151), (Z, 132), (O, 44)],
        },
    ];

    benchmarks.iter().for_each(|benchmark| {
        let id = format!("all-moves-{}", benchmark.id);
        c.bench_function(id.as_str(), |b| {
            b.iter(|| all_moves(benchmark));
        });
    });
}

fn bench_minimized_moves(c: &mut Criterion) {
    use Shape::*;
    let mirror = |board: Board64| {
        let mut freeze = board.clone();
        freeze.mirror();
        freeze
    };

    let benchmarks = vec![
        MovesBenchmarkData {
            id: format!("empty"),
            board: Board64::default(),
            spawn: cc(4, 20),
            expected: vec![(T, 34), (I, 17), (L, 34), (J, 34), (S, 17), (Z, 17), (O, 9)],
        },
        MovesBenchmarkData {
            id: format!("pco"),
            board: Board64::from_str(
                "\
                ##.....###\
                ##....####\
                ##...#####\
                ##....####\
            ").unwrap(),
            spawn: cc(4, 20),
            expected: vec![(T, 35), (I, 18), (L, 35), (J, 35), (S, 17), (Z, 18), (O, 9)],
        },
        MovesBenchmarkData {
            id: format!("mirrored-pco"),
            board: Board64::from_str(
                "\
                ##.....###\
                ##....####\
                ##...#####\
                ##....####\
            ").map(mirror).unwrap(),
            spawn: cc(4, 20),
            expected: vec![(T, 35), (I, 18), (L, 35), (J, 35), (S, 18), (Z, 17), (O, 9)],
        },
        MovesBenchmarkData {
            id: format!("loop"),
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
            ").unwrap(),
            spawn: cc(4, 20),
            expected: vec![(T, 35), (I, 54), (L, 35), (J, 36), (S, 17), (Z, 18), (O, 9)],
        },
        MovesBenchmarkData {
            id: format!("mirrored-loop"),
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
            ").map(mirror).unwrap(),
            spawn: cc(4, 20),
            expected: vec![(T, 35), (I, 54), (L, 36), (J, 35), (S, 18), (Z, 17), (O, 9)],
        },
        MovesBenchmarkData {
            id: format!("hatetris"),
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
            ").unwrap(),
            spawn: cc(4, 20),
            expected: vec![(T, 72), (I, 30), (L, 65), (J, 65), (S, 34), (Z, 33), (O, 9)],
        },
        MovesBenchmarkData {
            id: format!("mirrored-hatetris"),
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
            ").map(mirror).unwrap(),
            spawn: cc(4, 20),
            expected: vec![(T, 72), (I, 30), (L, 65), (J, 65), (S, 33), (Z, 34), (O, 9)],
        },
        MovesBenchmarkData {
            id: format!("side-4-wide"),
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
            ").unwrap(),
            spawn: cc(4, 20),
            expected: vec![(T, 34), (I, 17), (L, 34), (J, 34), (S, 17), (Z, 17), (O, 9)],
        },
        MovesBenchmarkData {
            id: format!("mirrored-side-4-wide"),
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
            ").map(mirror).unwrap(),
            spawn: cc(4, 20),
            expected: vec![(T, 34), (I, 17), (L, 34), (J, 34), (S, 17), (Z, 17), (O, 9)],
        },
        MovesBenchmarkData {
            id: format!("center-4-wide"),
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
            ").unwrap(),
            spawn: cc(4, 20),
            expected: vec![(T, 34), (I, 17), (L, 34), (J, 34), (S, 17), (Z, 17), (O, 9)],
        },
        MovesBenchmarkData {
            id: format!("mirrored-center-4-wide"),
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
            ").map(mirror).unwrap(),
            spawn: cc(4, 20),
            expected: vec![(T, 34), (I, 17), (L, 34), (J, 34), (S, 17), (Z, 17), (O, 9)],
        },
        MovesBenchmarkData {
            id: format!("dt"),
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
            ").unwrap(),
            spawn: cc(4, 20),
            expected: vec![(T, 39), (I, 17), (L, 35), (J, 37), (S, 17), (Z, 20), (O, 9)],
        },
        MovesBenchmarkData {
            id: format!("mirrored-dt"),
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
            ").map(mirror).unwrap(),
            spawn: cc(4, 20),
            expected: vec![(T, 39), (I, 17), (L, 37), (J, 35), (S, 20), (Z, 17), (O, 9)],
        },
        MovesBenchmarkData {
            id: format!("rotating-early"),
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
            ").unwrap(),
            spawn: cc(4, 20),
            expected: vec![(T, 73), (I, 35), (L, 72), (J, 69), (S, 35), (Z, 35), (O, 9)],
        },
        MovesBenchmarkData {
            id: format!("mirrored-rotating-early"),
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
            ").map(mirror).unwrap(),
            spawn: cc(4, 20),
            expected: vec![(T, 73), (I, 35), (L, 69), (J, 72), (S, 35), (Z, 35), (O, 9)],
        },
        MovesBenchmarkData {
            id: format!("randomize"),
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
            ").unwrap(),
            spawn: cc(4, 20),
            expected: vec![(T, 134), (I, 73), (L, 137), (J, 151), (S, 68), (Z, 78), (O, 11)],
        },
        MovesBenchmarkData {
            id: format!("mirrored-randomize"),
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
            ").map(mirror).unwrap(),
            spawn: cc(4, 20),
            expected: vec![(T, 134), (I, 74), (L, 151), (J, 137), (S, 78), (Z, 68), (O, 11)],
        },
    ];

    benchmarks.iter().for_each(|benchmark| {
        let id = format!("minimized-moves-{}", benchmark.id);
        c.bench_function(id.as_str(), |b| {
            b.iter(|| minimized_moves(benchmark));
        });
    });
}


criterion_group!(benches, bench_all_moves, bench_minimized_moves);
criterion_main!(benches);
