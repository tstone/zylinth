use bevy::log::*;
use lazy_static::lazy_static;
use rand::Rng;
use tilegen::*;

use super::tuesday::{TuesdayTile, TuesdayTile::*};

pub fn wrap_walls(input: TileGrid<TuesdayTile>, rng: &mut impl Rng) -> TileGrid<TuesdayTile> {
    let mut padded = TileGrid::pad(&input, 2, 1, 1, 1);
    padded.apply_layer_replacements(0, LOWER.to_vec(), rng);
    padded.apply_layer_replacements(0, UPPER.to_vec(), rng);
    padded
}

lazy_static! {
    static ref LOWER: Vec<ReplacementRule<TuesdayTile>> = vec![
        // u
        ReplacementRule::to(WallPanelSingle, |src, _| {
            *src == None && src.down() == Floor && src.left() == Floor && src.right() == Floor
        }),
        // n
        ReplacementRule::to(WallDoubleCornerTop, |src, _| {
            *src == None && src.up() == Floor && src.left() == Floor && src.right() == Floor
        }),
        // c
        ReplacementRule::to(WallPanelLeft, |src, _| {
            *src == None && src.up() == Floor && src.left() == Floor && src.down() == Floor
        }),
        // inverse c
        ReplacementRule::to(WallPanelRight, |src, _| {
            *src == None && src.up() == Floor && src.right() == Floor && src.down() == Floor
        }),

        // double bottom corner
        ReplacementRule::to(WallDoubleUpper, |src, _| {
            *src == None && src.up() == None && src.left() == None && src.right() == None
                && src.top_left() == Floor && src.top_right() == Floor
        }),
        // double top corner
        ReplacementRule::to(WallDoubleLower, |src, _| {
            *src == None && src.down() == None && src.left() == None && src.right() == None
                && src.bottom_left() == Floor && src.bottom_right() == Floor
        }),

        // top left corner
        ReplacementRule {
            condition: |src, _| {
                *src == None && src.down() == None && src.right() == None && src.up() == None && src.bottom_right() == Floor
            },
            replacements: vec![
                Replacement::this(WallLeft),
                Replacement::up(WallTopLeft),
            ],
            ..Default::default()
        },
        // top right corner
        ReplacementRule {
            condition: |src, _| {
                *src == None && src.down() == None && src.left() == None && src.up() == None && src.bottom_left() == Floor
            },
            replacements: vec![
                Replacement::this(WallRight),
                Replacement::up(WallTopRight),
            ],
            ..Default::default()
        },
        // bottom left corner
        ReplacementRule::to(WallPanelLeft, |src, _| {
            *src == None && src.up() == None && src.right() == None && src.top_right() == Floor && src.down() == Floor && src.bottom_right() == Floor
        }),
        ReplacementRule::to(WallPanelSingle, |src, _| {
            *src == None && src.up() == None && src.right() == None && src.top_right() == Floor && src.down() == Floor
        }),
        ReplacementRule::to(WallBottomLeft, |src, dest| {
            *src == None && src.up() == None && dest.right() == WallPanelRight && src.top_right() == Floor
        }),
        ReplacementRule::to(WallBottomLeft, |src, _| {
            *src == None && src.up() == None && src.right() == None && src.top_right() == Floor
        }),
        // bottom right corner
        ReplacementRule::to(WallPanelRight, |src, _| {
            *src == None && src.up() == None && src.left() == None && src.top_left() == Floor && src.down() == Floor && src.bottom_left() == Floor
        }),
        ReplacementRule::to(WallPanelSingle, |src, _| {
            *src == None && src.up() == None && src.left() == None && src.top_left() == Floor && src.down() == Floor
        }),
        ReplacementRule::to(WallPanelRight, |src, dest| {
            *src == None && src.up() == None && dest.left() == WallPanelLeft && src.top_left() == Floor
        }),
        ReplacementRule::to(WallBottomRight, |src, _| {
            *src == None && src.up() == None && src.left() == None && src.top_left() == Floor
        }),

        // └
        ReplacementRule::to(WallInnerCornerBottomLeft, |src, _| {
            *src == None && src.left() == Floor && src.down() == Floor
        }),
        // ┘
        ReplacementRule::to(WallInnerCornerBottomRight, |src, _| {
            *src == None && src.right() == Floor && src.down() == Floor
        }),
        // ┐
        ReplacementRule::to(WallInnerCornerTopRight, |src, _| {
            *src == None && src.right() == Floor && src.up() == Floor
        }),
        // ┌
        ReplacementRule::to(WallInnerCornerTopLeft, |src, _| {
            *src == None && src.left() == Floor && src.up() == Floor
        }),

        // upper walls
        ReplacementRule {
            condition: |src, _| {
                *src == None && src.up() == None && src.down() == Floor
            },
            replacements: vec![
                Replacement::this(WallPanelMiddle),
                Replacement::up(WallTop),
            ],
            ..Default::default()
        },
        // double wall vertical
        ReplacementRule::to(WallDoubleVertical, |src, _| {
            *src == None && src.left() == Floor && src.right() == Floor
        }),
        // double wall horizontal
        ReplacementRule::to(WallPanelMiddle, |src, _| {
            *src == None && src.up() == Floor && src.down() == Floor
        }),
        // left walls
        ReplacementRule::to(WallLeft, |src, _| {
            *src == None && src.right() == Floor
        }),
        // right walls
        ReplacementRule::to(WallRight, |src, _| {
            *src == None && src.left() == Floor
        }),
        // bottom walls
        ReplacementRule::to(WallBottom, |src, _| {
            *src == None && src.up() == Floor
        })
    ];

    static ref UPPER: Vec<ReplacementRule<TuesdayTile>> = vec![
        // ┘
        ReplacementRule::to(WallInnerCornerBottomRight, |src, _| {
            *src == WallLeft && src.left() == WallTopLeft
        }),
        // └
        ReplacementRule::to(WallInnerCornerBottomLeft, |src, _| {
            *src == WallRight && src.right() == WallTopRight
        }),
        // double bump up
        ReplacementRule {
            condition: |src, _ | {
                *src == WallDoubleLower && src.left() == WallPanelMiddle && src.right() == WallPanelMiddle && src.up() == None
            },
            replacements: vec![
                Replacement::this(WallPanelMiddle),
                Replacement::up(WallTop),
                Replacement::down(WallDoubleCornerTop)
            ],
            ..Default::default()
        },
        // double vert to u
        ReplacementRule {
            condition: |src, _ | {
                *src == WallDoubleVertical && (
                    src.down() == WallPanelLeft || src.down() == WallPanelRight || src.down() == WallPanelSingle
                )
            },
            replacements: vec![
                Replacement::this(WallDoubleCornerBottom),
            ],
            ..Default::default()
        },
        // box
        ReplacementRule {
            condition: |src, _| {
                (*src == WallInnerCornerBottomLeft || *src == WallInnerCornerBottomRight || *src == WallInnerCornerTopLeft || *src == WallInnerCornerTopRight) &&
                (src.up() == Floor || src.up() == WallPanelLeft && src.up() == WallPanelRight || src.up() == WallPanelMiddle || src.up() == WallPanelSingle) &&
                (src.down() == Floor || src.down() == WallPanelLeft && src.down() == WallPanelRight || src.down() == WallPanelMiddle || src.down() == WallPanelSingle) &&
                (src.left() == Floor || src.left() == WallPanelLeft && src.left() == WallPanelRight || src.left() == WallPanelMiddle || src.left() == WallPanelSingle) &&
                (src.right() == Floor || src.right() == WallPanelLeft && src.right() == WallPanelRight || src.right() == WallPanelMiddle || src.right() == WallPanelSingle)
            },
            replacements: vec![Replacement::this(WallAllCorner)],
            ..Default::default()
        },
    ];
}
