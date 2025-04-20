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
        ReplacementRule::none_to(WallTopLeft, |src, _dest| {
            src.up() == None && src.down() == None && src.right() == None && src.bottom_right() == Floor
        }),
        // top-right outer corner (small)
        ReplacementRule::none_to(WallTopRight, |src, _dest| {
            src.up() == None && src.down() == None && src.left() == None && src.bottom_left() == Floor
        }),
        // bottom-left outer corner (small)
        ReplacementRule::none_to(WallBottomLeft, |src, _dest| {
            src.up() == None && src.down() == None && src.right() == None && src.top_right() == Floor
        }),
        // bottom-right outer corner (small)
        ReplacementRule::none_to(WallBottomRight, |src, _dest| {
            src.up() == None && src.down() == None && src.left() == None && src.top_left() == Floor
        }),

        // └ - inner corner top right
        ReplacementRule::none_to(WallTopLower, |src, _dest| {
            src.up() == None && src.left() == Floor && src.down() == Floor
        }),
        // ┘ - inner corner top left
        ReplacementRule::none_to(WallTopLower, |src, _dest| {
            src.up() == None && src.right() == Floor && src.down() == Floor
        }),
        // ┐ - inner corner bottom left
        ReplacementRule::none_to(WallInnerCornerTopRight, |src, _dest| {
            src.up() == Floor && src.right() == Floor && src.top_right() == Floor
        }),
        // ┌ - inner corner bottom right
        ReplacementRule::none_to(WallInnerCornerTopLeft, |src, _dest| {
            src.up() == Floor && src.left() == Floor && src.top_left() == Floor
        }),

        // top wall
        ReplacementRule::none_to(WallTopLower, |src, _dest| {
            src.up() == None && src.down() == Floor
        }),
        // left
        ReplacementRule::none_to(WallLeft, |src, _dest| {
            src.left() == None && src.right() == Floor
        }),
        // right
        ReplacementRule::none_to(WallRight, |src, _dest| {
            src.right() == None && src.left() == Floor
        }),
        // bottom
        ReplacementRule::none_to(WallBottom, |src, _dest| {
            src.down() == None && src.up() == Floor
        }),
    ];

    // second pass makes the top wall aler double height
    static ref SECOND_PASS: Vec<ReplacementRule<UtilityTile>> = vec![
        // move top-left outer corner up
        ReplacementRule::none_to(WallTopLeft, |src, _dest| {
            src.down() == Some(WallTopLeft)
        }),
        // move top-right outer corner up
        ReplacementRule::none_to(WallTopRight, |src, _dest| {
            src.down() == Some(WallTopRight)
        }),
        // ┘ - inner corner top left
        ReplacementRule::to(WallTopUpper, |src, _dest| {
            *src == WallTopLeft && src.down() == WallTopLower && src.right() == WallTopLower
        }),
        // └ - inner corner top right
        ReplacementRule::to(WallTopUpper, |src, _dest| {
            *src == WallTopRight && src.down() == WallTopLower && src.left() == WallTopLower
        }),
        // swap top-left & top-right to wall left since it was moved up
        ReplacementRule::to(WallLeft, |src, _| { *src == WallTopLeft }),
        ReplacementRule::to(WallRight, |src, _| { *src == WallTopRight }),

        // double top wall
        ReplacementRule::none_to(WallTopUpper, |src, _dest| {
            src.down() == WallTopLower
        }),
        ReplacementRule::to(WallTopUpper, |src, _dest| {
            *src ==WallInnerCornerTopLeft && src.down() == WallTopLower
        }),
        ReplacementRule::to(WallTopUpper, |src, _dest| {
            *src == WallInnerCornerTopRight && src.down() == WallTopLower
        }),
        ReplacementRule::to(WallTopUpper, |src, _dest| {
            *src == WallLeft && src.down() == WallTopLower
        }),
        ReplacementRule::to(WallTopUpper, |src, _dest| {
            *src == WallRight && src.down() == WallTopLower
        }),
        // when rooms are close together make double walls come up to room above
        ReplacementRule::to(WallTopUpper, |src, _dest| {
            *src == WallBottom && src.down() == WallTopLower
        }),
        ReplacementRule::to(WallTopUpper, |src, _dest| {
            *src == WallBottomLeft && src.down() == WallTopLower
        }),
        ReplacementRule::to(WallTopUpper, |src, _dest| {
            *src == WallBottomRight && src.down() == WallTopLower
        }),
        ReplacementRule::to(WallTopUpper, |src, _dest| {
            *src == WallInnerCornerTopLeft && src.down() == WallTopLower
        }),
        ReplacementRule::to(WallTopUpper, |src, _dest| {
            *src == WallInnerCornerTopRight && src.down() == WallTopLower
        }),
    ];

    // third pass wraps the wall in the top most layer
    static ref THIRD_PASS: Vec<ReplacementRule<UtilityTile>> = vec![
        // move top-left outer corner up
        ReplacementRule::none_to(WallTopLeft, |src, _dest| {
            src.down() == Some(WallTopLeft)
        }),
        // move top-right outer corner up
        ReplacementRule::none_to(WallTopRight, |src, _dest| {
            src.down() == Some(WallTopRight)
        }),
        // ┘ - inner corner top left
        ReplacementRule::to(WallInnerCornerBottomRight, |src, _dest| {
            *src == WallTopLeft &&  src.down() == WallTopUpper && src.right() == WallTopUpper
        }),
        ReplacementRule::to(WallInnerCornerBottomRight, |src, _dest| {
            *src == WallBottomLeft && src.down() == WallTopUpper && src.right() == WallTopUpper
        }),
        // └ - inner corner top right
        ReplacementRule::to(WallInnerCornerBottomLeft, |src, _dest| {
            *src == WallTopRight && src.down() == WallTopUpper && src.left() == WallTopUpper
        }),
        ReplacementRule::to(WallInnerCornerBottomLeft, |src, _dest| {
            *src == WallBottomRight && src.down() == WallTopUpper && src.left() == WallTopUpper
        }),
        // swap top-left & top-right to wall left since it was moved up
        ReplacementRule::to(WallLeft, |src, _dest| { *src == WallTopLeft && src.up() == None }),
        ReplacementRule::to(WallRight, |src, _dest| { *src == WallTopRight && src.up() == None }),

        // after the second round of wall tops, some outlines will need to be turned
        // to inner corners
        // ┘ - inner corner top left
        ReplacementRule::to(WallInnerCornerBottomRight, |src, _dest| {
            *src == WallLeft && src.down() == WallTopUpper
                && (src.bottom_left() == Some(WallTopLeft) || src.bottom_left() == WallTopUpper)
        }),
        // └ - inner corner top right
        ReplacementRule::to(WallInnerCornerBottomLeft, |src, _dest| {
            *src == WallRight && src.down() == WallTopUpper
                && (src.bottom_right() == Some(WallTopRight) || src.bottom_right() == WallTopUpper)
        }),
        // ┐ - inner corner bottom left
        ReplacementRule::to(WallInnerCornerTopRight, |src, _dest| {
            *src == WallBottom && src.up() == Floor
                && (src.right() == WallTopUpper || src.right() == WallTopLower) && src.top_right() == Floor
        }),
        // ┌ - inner corner bottom right
        ReplacementRule::to(WallInnerCornerTopLeft, |src, _dest| {
            *src == WallBottom && src.up() == Floor
                && (src.right() == WallTopUpper || src.right() == WallTopLower) && src.top_left() == Floor
        }),

        // top most
        ReplacementRule::none_to(WallTopmost, |src, _dest| {
            src.down() == WallTopUpper
        }),
    ];
}
