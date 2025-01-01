use crate::coordinates::cc;
use crate::pieces::{Orientation, Piece, ToBlPosition};
use crate::placements::BlPlacement;
use crate::With;

#[derive(Debug)]
pub struct Moves4 {
    pub spawn_piece: Piece,
    pub reachables: [[u32; 10]; 4],
}

impl Moves4 {
    #[inline]
    pub fn vec(&self) -> Vec<BlPlacement> {
        let mut out = Vec::<BlPlacement>::with_capacity(128);

        for piece in self.spawn_piece.shape.all_pieces_iter() {
            let cols = self.reachables[piece.orientation as usize];
            for (cx, mut col) in cols.into_iter().enumerate() {
                while 0 < col {
                    let cy = col.trailing_zeros();
                    out.push(piece.with(cc(cx as i32, cy as i32)).to_bl_placement());
                    col -= (1u32) << cy;
                }
            }
        }

        out
    }
}

#[derive(Debug)]
pub struct Moves1 {
    pub spawn_piece: Piece,
    pub reachable: [u32; 10],
    pub minimized: bool,
}

impl Moves1 {
    #[inline]
    pub fn vec(&self) -> Vec<BlPlacement> {
        debug_assert!(self.spawn_piece.canonical().is_none());

        let mut out = Vec::<BlPlacement>::with_capacity(128);

        let shape = self.spawn_piece.shape;
        let piece_blocks = self.spawn_piece.to_piece_blocks();
        let cols = self.reachable;
        for (cx, mut col) in cols.into_iter().enumerate() {
            while 0 < col {
                let cy = col.trailing_zeros();
                let bl_position = cc(cx as i32, cy as i32).to_bl_position(piece_blocks);
                if self.minimized {
                    out.push(self.spawn_piece.with(bl_position));
                } else {
                    for orientation in Orientation::all_iter() {
                        out.push(shape.with(orientation).with(bl_position));
                    }
                }
                col -= 1u32 << cy;
            }
        }

        out
    }
}
