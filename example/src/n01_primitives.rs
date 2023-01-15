#[cfg(test)]
mod tests {
    use bitris::macros::piece;
    use bitris::prelude::*;
    
    #[test]
    fn primitives() {
        // Make pieces
        let piece1 = Piece { shape: Shape::T, orientation: Orientation::North };
        let piece2 = Piece::new(Shape::T, Orientation::North);
        let piece3 = Shape::T.with(Orientation::North);
        let piece4 = piece!(TN);
        assert!(piece1 == piece2 && piece2 == piece3 && piece3 == piece4);

        // Mate cc placements
        // ..........
        // ..#.......
        // .#@#......  << @ is cc(2,2)
        // ..........
        // ..........
        let cc1 = CcPlacement {
            piece: Piece::new(Shape::T, Orientation::North),
            position: CcPosition { cx: 2, cy: 2 },
        };
        let cc2 = CcPlacement { piece: piece!(TN), position: cc(2, 2) };
        assert_eq!(cc1, cc2);

        // The placements can be interconverted.
        // ..........
        // ..#.......
        // .@##......  << @ is bl(1,2)
        // ..........
        // ..........
        let bl1 = BlPlacement {
            piece: Piece::new(Shape::T, Orientation::North),
            position: BlPosition { lx: 1, by: 2 },
        };
        let bl2 = BlPlacement { piece: piece!(TN), position: bl(1, 2) };
        assert_eq!(bl1, bl2);
        assert_eq!(bl1.cc_placement(), cc1);
        assert_eq!(cc1.bl_placement(), bl1);

        // ..........
        // ..#@......
        // .###......  << @ is tr(3,3)
        // ..........
        // ..........
        let tr1 = TrPlacement {
            piece: Piece::new(Shape::T, Orientation::North),
            position: TrPosition { rx: 3, ty: 3 },
        };
        let tr2 = TrPlacement { piece: piece!(TN), position: tr(3, 3) };
        assert_eq!(tr1, tr2);
        assert_eq!(tr1.cc_placement(), cc1);
        assert_eq!(cc1.tr_placement(), tr1);

        // The placements can be rotated.
        // ..........
        // ..#.......
        // ..@#......  << @ is cc
        // ..#.......
        // ..........
        let north = piece!(TN).with(cc(2, 2));
        assert_eq!(north.rotate(Rotation::Cw), piece!(TE).with(cc(2, 2)));

        // Note that the position changes except for cc.
        // ..........     ..........
        // ..#.......     ..#.......
        // .@##......  => ..##......  << @ is bl
        // ..........     ..@.......
        // ..........     ..........
        let north = piece!(TN).with(bl(1, 2));
        assert_eq!(north.rotate(Rotation::Cw), piece!(TE).with(bl(2, 1)));

        // ..........     ..........
        // ..........     ..#@......
        // .##@......  => ..##......  << @ is tr
        // ..#.......     ..#.......
        // ..........     ..........
        let south = piece!(TS).with(tr(3, 2));
        assert_eq!(south.rotate(Rotation::Ccw), piece!(TE).with(tr(3, 3)));
    }
}
