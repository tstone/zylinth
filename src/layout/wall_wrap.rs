use super::{
    functional_tiles::UtilityTile, functional_tiles::UtilityTile::*, modifications::padding,
    replacement::*,
};
use lazy_static::lazy_static;
use rand_chacha::ChaCha8Rng;

pub fn wrap_walls(
    input: Vec<Vec<Option<UtilityTile>>>,
    rng: &mut ChaCha8Rng,
) -> Vec<Vec<Option<UtilityTile>>> {
    let padded = padding(input, 3, 1, 1, 1);
    let p1 = replace_tiles(&padded, FIRST_PASS.to_vec(), padded.clone(), rng);
    let p2 = replace_tiles(&p1, SECOND_PASS.to_vec(), p1.clone(), rng);
    replace_tiles(&p2, THIRD_PASS.to_vec(), p2.clone(), rng)
}

lazy_static! {
    // first pass establishes walls all the way around floors
    static ref FIRST_PASS: Vec<Replacement<UtilityTile, UtilityTile>> = vec![
        // top-left outer corner (small)
        Replacement::from_to(Empty, WallTopLeft, |ctx| {
            ctx.above() == None && ctx.below() == None && ctx.right() == None && ctx.bottom_right() == Floor
        }),
        // top-right outer corner (small)
        Replacement::from_to(Empty, WallTopRight, |ctx| {
            ctx.above() == None && ctx.below() == None && ctx.left() == None && ctx.bottom_left() == Floor
        }),
        // bottom-left outer corner (small)
        Replacement::from_to(Empty, WallBottomLeft, |ctx| {
            ctx.above() == None && ctx.below() == None && ctx.right() == None && ctx.top_right() == Floor
        }),
        // bottom-right outer corner (small)
        Replacement::from_to(Empty, WallBottomRight, |ctx| {
            ctx.above() == None && ctx.below() == None && ctx.left() == None && ctx.top_left() == Floor
        }),

        // └ - inner corner top right
        Replacement::from_to(Empty, WallTopLower, |ctx| {
            ctx.above() == None && ctx.left() == Floor && ctx.below() == Floor
        }),
        // ┘ - inner corner top left
        Replacement::from_to(Empty, WallTopLower, |ctx| {
            ctx.above() == None && ctx.right() == Floor && ctx.below() == Floor
        }),
        // ┐ - inner corner bottom left
        Replacement::from_to(Empty, WallInnerCornerTopRight, |ctx| {
            ctx.above() == Floor && ctx.right() == Floor && ctx.top_right() == Floor
        }),
        // ┌ - inner corner bottom right
        Replacement::from_to(Empty, WallInnerCornerTopLeft, |ctx| {
            ctx.above() == Floor && ctx.left() == Floor && ctx.top_left() == Floor
        }),

        // top wall
        Replacement::from_to(Empty, WallTopLower, |ctx| {
            ctx.above() == None && ctx.below() == Floor
        }),
        // left
        Replacement::from_to(Empty, WallLeft, |ctx| {
            ctx.left() == None && ctx.right() == Floor
        }),
        // right
        Replacement::from_to(Empty, WallRight, |ctx| {
            ctx.right() == None && ctx.left() == Floor
        }),
        // bottom
        Replacement::from_to(Empty, WallBottom, |ctx| {
            ctx.below() == None && ctx.above() == Floor
        }),
    ];

    // second pass makes the top wall aler double height
    static ref SECOND_PASS: Vec<Replacement<UtilityTile, UtilityTile>> = vec![
        // move top-left outer corner up
        Replacement::from_to(Empty, WallTopLeft, |ctx| {
            ctx.below() == Some(WallTopLeft)
        }),
        // move top-right outer corner up
        Replacement::from_to(Empty, WallTopRight, |ctx| {
            ctx.below() == Some(WallTopRight)
        }),
        // ┘ - inner corner top left
        Replacement::from_to(WallTopLeft, WallTopUpper, |ctx| {
            ctx.below() == WallTopLower && ctx.right() == WallTopLower
        }),
        // └ - inner corner top right
        Replacement::from_to(WallTopRight, WallTopUpper, |ctx| {
            ctx.below() == WallTopLower && ctx.left() == WallTopLower
        }),
        // swap top-left & top-right to wall left since it was moved up
        Replacement::from_to(WallTopLeft, WallLeft, |_| { true }),
        Replacement::from_to(WallTopRight, WallRight, |_| { true }),

        // double top wall
        Replacement::from_to(Empty, WallTopUpper, |ctx| {
            ctx.below() == WallTopLower
        }),
        Replacement::from_to(WallInnerCornerTopLeft, WallTopUpper, |ctx| {
            ctx.below() == WallTopLower
        }),
        Replacement::from_to(WallInnerCornerTopRight, WallTopUpper, |ctx| {
            ctx.below() == WallTopLower
        }),
        Replacement::from_to(WallLeft, WallTopUpper, |ctx| {
            ctx.below() == WallTopLower
        }),
        Replacement::from_to(WallRight, WallTopUpper, |ctx| {
            ctx.below() == WallTopLower
        }),
        // when rooms are close together make double walls come up to room above
        Replacement::from_to(WallBottom, WallTopUpper, |ctx| {
            ctx.below() == WallTopLower
        }),
        Replacement::from_to(WallBottomLeft, WallTopUpper, |ctx| {
            ctx.below() == WallTopLower
        }),
        Replacement::from_to(WallBottomRight, WallTopUpper, |ctx| {
            ctx.below() == WallTopLower
        }),
        Replacement::from_to(WallInnerCornerTopLeft, WallTopUpper, |ctx| {
            ctx.below() == WallTopLower
        }),
        Replacement::from_to(WallInnerCornerTopRight, WallTopUpper, |ctx| {
            ctx.below() == WallTopLower
        }),
    ];

    // third pass wraps the wall in the top most layer
    static ref THIRD_PASS: Vec<Replacement<UtilityTile, UtilityTile>> = vec![
        // move top-left outer corner up
        Replacement::from_to(Empty, WallTopLeft, |ctx| {
            ctx.below() == Some(WallTopLeft)
        }),
        // move top-right outer corner up
        Replacement::from_to(Empty, WallTopRight, |ctx| {
            ctx.below() == Some(WallTopRight)
        }),
        // ┘ - inner corner top left
        Replacement::from_to(WallTopLeft, WallInnerCornerBottomRight, |ctx| {
            ctx.below() == WallTopUpper && ctx.right() == WallTopUpper
        }),
        Replacement::from_to(WallBottomLeft, WallInnerCornerBottomRight, |ctx| {
            ctx.below() == WallTopUpper && ctx.right() == WallTopUpper
        }),
        // └ - inner corner top right
        Replacement::from_to(WallTopRight, WallInnerCornerBottomLeft, |ctx| {
            ctx.below() == WallTopUpper && ctx.left() == WallTopUpper
        }),
        Replacement::from_to(WallBottomRight, WallInnerCornerBottomLeft, |ctx| {
            ctx.below() == WallTopUpper && ctx.left() == WallTopUpper
        }),
        // swap top-left & top-right to wall left since it was moved up
        Replacement::from_to(WallTopLeft, WallLeft, |ctx| { ctx.above() == None }),
        Replacement::from_to(WallTopRight, WallRight, |ctx| { ctx.above() == None }),

        // after the second round of wall tops, some outlines will need to be turned
        // to inner corners
        // ┘ - inner corner top left
        Replacement::from_to(WallLeft, WallInnerCornerBottomRight, |ctx| {
            ctx.below() == WallTopUpper && (ctx.bottom_left() == Some(WallTopLeft) || ctx.bottom_left() == WallTopUpper)
        }),
        // └ - inner corner top right
        Replacement::from_to(WallRight, WallInnerCornerBottomLeft, |ctx| {
            ctx.below() == WallTopUpper && (ctx.bottom_right() == Some(WallTopRight) || ctx.bottom_right() == WallTopUpper)
        }),
        // ┐ - inner corner bottom left
        Replacement::from_to(WallBottom, WallInnerCornerTopRight, |ctx| {
            ctx.above() == Floor && (ctx.right() == WallTopUpper || ctx.right() == WallTopLower) && ctx.top_right() == Floor
        }),
        // ┌ - inner corner bottom right
        Replacement::from_to(WallBottom, WallInnerCornerTopLeft, |ctx| {
            ctx.above() == Floor && (ctx.right() == WallTopUpper || ctx.right() == WallTopLower) && ctx.top_left() == Floor
        }),

        // top most
        Replacement::from_to(Empty, WallTopmost, |ctx| {
            ctx.below() == WallTopUpper
        }),
    ];
}
