use super::{functional_tiles::UtilityTile, modifications::padding, replacement::*};
use bevy::log::*;
use lazy_static::lazy_static;
use rand_chacha::ChaCha8Rng;

pub fn wrap_walls(
    input: Vec<Vec<Option<UtilityTile>>>,
    rng: &mut ChaCha8Rng,
) -> Vec<Vec<Option<UtilityTile>>> {
    let padded = padding(input, 3, 1, 1, 1);

    // 2 height walls
    let p1 = replace_tiles(&padded, FIRST_PASS.to_vec(), padded.clone(), rng);
    return p1;
    // let p2 = replace_tiles(&p1, SECOND_PASS.to_vec(), p1.clone(), rng);
    // replace_tiles(&p2, THIRD_PASS.to_vec(), p2.clone(), rng)
}

lazy_static! {
    static ref FIRST_PASS: Vec<Replacement<UtilityTile, UtilityTile>> = vec![
        // top-left
        Replacement::from_to(UtilityTile::Empty, UtilityTile::WallOutlineTopLeft, |ctx| {
            ctx.above == None && ctx.below == None && ctx.bottom_right == Some(UtilityTile::Floor)
        }),
        // Replacement {
        //     desc: "wall top-right",
        //     target: UtilityTile::Empty,
        //     above: HashSet::from([UtilityTile::Empty]),
        //     below: HashSet::from([UtilityTile::Empty]),
        //     on_bottom_left: HashSet::from([UtilityTile::Floor]),
        //     replacement: UtilityTile::WallOutlineTopRight,
        //     ..Default::default()
        // },
        // Replacement {
        //     desc: "wall inner top-left",
        //     target: UtilityTile::Empty,
        //     below: HashSet::from([UtilityTile::Floor]),
        //     on_right: HashSet::from([UtilityTile::Floor]),
        //     on_bottom_right: HashSet::from([UtilityTile::Floor]),
        //     replacement: UtilityTile::WallTop,
        //     ..Default::default()
        // },
        // Replacement {
        //     desc: "wall inner top-right",
        //     target: UtilityTile::Empty,
        //     above: HashSet::from([UtilityTile::WallLeft, UtilityTile::WallOutlineTopLeft, UtilityTile::Empty]),
        //     below: HashSet::from([UtilityTile::Floor]),
        //     on_left: HashSet::from([UtilityTile::Floor]),
        //     on_bottom_right: HashSet::from([UtilityTile::Floor]),
        //     replacement: UtilityTile::WallTop,
        //     ..Default::default()
        // },
        // Replacement {
        //     desc: "wall inner bottom-left",
        //     target: UtilityTile::Empty,
        //     above: HashSet::from([UtilityTile::WallRight, UtilityTile::WallOutlineTopRight, UtilityTile::Empty]),
        //     on_left: HashSet::from([UtilityTile::Floor]),
        //     on_bottom_left: HashSet::from([UtilityTile::Floor]),
        //     below: HashSet::from([UtilityTile::Floor]),
        //     replacement: UtilityTile::WallTop,
        //     ..Default::default()
        // },
        // Replacement {
        //     desc: "wall inner bottom-right",
        //     target: UtilityTile::Empty,
        //     above: HashSet::from([UtilityTile::WallRight]),
        //     on_right: HashSet::from([UtilityTile::Floor]),
        //     on_bottom_right: HashSet::from([UtilityTile::Floor]),
        //     below: HashSet::from([UtilityTile::Floor]),
        //     replacement: UtilityTile::WallTop,
        //     ..Default::default()
        // },
        // Replacement {
        //     desc: "wall inner top-left #2",
        //     target: UtilityTile::Empty,
        //     above: HashSet::from([UtilityTile::Floor]),
        //     on_left: HashSet::from([UtilityTile::Floor]),
        //     on_top_left: HashSet::from([UtilityTile::Floor]),
        //     replacement: UtilityTile::WallOutlineInnerCornerTopLeft,
        //     ..Default::default()
        // },
        // Replacement {
        //     desc: "wall inner top-right #2",
        //     target: UtilityTile::Empty,
        //     above: HashSet::from([UtilityTile::Floor]),
        //     on_right: HashSet::from([UtilityTile::Floor]),
        //     on_top_right: HashSet::from([UtilityTile::Floor]),
        //     replacement: UtilityTile::WallOutlineInnerCornerTopRight,
        //     ..Default::default()
        // },
        // Replacement {
        //     desc: "wall top",
        //     target: UtilityTile::Empty,
        //     below: HashSet::from([UtilityTile::Floor]),
        //     above: HashSet::from([UtilityTile::Empty]),
        //     replacement: UtilityTile::WallTop,
        //     ..Default::default()
        // },
        // Replacement {
        //     desc: "wall inverse L",
        //     target: UtilityTile::Empty,
        //     on_right: HashSet::from([UtilityTile::Floor]),
        //     below: HashSet::from([UtilityTile::Floor]),
        //     on_bottom_right: HashSet::from([UtilityTile::Floor]),
        //     replacement: UtilityTile::WallTop,
        //     ..Default::default()
        // },
        // Replacement {
        //     desc: "wall L",
        //     target: UtilityTile::Empty,
        //     on_left: HashSet::from([UtilityTile::Floor]),
        //     below: HashSet::from([UtilityTile::Floor]),
        //     on_bottom_left: HashSet::from([UtilityTile::Floor]),
        //     replacement: UtilityTile::WallTop,
        //     ..Default::default()
        // },
        // Replacement {
        //     desc: "wall left",
        //     target: UtilityTile::Empty,
        //     on_right: HashSet::from([UtilityTile::Floor]),
        //     on_left: HashSet::from([UtilityTile::Empty]),
        //     replacement: UtilityTile::WallLeft,
        //     ..Default::default()
        // },
        // Replacement {
        //     desc: "wall right",
        //     target: UtilityTile::Empty,
        //     on_left: HashSet::from([UtilityTile::Floor]),
        //     on_right: HashSet::from([UtilityTile::Empty]),
        //     below: HashSet::from([UtilityTile::Empty]),
        //     replacement: UtilityTile::WallRight,
        //     ..Default::default()
        // },
        // Replacement {
        //     desc: "wall bottom",
        //     target: UtilityTile::Empty,
        //     above: HashSet::from([UtilityTile::Floor]),
        //     replacement: UtilityTile::WallBottom,
        //     ..Default::default()
        // },
        // Replacement {
        //     desc: "wall bottom outer left",
        //     target: UtilityTile::Empty,
        //     above: HashSet::from([UtilityTile::WallLeft, UtilityTile::WallOutlineInnerCornerTopRight]),
        //     replacement: UtilityTile::WallBottomLeft,
        //     ..Default::default()
        // },
        // Replacement {
        //     desc: "wall bottom outer right",
        //     target: UtilityTile::Empty,
        //     above: HashSet::from([UtilityTile::WallRight, UtilityTile::WallOutlineInnerCornerTopLeft]),
        //     on_left: HashSet::from([UtilityTile::WallBottom, UtilityTile::WallBottomLeft, UtilityTile::WallOutlineInnerCornerTopLeft]),
        //     replacement: UtilityTile::WallBottomRight,
        //     ..Default::default()
        // },
    ];

    // static ref SECOND_PASS: Vec<Replacement<UtilityTile, UtilityTile>> = vec![
    //     // move outline top left up one
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
    //     // move outline top right up one
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
    //     // inner-left corner 2nd wall
    //     Replacement {
    //         target: UtilityTile::WallLeft,
    //         below: HashSet::from([UtilityTile::WallTop]),
    //         replacement: UtilityTile::WallTop,
    //         ..Default::default()
    //     },
    //     // inner-right corner 2nd wall
    //     Replacement {
    //         target: UtilityTile::WallRight,
    //         below: HashSet::from([UtilityTile::WallTop]),
    //         replacement: UtilityTile::WallTop,
    //         ..Default::default()
    //     },
    //     // 2 story wall
    //     Replacement {
    //         target: UtilityTile::Empty,
    //         below: HashSet::from([UtilityTile::WallTop]),
    //         above: HashSet::from([UtilityTile::Empty]),
    //         replacement: UtilityTile::WallTop,
    //         ..Default::default()
    //     },
    // ];

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
}
