use bitris::prelude::*;
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
        self.println(format!("/** {} */", comment).as_str());
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

    if 0 < offset.dx {
        line.push_str(format!("{}, {}", offset.dx, 0).as_str());
    } else if offset.dx < 0 {
        line.push_str(format!("{}, {}", 0, -offset.dx).as_str());
    } else {
        line.push_str("0, 0");
    }

    line.push_str(", ");

    if 0 < offset.dy {
        line.push_str(format!("{}, {}", offset.dy, 0).as_str());
    } else if offset.dy < 0 {
        line.push_str(format!("{}, {}", 0, -offset.dy).as_str());
    } else {
        line.push_str("0, 0");
    }

    line
}

fn generate_free(path: &str) {
    let content = begin(|b| {
        b._comment_block("It's auto generated.");
        b._use("crate::internal_moves::u64::free_space::FreeSpace64");
        b._use("crate::pieces::{Shape, Orientation, Piece}");

        b.newline();
        b.println("#[inline(always)]");
        b._pub_fn(
            "to_free_spaces",
            "free_space_block: FreeSpace64, shape: Shape",
            "[FreeSpace64; 4]",
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
            "free_space_block: FreeSpace64, piece: Piece",
            "FreeSpace64",
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
                b._fn(
                    function_name.to_lowercase().as_str(),
                    "space: FreeSpace64",
                    "FreeSpace64",
                    |b| {
                        fn format_space(offset: Offset, clone: bool) -> String {
                            let mut line = String::with_capacity(256);
                            line.push_str("space");
                            if clone {
                                line.push_str(".clone()");
                            }

                            if offset != Offset::new(0, 0) {
                                line.push_str(".shift::<");
                                line.push_str(format_offset(offset).as_str());
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
                        b.println(spaces[0].to_string().as_str());
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

fn main() {
    generate_free("src/internal_moves/u64/free.rs");
}
