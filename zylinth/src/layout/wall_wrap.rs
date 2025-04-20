use super::{functional_tiles::UtilityTile, functional_tiles::UtilityTile::*};
use lazy_static::lazy_static;
use rand_chacha::ChaCha8Rng;
use tilegen::*;

pub fn wrap_walls(input: TileGrid<UtilityTile>, rng: &mut ChaCha8Rng) -> TileGrid<UtilityTile> {
    let mut padded = TileGrid::pad(&input, 3, 1, 1, 1);
    padded.apply_layer_replacements(0, FIRST_PASS.to_vec(), rng);
    padded.apply_layer_replacements(0, SECOND_PASS.to_vec(), rng);
    padded.apply_layer_replacements(0, THIRD_PASS.to_vec(), rng);
    padded
}

lazy_static! {
    // first pass establishes walls all the way around floors
    static ref FIRST_PASS: Vec<ReplacementRule<UtilityTile>> = vec![
        // top-left outer corner (small)
        ReplacementRule::to(WallTopLeft, |src, _| {
            *src == None && src.up() == None && src.down() == None && src.right() == None && src.bottom_right() == Floor
        }),
        // top-right outer corner (small)
        ReplacementRule::to(WallTopRight, |src, _| {
            *src == None && src.up() == None && src.down() == None && src.left() == None && src.bottom_left() == Floor
        }),
        // bottom-left outer corner (small)
        ReplacementRule::to(WallBottomLeft, |src, _| {
            *src == None && src.up() == None && src.down() == None && src.right() == None && src.top_right() == Floor
        }),
        // bottom-right outer corner (small)
        ReplacementRule::to(WallBottomRight, |src, _| {
            *src == None && src.up() == None && src.down() == None && src.left() == None && src.top_left() == Floor
        }),

        // └ - inner corner top right
        ReplacementRule::to(WallTopLower, |src, _| {
            *src == None && src.up() == None && src.left() == Floor && src.down() == Floor
        }),
        // ┘ - inner corner top left
        ReplacementRule::to(WallTopLower, |src, _| {
            *src == None && src.up() == None && src.right() == Floor && src.down() == Floor
        }),
        // ┐ - inner corner bottom left
        ReplacementRule::to(WallInnerCornerTopRight, |src, _| {
            *src == None && src.up() == Floor && src.right() == Floor && src.top_right() == Floor
        }),
        // ┌ - inner corner bottom right
        ReplacementRule::to(WallInnerCornerTopLeft, |src, _| {
            *src == None && src.up() == Floor && src.left() == Floor && src.top_left() == Floor
        }),

        // top wall
        ReplacementRule::to(WallTopLower, |src, _| {
            *src == None && src.up() == None && src.down() == Floor
        }),
        // left
        ReplacementRule::to(WallLeft, |src, _| {
            *src == None && src.left() == None && src.right() == Floor
        }),
        // right
        ReplacementRule::to(WallRight, |src, _| {
            *src == None && src.right() == None && src.left() == Floor
        }),
        // bottom
        ReplacementRule::to(WallBottom, |src, _| {
            *src == None && src.down() == None && src.up() == Floor
        }),
    ];

    // second pass makes the top wall aler double height
    static ref SECOND_PASS: Vec<ReplacementRule<UtilityTile>> = vec![
        // move top-left outer corner up
        ReplacementRule::to(WallTopLeft, |src, _| {
            *src == None && src.down() == Some(WallTopLeft)
        }),
        // move top-right outer corner up
        ReplacementRule::to(WallTopRight, |src, _| {
            *src == None && src.down() == Some(WallTopRight)
        }),
        // ┘ - inner corner top left
        ReplacementRule::to(WallTopUpper, |src, _| {
            *src == WallTopLeft && src.down() == WallTopLower && src.right() == WallTopLower
        }),
        // └ - inner corner top right
        ReplacementRule::to(WallTopUpper, |src, _| {
            *src == WallTopRight && src.down() == WallTopLower && src.left() == WallTopLower
        }),
        // swap top-left & top-right to wall left since it was moved up
        ReplacementRule::to(WallLeft, |src, _| { *src == WallTopLeft }),
        ReplacementRule::to(WallRight, |src, _| { *src == WallTopRight }),

        // double top wall
        ReplacementRule::to(WallTopUpper, |src, _| {
            *src == None && src.down() == WallTopLower
        }),
        ReplacementRule::to(WallTopUpper, |src, _| {
            *src ==WallInnerCornerTopLeft && src.down() == WallTopLower
        }),
        ReplacementRule::to(WallTopUpper, |src, _| {
            *src == WallInnerCornerTopRight && src.down() == WallTopLower
        }),
        ReplacementRule::to(WallTopUpper, |src, _| {
            *src == WallLeft && src.down() == WallTopLower
        }),
        ReplacementRule::to(WallTopUpper, |src, _| {
            *src == WallRight && src.down() == WallTopLower
        }),
        // when rooms are close together make double walls come up to room above
        ReplacementRule::to(WallTopUpper, |src, _| {
            *src == WallBottom && src.down() == WallTopLower
        }),
        ReplacementRule::to(WallTopUpper, |src, _| {
            *src == WallBottomLeft && src.down() == WallTopLower
        }),
        ReplacementRule::to(WallTopUpper, |src, _| {
            *src == WallBottomRight && src.down() == WallTopLower
        }),
        ReplacementRule::to(WallTopUpper, |src, _| {
            *src == WallInnerCornerTopLeft && src.down() == WallTopLower
        }),
        ReplacementRule::to(WallTopUpper, |src, _| {
            *src == WallInnerCornerTopRight && src.down() == WallTopLower
        }),
    ];

    // third pass wraps the wall in the top most layer
    static ref THIRD_PASS: Vec<ReplacementRule<UtilityTile>> = vec![
        // move top-left outer corner up
        ReplacementRule::to(WallTopLeft, |src, _| {
            *src == None && src.down() == Some(WallTopLeft)
        }),
        // move top-right outer corner up
        ReplacementRule::to(WallTopRight, |src, _| {
            *src == None && src.down() == Some(WallTopRight)
        }),
        // ┘ - inner corner top left
        ReplacementRule::to(WallInnerCornerBottomRight, |src, _| {
            *src == WallTopLeft &&  src.down() == WallTopUpper && src.right() == WallTopUpper
        }),
        ReplacementRule::to(WallInnerCornerBottomRight, |src, _| {
            *src == WallBottomLeft && src.down() == WallTopUpper && src.right() == WallTopUpper
        }),
        // └ - inner corner top right
        ReplacementRule::to(WallInnerCornerBottomLeft, |src, _| {
            *src == WallTopRight && src.down() == WallTopUpper && src.left() == WallTopUpper
        }),
        ReplacementRule::to(WallInnerCornerBottomLeft, |src, _| {
            *src == WallBottomRight && src.down() == WallTopUpper && src.left() == WallTopUpper
        }),
        // swap top-left & top-right to wall left since it was moved up
        ReplacementRule::to(WallLeft, |src, _| { *src == WallTopLeft && src.up() == None }),
        ReplacementRule::to(WallRight, |src, _| { *src == WallTopRight && src.up() == None }),

        // after the second round of wall tops, some outlines will need to be turned
        // to inner corners
        // ┘ - inner corner top left
        ReplacementRule::to(WallInnerCornerBottomRight, |src, _| {
            *src == WallLeft && src.down() == WallTopUpper
                && (src.bottom_left() == Some(WallTopLeft) || src.bottom_left() == WallTopUpper)
        }),
        // └ - inner corner top right
        ReplacementRule::to(WallInnerCornerBottomLeft, |src, _| {
            *src == WallRight && src.down() == WallTopUpper
                && (src.bottom_right() == Some(WallTopRight) || src.bottom_right() == WallTopUpper)
        }),
        // ┐ - inner corner bottom left
        ReplacementRule::to(WallInnerCornerTopRight, |src, _| {
            *src == WallBottom && src.up() == Floor
                && (src.right() == WallTopUpper || src.right() == WallTopLower) && src.top_right() == Floor
        }),
        // ┌ - inner corner bottom right
        ReplacementRule::to(WallInnerCornerTopLeft, |src, _| {
            *src == WallBottom && src.up() == Floor
                && (src.right() == WallTopUpper || src.right() == WallTopLower) && src.top_left() == Floor
        }),

        // top most
        ReplacementRule::to(WallTopmost, |src, _| {
            *src == None && src.down() == WallTopUpper
        }),
    ];
}
