use bitris::prelude::*;
use std::cmp::Ordering::{Equal, Greater, Less};
use std::fs::File;
use std::io::Write;

struct Buffer {
    indent: usize,
    content: String,
}

impl Buffer {
    pub(crate) fn println(&mut self, line: &str) {
        for _ in 0..self.indent {
            self.content.push(' ');
        }
        self.content.push_str(line);
        self.newline();
    }

    pub(crate) fn newline(&mut self) {
        self.content.push('\n');
    }

    pub fn deep<F>(&mut self, func: F)
    where
        F: Fn(&mut Buffer),
    {
        self.indent += 4;
        func(self);
        self.indent -= 4;
    }

    pub fn _comment_block(&mut self, comment: &str) {
        self.println(format!("/*! {} */", comment).as_str());
    }

    pub fn _use(&mut self, package: &str) {
        self.println(format!("use {};", package).as_str());
    }

    pub fn _pub_mod<F>(&mut self, name: &str, func: F)
    where
        F: Fn(&mut Buffer),
    {
        self.println(format!("pub mod {} {{", name).as_str());
        self.indent += 4;
        func(self);
        self.indent -= 4;
        self.println("}");
    }

    pub fn _pub_fn<F>(&mut self, name: &str, args: &str, res: &str, func: F)
    where
        F: Fn(&mut Buffer),
    {
        self.println(format!("pub fn {}({}) -> {} {{", name, args, res).as_str());
        self.indent += 4;
        func(self);
        self.indent -= 4;
        self.println("}");
    }

    pub fn _fn<F>(&mut self, name: &str, args: &str, res: &str, func: F)
    where
        F: Fn(&mut Buffer),
    {
        self.println(format!("fn {}({}) -> {} {{", name, args, res).as_str());
        self.indent += 4;
        func(self);
        self.indent -= 4;
        self.println("}");
    }

    pub fn _match<F>(&mut self, arg: &str, func: F)
    where
        F: Fn(&mut Buffer),
    {
        self.println(format!("match {} {{", arg).as_str());
        self.indent += 4;
        func(self);
        self.indent -= 4;
        self.println("}");
    }
}

fn begin(func: fn(b: &mut Buffer)) -> String {
    let mut buffer = Buffer {
        indent: 0,
        content: String::with_capacity(1024 * 10),
    };
    func(&mut buffer);
    buffer.content
}

fn format_offset(offset: Offset) -> String {
    let mut line = String::with_capacity(16);

    match offset.dx.cmp(&0) {
        Greater => line.push_str(format!("{}, {}", 0, offset.dx).as_str()),
        Less => line.push_str(format!("{}, {}", -offset.dx, 0).as_str()),
        Equal => line.push_str("0, 0"),
    }

    line.push_str(", ");

    match offset.dy.cmp(&0) {
        Greater => line.push_str(format!("{}, {}", 0, offset.dy).as_str()),
        Less => line.push_str(format!("{}, {}", -offset.dy, 0).as_str()),
        Equal => line.push_str("0, 0"),
    }

    line
}

fn generate_free(path: &str) {
    let content = begin(|b| {
        b._comment_block("It's auto generated.");
        b._use("crate::internal_moves::avx2::h16::free_space::FreeSpaceSimd16");
        b._use("crate::pieces::{Orientation, Piece, Shape}");

        b.newline();
        b.println("#[inline(always)]");
        b._pub_fn(
            "to_free_spaces",
            "free_space_block: FreeSpaceSimd16, shape: Shape",
            "[FreeSpaceSimd16; 4]",
            |b| {
                b._match("shape", |b| {
                    for shape in Shape::all_iter() {
                        b.println(format!("Shape::{} => [", shape).as_str());
                        b.deep(|b| {
                            let shape = shape.to_string().to_lowercase();
                            b.println(
                                format!("{}_north(free_space_block.clone()),", shape).as_str(),
                            );
                            b.println(
                                format!("{}_east(free_space_block.clone()),", shape).as_str(),
                            );
                            b.println(
                                format!("{}_south(free_space_block.clone()),", shape).as_str(),
                            );
                            b.println(format!("{}_west(free_space_block),", shape).as_str());
                        });
                        b.println("],");
                    }
                });
            },
        );

        b.newline();
        b.println("#[inline(always)]");
        b._pub_fn(
            "to_free_space",
            "free_space_block: FreeSpaceSimd16, piece: Piece",
            "FreeSpaceSimd16",
            |b| {
                b._match("piece.shape", |b| {
                    for shape in Shape::all_iter() {
                        b.println(format!("Shape::{} => {{", shape).as_str());
                        b.deep(|b| {
                            b._match("piece.orientation", |b| {
                                for orientation in Orientation::all_iter() {
                                    b.println(
                                        format!(
                                            "Orientation::{} => {}_{}(free_space_block.clone()),",
                                            orientation,
                                            shape.to_string().to_lowercase(),
                                            orientation.to_string().to_lowercase(),
                                        )
                                        .as_str(),
                                    );
                                }
                            });
                        });
                        b.println("},");
                    }
                });
            },
        );

        for shape in Shape::all_iter() {
            for orientation in Orientation::all_iter() {
                let piece = Piece::new(shape, orientation);
                let piece_blocks = piece.to_piece_blocks();
                let function_name = format!("{:?}_{:?}", shape, orientation);

                b.newline();
                b.println("#[inline(always)]");
                b._pub_fn(
                    function_name.to_lowercase().as_str(),
                    "space: FreeSpaceSimd16",
                    "FreeSpaceSimd16",
                    |b| {
                        fn format_space(offset: Offset, clone: bool) -> String {
                            let mut line = String::with_capacity(256);
                            line.push_str("space");
                            if clone {
                                line.push_str(".clone()");
                            }

                            if offset != Offset::new(0, 0) {
                                line.push_str(".shift::<");
                                line.push_str(format_offset(-offset).as_str());
                                line.push_str(">()");
                            }
                            line
                        }

                        let spaces = [
                            format_space(piece_blocks.offsets[0], true),
                            format_space(piece_blocks.offsets[1], true),
                            format_space(piece_blocks.offsets[2], true),
                            format_space(piece_blocks.offsets[3], false),
                        ];
                        b.println(format!("{}", spaces[0]).as_str());
                        b.deep(|b| {
                            b.println(format!(".and({})", spaces[1]).as_str());
                            b.println(format!(".and({})", spaces[2]).as_str());
                            b.println(format!(".and({})", spaces[3]).as_str());
                        });
                    },
                );
            }
        }
    });

    let mut file = File::create(path).expect("Unable to create file");
    file.write_all(content.as_bytes())
        .expect("Unable to write data");
}

fn generate_rotate(path: &str) {
    let content = begin(|b| {
        b._comment_block("It's auto generated.");

        b._use("crate::internal_moves::avx2::h16::free_space::FreeSpaceSimd16");
        b._use("crate::internal_moves::avx2::h16::reachable::ReachableSimd16");
        b._use("crate::pieces::{Orientation, Piece, Shape}");

        for rotation in [Rotation::Cw, Rotation::Ccw] {
            let rotation = rotation.to_string().to_lowercase();
            b.newline();
            b.println("#[inline(always)]");
            b._pub_fn(
                format!("rotate_{}", rotation).as_str(),
                "from_piece: Piece, src_reachable: &ReachableSimd16, dest_free_space: &FreeSpaceSimd16",
                "ReachableSimd16",
                |b| {
                    b.println("debug_assert!(from_piece.shape != Shape::O);");
                    b.newline();
                    b._match("from_piece.shape", |b| {
                        for shape in Shape::all_iter() {
                            if shape == Shape::O {
                                b.println(format!("Shape::{} => ReachableSimd16::blank(),", shape).as_str());
                                continue;
                            }

                            b.println(format!("Shape::{} => {{", shape).as_str());
                            b.deep(|b| {
                                b._match("from_piece.orientation", |b| {
                                    for orientation in Orientation::all_iter() {
                                        b.println(
                                            format!(
                                                "Orientation::{} => {}::from_{}_{}(src_reachable, dest_free_space),",
                                                orientation,
                                                rotation,
                                                shape.to_string().to_lowercase(),
                                                orientation.to_string().to_lowercase(),
                                            )
                                                .as_str(),
                                        );
                                    }
                                });
                            });
                            b.println("}");
                        }
                    });
                },
            );
        }

        for rotation in [Rotation::Cw, Rotation::Ccw] {
            b.newline();
            b._pub_mod(format!("{}", rotation).to_lowercase().as_str(), |b| {
                b._use("crate::internal_moves::avx2::h16::free_space::FreeSpaceSimd16");
                b._use("crate::internal_moves::avx2::h16::reachable::ReachableSimd16");

                for shape in Shape::all_iter() {
                    if shape == Shape::O {
                        continue;
                    }

                    for orientation in Orientation::all_iter() {
                        let piece = Piece::new(shape, orientation);
                        let function_name = format!("from_{:?}_{:?}", shape, orientation);

                        b.newline();
                        b.println("#[inline(always)]");
                        b._pub_fn(
                            function_name.to_lowercase().as_str(),
                            "src_reachable: &ReachableSimd16, dest_free_space: &FreeSpaceSimd16",
                            "ReachableSimd16",
                            |b| {
                                fn format_kick(kick: Kick, last: bool) -> Vec<String> {
                                    let offset = kick.offset;
                                    let mut lines = Vec::new();

                                    lines.push(format!("// {}", kick));

                                    let forward_offset = format_offset(offset);
                                    let backward_offset = format_offset(-offset);

                                    lines.push(format!(
                                        "let shift_forward = src_candidates.clone().jump_and::<{}>(dest_free_space);",
                                        forward_offset
                                    ));
                                    lines.push("let dest_reachable = dest_reachable.or(&shift_forward);".to_string());

                                    if !last {
                                        lines.push(format!(
                                            "let src_candidates = src_candidates.jump_rev::<{}>(shift_forward);",
                                            backward_offset
                                        ));
                                        lines.push("if src_candidates.empty() {".to_string());
                                        lines.push("    return dest_reachable;".to_string());
                                        lines.push("}".to_string());
                                        lines.push(String::new());
                                    }

                                    lines
                                }

                                b.println("debug_assert!(!src_reachable.empty());");
                                b.newline();
                                b.println("let src_candidates = src_reachable.clone();");
                                b.println("let dest_reachable = ReachableSimd16::blank();");
                                b.newline();

                                let kicks = SrsKickTable.iter_kicks(piece, rotation).enumerate().collect::<Vec<_>>();
                                for (index, &kick) in &kicks {
                                    let last = *index == kicks.len() - 1;
                                    for line in format_kick(kick, last) {
                                        b.println(line.as_str());
                                    }
                                }

                                b.println("dest_reachable");
                            },
                        );
                    }
                }
            });
        }
    });

    let mut file = File::create(path).expect("Unable to create file");
    file.write_all(content.as_bytes())
        .expect("Unable to write data");
}

fn generate_minimize(path: &str) {
    let content = begin(|b| {
        b._comment_block("It's auto generated.");
        b._use("crate::internal_moves::avx2::h16::reachable::ReachableSimd16");
        b._use("crate::pieces::Shape");

        b.newline();
        b.println("#[inline(always)]");
        b._pub_fn(
            "minimize",
            "reachables: [ReachableSimd16; 4], shape: Shape",
            "[ReachableSimd16; 4]",
            |b| {
                b._match("shape", |b| {
                    for shape in Shape::all_iter() {
                        if shape == Shape::T || shape == Shape::L || shape == Shape::J {
                            b.println(format!("Shape::{} => reachables,", shape).as_str());
                            continue
                        }

                        b.println(format!("Shape::{} => [", shape).as_str());
                        b.deep(|b| {
                            for dest_orientation in Orientation::all_iter() {
                                let piece = Piece::new(shape, dest_orientation);
                                if piece.canonical().is_some() {
                                    b.println("ReachableSimd16::blank(),");
                                    continue;
                                }

                                // aggregate
                                b.println(format!("reachables[{}].clone()", dest_orientation as usize).as_str());
                                piece.orientations_having_same_form().iter()
                                    .filter(|&src_orientation| *src_orientation != dest_orientation)
                                    .for_each(|src_orientation| {
                                        let src_bl = shape.with(*src_orientation).to_piece_blocks().bottom_left;
                                        let dest_bl = shape.with(dest_orientation).to_piece_blocks().bottom_left;
                                        let offset = format_offset(src_bl - dest_bl);
                                        b.println(format!(".or_shift::<{}>(&reachables[{}])", offset, *src_orientation as usize).as_str());
                                    });
                                b.println(",");
                            }
                        });
                        b.println("],");
                    }
                });
            },
        );
    });

    let mut file = File::create(path).expect("Unable to create file");
    file.write_all(content.as_bytes())
        .expect("Unable to write data");
}

fn main() {
    generate_free("src/internal_moves/avx2/h16/free.rs");
    generate_rotate("src/internal_moves/avx2/h16/rotate.rs");
    generate_minimize("src/internal_moves/avx2/h16/minimize.rs");
}
