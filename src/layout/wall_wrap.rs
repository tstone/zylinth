use super::{
    functional_tiles::UtilityTile, functional_tiles::UtilityTile::*, modifications::padding,
    replacement::*,
};
use bevy::log::*;
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
        Replacement::from_to(Empty, WallTop, |ctx| {
            ctx.above == None && ctx.left == Some(Floor) && ctx.below == Some(Floor)
        }),
        // ┘ - inner corner top left
        Replacement::from_to(Empty, WallTop, |ctx| {
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
        Replacement::from_to(Empty, WallTop, |ctx| {
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
        Replacement::from_to(WallTopLeft, WallTop, |ctx| {
            ctx.below == Some(WallTop) && ctx.right == Some(WallTop)
        }),
        // └ - inner corner top right
        Replacement::from_to(WallTopRight, WallTop, |ctx| {
            ctx.below == Some(WallTop) && ctx.left == Some(WallTop)
        }),
        // swap top-left & top-right to wall left since it was moved up
        Replacement::from_to(WallTopLeft, WallLeft, |_| { true }),
        Replacement::from_to(WallTopRight, WallRight, |_| { true }),

        // double top wall
        Replacement::from_to(Empty, WallTop, |ctx| {
            ctx.below == Some(WallTop)
        }),
        Replacement::from_to(WallInnerCornerTopLeft, WallTop, |ctx| {
            ctx.below == Some(WallTop)
        }),
        Replacement::from_to(WallInnerCornerTopRight, WallTop, |ctx| {
            ctx.below == Some(WallTop)
        }),
        Replacement::from_to(WallLeft, WallTop, |ctx| {
            ctx.below == Some(WallTop)
        }),
        Replacement::from_to(WallRight, WallTop, |ctx| {
            ctx.below == Some(WallTop)
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
            ctx.below == Some(WallTop) && ctx.right == Some(WallTop)
        }),
        Replacement::from_to(WallBottomLeft, WallInnerCornerBottomRight, |ctx| {
            ctx.below == Some(WallTop) && ctx.right == Some(WallTop)
        }),
        // └ - inner corner top right
        Replacement::from_to(WallTopRight, WallInnerCornerBottomLeft, |ctx| {
            ctx.below == Some(WallTop) && ctx.left == Some(WallTop)
        }),
        Replacement::from_to(WallBottomRight, WallInnerCornerBottomLeft, |ctx| {
            ctx.below == Some(WallTop) && ctx.left == Some(WallTop)
        }),
        // swap top-left & top-right to wall left since it was moved up
        Replacement::from_to(WallTopLeft, WallLeft, |ctx| { ctx.above.is_none() }),
        Replacement::from_to(WallTopRight, WallRight, |ctx| { ctx.above.is_none() }),

        // after the second round of wall tops, some outlines will need to be turned
        // to inner corners
        // ┘ - inner corner top left
        Replacement::from_to(WallLeft, WallInnerCornerBottomRight, |ctx| {
            ctx.below == Some(WallTop) && ctx.bottom_left == Some(WallTopLeft)
        }),
        // └ - inner corner top right
        Replacement::from_to(WallRight, WallInnerCornerBottomLeft, |ctx| {
            ctx.below == Some(WallTop) && ctx.bottom_right == Some(WallTopRight)
        }),

        // top most
        Replacement::from_to(Empty, WallTopmost, |ctx| {
            ctx.below == Some(WallTop)
        }),
    ];

    // static ref THIRD_PASS: Vec<Replacement<UtilityTile, UtilityTile>> = vec![
    //     // inner bottom left
    //     Replacement {
    //         target: UtilityTile::WallOutlineTopRight,
    //         on_left: HashSet::from([UtilityTile::WallTop]),
    //         on_bottom_left: HashSet::from([UtilityTile::WallTop]),
    //         below: HashSet::from([UtilityTile::WallTop]),
    //         replacement: UtilityTile::WallOutlineInnerCornerBottomLeft,
    //         ..Default::default()
    //     },
    //     // inner bottom right
    //     Replacement {
    //         target: UtilityTile::WallOutlineTopLeft,
    //         on_right: HashSet::from([UtilityTile::WallTop]),
    //         on_bottom_right: HashSet::from([UtilityTile::WallTop]),
    //         below: HashSet::from([UtilityTile::WallTop]),
    //         replacement: UtilityTile::WallOutlineInnerCornerBottomRight,
    //         ..Default::default()
    //     },
    //     // move caps up a third time to be on top of 2nd wall
    //     Replacement {
    //         target: UtilityTile::Empty,
    //         below: HashSet::from([UtilityTile::WallOutlineTopLeft]),
    //         above: HashSet::from([UtilityTile::Empty]),
    //         replacement: UtilityTile::WallOutlineTopLeft,
    //         ..Default::default()
    //     },
    //     Replacement {
    //         target: UtilityTile::WallOutlineTopLeft,
    //         above: HashSet::from([UtilityTile::WallOutlineTopLeft]),
    //         replacement: UtilityTile::WallLeft,
    //         ..Default::default()
    //     },
    //     Replacement {
    //         target: UtilityTile::Empty,
    //         below: HashSet::from([UtilityTile::WallOutlineTopRight]),
    //         above: HashSet::from([UtilityTile::Empty]),
    //         replacement: UtilityTile::WallOutlineTopRight,
    //         ..Default::default()
    //     },
    //     Replacement {
    //         target: UtilityTile::WallOutlineTopRight,
    //         above: HashSet::from([UtilityTile::WallOutlineTopRight]),
    //         replacement: UtilityTile::WallRight,
    //         ..Default::default()
    //     },
    //     Replacement {
    //         target: UtilityTile::WallOutlineInnerCornerTopRight,
    //         below: HashSet::from([UtilityTile::WallTop]),
    //         replacement: UtilityTile::WallTop,
    //         ..Default::default()
    //     },
    //     // Top cap
    //     Replacement {
    //         target: UtilityTile::Empty,
    //         below: HashSet::from([UtilityTile::WallTop]),
    //         above: HashSet::from([UtilityTile::Empty]),
    //         replacement: UtilityTile::WallOutlineTop,
    //         ..Default::default()
    //     },
    //     // inner bottom-right corner
    //     Replacement {
    //         target: UtilityTile::WallLeft,
    //         on_left: HashSet::from([UtilityTile::WallOutlineTop, UtilityTile::WallOutlineTopLeft]),
    //         above: HashSet::from([UtilityTile::WallLeft, UtilityTile::WallOutlineTopLeft]),
    //         below: HashSet::from([UtilityTile::WallTop]),
    //         replacement: UtilityTile::WallOutlineInnerCornerBottomRight,
    //         ..Default::default()
    //     },
    //     // inner bottom-left corner
    //     Replacement {
    //         target: UtilityTile::WallRight,
    //         on_left: HashSet::from([UtilityTile::Floor]),
    //         above: HashSet::from([UtilityTile::WallRight, UtilityTile::WallOutlineTopRight]),
    //         below: HashSet::from([UtilityTile::WallTop]),
    //         replacement: UtilityTile::WallOutlineInnerCornerBottomLeft,
    //         ..Default::default()
    //     },
    // ];

    // TODO: also handle top left/right inner corner to wall top if above wall top
}
