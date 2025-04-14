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
            ctx.above == None && ctx.below == None && ctx.right == None && ctx.bottom_right == Some(Floor)
        }),
        // top-right outer corner (small)
        Replacement::from_to(Empty, WallTopRight, |ctx| {
            ctx.above == None && ctx.below == None && ctx.left == None && ctx.bottom_left == Some(Floor)
        }),
        // bottom-left outer corner (small)
        Replacement::from_to(Empty, WallBottomLeft, |ctx| {
            ctx.above == None && ctx.below == None && ctx.right == None && ctx.top_right == Some(Floor)
        }),
        // bottom-right outer corner (small)
        Replacement::from_to(Empty, WallBottomRight, |ctx| {
            ctx.above == None && ctx.below == None && ctx.left == None && ctx.top_left == Some(Floor)
        }),

        // └ - inner corner top right
        Replacement::from_to(Empty, WallTopLower, |ctx| {
            ctx.above == None && ctx.left == Some(Floor) && ctx.below == Some(Floor)
        }),
        // ┘ - inner corner top left
        Replacement::from_to(Empty, WallTopLower, |ctx| {
            ctx.above == None && ctx.right == Some(Floor) && ctx.below == Some(Floor)
        }),
        // ┐ - inner corner bottom left
        Replacement::from_to(Empty, WallInnerCornerTopRight, |ctx| {
            ctx.above == Some(Floor) && ctx.right == Some(Floor) && ctx.top_right == Some(Floor)
        }),
        // ┌ - inner corner bottom right
        Replacement::from_to(Empty, WallInnerCornerTopLeft, |ctx| {
            ctx.above == Some(Floor) && ctx.left == Some(Floor) && ctx.top_left == Some(Floor)
        }),

        // top wall
        Replacement::from_to(Empty, WallTopLower, |ctx| {
            ctx.above == None && ctx.below == Some(Floor)
        }),
        // left
        Replacement::from_to(Empty, WallLeft, |ctx| {
            ctx.left == None && ctx.right == Some(Floor)
        }),
        // right
        Replacement::from_to(Empty, WallRight, |ctx| {
            ctx.right == None && ctx.left == Some(Floor)
        }),
        // bottom
        Replacement::from_to(Empty, WallBottom, |ctx| {
            ctx.below == None && ctx.above == Some(Floor)
        }),
    ];

    // second pass makes the top wall aler double height
    static ref SECOND_PASS: Vec<Replacement<UtilityTile, UtilityTile>> = vec![
        // move top-left outer corner up
        Replacement::from_to(Empty, WallTopLeft, |ctx| {
            ctx.below == Some(WallTopLeft)
        }),
        // move top-right outer corner up
        Replacement::from_to(Empty, WallTopRight, |ctx| {
            ctx.below == Some(WallTopRight)
        }),
        // ┘ - inner corner top left
        Replacement::from_to(WallTopLeft, WallTopUpper, |ctx| {
            ctx.below == Some(WallTopLower) && ctx.right == Some(WallTopLower)
        }),
        // └ - inner corner top right
        Replacement::from_to(WallTopRight, WallTopUpper, |ctx| {
            ctx.below == Some(WallTopLower) && ctx.left == Some(WallTopLower)
        }),
        // swap top-left & top-right to wall left since it was moved up
        Replacement::from_to(WallTopLeft, WallLeft, |_| { true }),
        Replacement::from_to(WallTopRight, WallRight, |_| { true }),

        // double top wall
        Replacement::from_to(Empty, WallTopUpper, |ctx| {
            ctx.below == Some(WallTopLower)
        }),
        Replacement::from_to(WallInnerCornerTopLeft, WallTopUpper, |ctx| {
            ctx.below == Some(WallTopLower)
        }),
        Replacement::from_to(WallInnerCornerTopRight, WallTopUpper, |ctx| {
            ctx.below == Some(WallTopLower)
        }),
        Replacement::from_to(WallLeft, WallTopUpper, |ctx| {
            ctx.below == Some(WallTopLower)
        }),
        Replacement::from_to(WallRight, WallTopUpper, |ctx| {
            ctx.below == Some(WallTopLower)
        }),
    ];

    // third pass wraps the wall in the top most layer
    static ref THIRD_PASS: Vec<Replacement<UtilityTile, UtilityTile>> = vec![
        // move top-left outer corner up
        Replacement::from_to(Empty, WallTopLeft, |ctx| {
            ctx.below == Some(WallTopLeft)
        }),
        // move top-right outer corner up
        Replacement::from_to(Empty, WallTopRight, |ctx| {
            ctx.below == Some(WallTopRight)
        }),
        // ┘ - inner corner top left
        Replacement::from_to(WallTopLeft, WallInnerCornerBottomRight, |ctx| {
            ctx.below == Some(WallTopUpper) && ctx.right == Some(WallTopUpper)
        }),
        Replacement::from_to(WallBottomLeft, WallInnerCornerBottomRight, |ctx| {
            ctx.below == Some(WallTopUpper) && ctx.right == Some(WallTopUpper)
        }),
        // └ - inner corner top right
        Replacement::from_to(WallTopRight, WallInnerCornerBottomLeft, |ctx| {
            ctx.below == Some(WallTopUpper) && ctx.left == Some(WallTopUpper)
        }),
        Replacement::from_to(WallBottomRight, WallInnerCornerBottomLeft, |ctx| {
            ctx.below == Some(WallTopUpper) && ctx.left == Some(WallTopUpper)
        }),
        // swap top-left & top-right to wall left since it was moved up
        Replacement::from_to(WallTopLeft, WallLeft, |ctx| { ctx.above.is_none() }),
        Replacement::from_to(WallTopRight, WallRight, |ctx| { ctx.above.is_none() }),

        // after the second round of wall tops, some outlines will need to be turned
        // to inner corners
        // ┘ - inner corner top left
        Replacement::from_to(WallLeft, WallInnerCornerBottomRight, |ctx| {
            ctx.below == Some(WallTopUpper) && ctx.bottom_left == Some(WallTopLeft)
        }),
        // └ - inner corner top right
        Replacement::from_to(WallRight, WallInnerCornerBottomLeft, |ctx| {
            ctx.below == Some(WallTopUpper) && ctx.bottom_right == Some(WallTopRight)
        }),

        // top most
        Replacement::from_to(Empty, WallTopmost, |ctx| {
            ctx.below == Some(WallTopUpper)
        }),
    ];
}
