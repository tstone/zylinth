use crate::layout::functional_tiles::UtilityTile;
use crate::layout::functional_tiles::UtilityTile::*;
use lazy_static::lazy_static;
use rand_chacha::ChaCha8Rng;
use tilegen::*;

pub fn decorate_layer(input: &mut TileGrid<UtilityTile>, layer: usize, rng: &mut ChaCha8Rng) {
    input.apply_layer_replacements(layer, VERT_DECORATIONS.to_vec(), rng);
}

lazy_static! {
    static ref VERT_DECORATIONS: Vec<ReplacementRule<UtilityTile>> = vec![
        // locker
        ReplacementRule {
            condition: |src, _| {
                let base = src.layer(0);
                *src == UtilityTile::WallTopLower && base.down() == Floor && (
                    base.left() == WallTopLower || base.right() == WallTopLower
                )
            },
            replacements: vec![
                Replacement::this(UtilityTile::VertDecorationTop(1)),
                Replacement::down(UtilityTile::VertDecorationBottom(1)),
            ],
            chance: 0.125,
            ..Default::default()
        },
        // locker open
        ReplacementRule {
            condition: |src, _| {
                let base = src.layer(0);
                *src == UtilityTile::WallTopLower && base.down() == Floor && (
                    base.left() == WallTopLower || base.right() == WallTopLower
                )
            },
            replacements: vec![
                Replacement::this(UtilityTile::VertDecorationTop(2)),
                Replacement::down(UtilityTile::VertDecorationBottom(2)),
            ],
            chance: 0.08,
            ..Default::default()
        },
        // alien
        ReplacementRule {
            condition: |src, _| {
                let base = src.layer(0);
                *src == UtilityTile::WallTopLower && base.down() == Floor && (
                    base.left() == WallTopLower || base.right() == WallTopLower
                )
            },
            replacements: vec![
                Replacement::this(UtilityTile::VertDecorationTop(3)),
                Replacement::down(UtilityTile::VertDecorationBottom(3)),
            ],
            chance: 0.19,
            ..Default::default()
        },
        // bookshelf
        ReplacementRule {
            condition: |src, _| {
                let base = src.layer(0);
                *src == UtilityTile::WallTopLower && base.down() == Floor && (
                    base.left() == WallTopLower || base.right() == WallTopLower
                )
            },
            replacements: vec![
                Replacement::this(UtilityTile::VertDecorationTop(4)),
                Replacement::down(UtilityTile::VertDecorationBottom(4)),
            ],
            chance: 0.2,
            ..Default::default()
        },
    ];
}
