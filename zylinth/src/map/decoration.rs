use lazy_static::lazy_static;
use rand::Rng;
use tilegen::*;

use super::{TuesdayTile, TuesdayTile::*};

pub fn decorate_empty(input: &mut TileGrid<TuesdayTile>, rng: &mut impl Rng) {
    input.insert_layer();
    input.apply_layer_replacements(0, EMPTIES.to_vec(), rng);
}

lazy_static! {
    static ref EMPTIES: Vec<ReplacementRule<TuesdayTile>> = vec![
        ReplacementRule {
            condition: |src, _| { is_wall_ish(src.above()) },
            replacements: vec![Replacement::this(TuesdayTile::EmptyDecoration1),],
            chance: 0.1,
            apply_count: Some(2),
            ..Default::default()
        },
        ReplacementRule {
            condition: |src, _| { is_wall_ish(src.above()) },
            replacements: vec![Replacement::this(TuesdayTile::EmptyDecoration2),],
            chance: 0.1,
            apply_count: Some(2),
            ..Default::default()
        },
    ];
}

fn is_wall_ish(ctx: TileContext<TuesdayTile>) -> bool {
    ctx == WallLeft
        || ctx == WallRight
        || ctx == WallAllCorner
        || ctx == WallDoubleVertical
        || ctx == WallDoubleHorizontal
        || ctx == WallBottom
        || ctx == WallTop
        || ctx == WallDoubleUpper
        || ctx == WallDoubleLower
        || ctx == WallInnerCornerBottomLeft
        || ctx == WallInnerCornerBottomRight
        || ctx == WallInnerCornerTopLeft
        || ctx == WallInnerCornerTopRight
}
