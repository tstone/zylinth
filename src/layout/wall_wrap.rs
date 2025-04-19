use super::modifications::TileGrid;
use super::{functional_tiles::UtilityTile, functional_tiles::UtilityTile::*, replacement::*};
use lazy_static::lazy_static;
use rand_chacha::ChaCha8Rng;

pub fn wrap_walls(
    input: Vec<Vec<Vec<Option<UtilityTile>>>>,
    rng: &mut ChaCha8Rng,
) -> Vec<Vec<Vec<Option<UtilityTile>>>> {
    let mut padded = TileGrid::pad(&input, 3, 1, 1, 1);
    replace_tiles(&mut padded, 0, FIRST_PASS.to_vec(), rng);
    // replace_tiles(&mut padded, 0, SECOND_PASS.to_vec(), rng);
    // replace_tiles(&mut padded, 0, THIRD_PASS.to_vec(), rng);
    padded
}

lazy_static! {
    // first pass establishes walls all the way around floors
    static ref FIRST_PASS: Vec<ReplacementRule<UtilityTile>> = vec![
        // top-left outer corner (small)
        ReplacementRule::from_to(Empty, WallTopLeft, |ctx| {
            ctx.up() == None && ctx.down() == None && ctx.right() == None && ctx.bottom_right() == Floor
        }),
        // top-right outer corner (small)
        ReplacementRule::from_to(Empty, WallTopRight, |ctx| {
            ctx.up() == None && ctx.down() == None && ctx.left() == None && ctx.bottom_left() == Floor
        }),
        // bottom-left outer corner (small)
        ReplacementRule::from_to(Empty, WallBottomLeft, |ctx| {
            ctx.up() == None && ctx.down() == None && ctx.right() == None && ctx.top_right() == Floor
        }),
        // bottom-right outer corner (small)
        ReplacementRule::from_to(Empty, WallBottomRight, |ctx| {
            ctx.up() == None && ctx.down() == None && ctx.left() == None && ctx.top_left() == Floor
        }),

        // └ - inner corner top right
        ReplacementRule::from_to(Empty, WallTopLower, |ctx| {
            ctx.up() == None && ctx.left() == Floor && ctx.down() == Floor
        }),
        // ┘ - inner corner top left
        ReplacementRule::from_to(Empty, WallTopLower, |ctx| {
            ctx.up() == None && ctx.right() == Floor && ctx.down() == Floor
        }),
        // ┐ - inner corner bottom left
        ReplacementRule::from_to(Empty, WallInnerCornerTopRight, |ctx| {
            ctx.up() == Floor && ctx.right() == Floor && ctx.top_right() == Floor
        }),
        // ┌ - inner corner bottom right
        ReplacementRule::from_to(Empty, WallInnerCornerTopLeft, |ctx| {
            ctx.up() == Floor && ctx.left() == Floor && ctx.top_left() == Floor
        }),

        // top wall
        ReplacementRule::from_to(Empty, WallTopLower, |ctx| {
            ctx.up() == None && ctx.down() == Floor
        }),
        // left
        ReplacementRule::from_to(Empty, WallLeft, |ctx| {
            ctx.left() == None && ctx.right() == Floor
        }),
        // right
        ReplacementRule::from_to(Empty, WallRight, |ctx| {
            ctx.right() == None && ctx.left() == Floor
        }),
        // bottom
        ReplacementRule::from_to(Empty, WallBottom, |ctx| {
            ctx.down() == None && ctx.up() == Floor
        }),
    ];

    // second pass makes the top wall aler double height
    static ref SECOND_PASS: Vec<ReplacementRule<UtilityTile>> = vec![
        // move top-left outer corner up
        ReplacementRule::from_to(Empty, WallTopLeft, |ctx| {
            ctx.down() == Some(WallTopLeft)
        }),
        // move top-right outer corner up
        ReplacementRule::from_to(Empty, WallTopRight, |ctx| {
            ctx.down() == Some(WallTopRight)
        }),
        // ┘ - inner corner top left
        ReplacementRule::from_to(WallTopLeft, WallTopUpper, |ctx| {
            ctx.down() == WallTopLower && ctx.right() == WallTopLower
        }),
        // └ - inner corner top right
        ReplacementRule::from_to(WallTopRight, WallTopUpper, |ctx| {
            ctx.down() == WallTopLower && ctx.left() == WallTopLower
        }),
        // swap top-left & top-right to wall left since it was moved up
        ReplacementRule::from_to(WallTopLeft, WallLeft, |_| { true }),
        ReplacementRule::from_to(WallTopRight, WallRight, |_| { true }),

        // double top wall
        ReplacementRule::from_to(Empty, WallTopUpper, |ctx| {
            ctx.down() == WallTopLower
        }),
        ReplacementRule::from_to(WallInnerCornerTopLeft, WallTopUpper, |ctx| {
            ctx.down() == WallTopLower
        }),
        ReplacementRule::from_to(WallInnerCornerTopRight, WallTopUpper, |ctx| {
            ctx.down() == WallTopLower
        }),
        ReplacementRule::from_to(WallLeft, WallTopUpper, |ctx| {
            ctx.down() == WallTopLower
        }),
        ReplacementRule::from_to(WallRight, WallTopUpper, |ctx| {
            ctx.down() == WallTopLower
        }),
        // when rooms are close together make double walls come up to room above
        ReplacementRule::from_to(WallBottom, WallTopUpper, |ctx| {
            ctx.down() == WallTopLower
        }),
        ReplacementRule::from_to(WallBottomLeft, WallTopUpper, |ctx| {
            ctx.down() == WallTopLower
        }),
        ReplacementRule::from_to(WallBottomRight, WallTopUpper, |ctx| {
            ctx.down() == WallTopLower
        }),
        ReplacementRule::from_to(WallInnerCornerTopLeft, WallTopUpper, |ctx| {
            ctx.down() == WallTopLower
        }),
        ReplacementRule::from_to(WallInnerCornerTopRight, WallTopUpper, |ctx| {
            ctx.down() == WallTopLower
        }),
    ];

    // third pass wraps the wall in the top most layer
    static ref THIRD_PASS: Vec<ReplacementRule<UtilityTile>> = vec![
        // move top-left outer corner up
        ReplacementRule::from_to(Empty, WallTopLeft, |ctx| {
            ctx.down() == Some(WallTopLeft)
        }),
        // move top-right outer corner up
        ReplacementRule::from_to(Empty, WallTopRight, |ctx| {
            ctx.down() == Some(WallTopRight)
        }),
        // ┘ - inner corner top left
        ReplacementRule::from_to(WallTopLeft, WallInnerCornerBottomRight, |ctx| {
            ctx.down() == WallTopUpper && ctx.right() == WallTopUpper
        }),
        ReplacementRule::from_to(WallBottomLeft, WallInnerCornerBottomRight, |ctx| {
            ctx.down() == WallTopUpper && ctx.right() == WallTopUpper
        }),
        // └ - inner corner top right
        ReplacementRule::from_to(WallTopRight, WallInnerCornerBottomLeft, |ctx| {
            ctx.down() == WallTopUpper && ctx.left() == WallTopUpper
        }),
        ReplacementRule::from_to(WallBottomRight, WallInnerCornerBottomLeft, |ctx| {
            ctx.down() == WallTopUpper && ctx.left() == WallTopUpper
        }),
        // swap top-left & top-right to wall left since it was moved up
        ReplacementRule::from_to(WallTopLeft, WallLeft, |ctx| { ctx.up() == None }),
        ReplacementRule::from_to(WallTopRight, WallRight, |ctx| { ctx.up() == None }),

        // after the second round of wall tops, some outlines will need to be turned
        // to inner corners
        // ┘ - inner corner top left
        ReplacementRule::from_to(WallLeft, WallInnerCornerBottomRight, |ctx| {
            ctx.down() == WallTopUpper && (ctx.bottom_left() == Some(WallTopLeft) || ctx.bottom_left() == WallTopUpper)
        }),
        // └ - inner corner top right
        ReplacementRule::from_to(WallRight, WallInnerCornerBottomLeft, |ctx| {
            ctx.down() == WallTopUpper && (ctx.bottom_right() == Some(WallTopRight) || ctx.bottom_right() == WallTopUpper)
        }),
        // ┐ - inner corner bottom left
        ReplacementRule::from_to(WallBottom, WallInnerCornerTopRight, |ctx| {
            ctx.up() == Floor && (ctx.right() == WallTopUpper || ctx.right() == WallTopLower) && ctx.top_right() == Floor
        }),
        // ┌ - inner corner bottom right
        ReplacementRule::from_to(WallBottom, WallInnerCornerTopLeft, |ctx| {
            ctx.up() == Floor && (ctx.right() == WallTopUpper || ctx.right() == WallTopLower) && ctx.top_left() == Floor
        }),

        // top most
        ReplacementRule::from_to(Empty, WallTopmost, |ctx| {
            ctx.down() == WallTopUpper
        }),
    ];
}
