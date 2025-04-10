use lazy_static::lazy_static;
use std::collections::HashMap;

#[derive(Copy, Clone, Default, Debug, PartialEq, Eq, Hash)]
#[allow(dead_code)]
pub enum CosmicLegacyTiles {
    // Row 1
    TopLeftOuterCorner = 0,
    TopCapTopVar1 = 1,
    TopCapTopVar2 = 2,
    TopRightOuterCorner = 3,
    TransPlateMesh = 4,
    PlateSteel = 5,
    WallLighted = 6,
    WallFan = 7,
    TopCapTopSimple = 8,
    TopCapBottomSimple = 9,
    PlateGrill = 10,
    WallCircuits = 11,
    WallSmall = 12,
    RootsTopLeft = 13,
    RootsTopRight = 14,
    ShelfLeft = 15,
    ShelfRight = 16,
    // Row 2
    TopCapLeft = 17,
    Wall = 18,
    WallSplit = 19,
    TopCapRight = 20,
    BottomRightCorner = 21,
    BottomLeftCorner = 22,
    FloorShadowTopWithLight = 23,
    FloorShadowTop = 24,
    FloorShadowOuterCorner = 25,
    FloorShadowInnerCorner = 26,
    AlienTop = 27,
    Widget = 28, // ???
    NestedBottomLeft = 29,
    RootsBottomLeft = 30,
    RootsBottomRight = 31,
    LockerClosedTop = 32,
    LockerOpenTop = 33,
    // Row 3
    TopCapLeftVar2 = 34,
    FloorCutOut = 35,
    Floor = 36,
    TopCapRightVar2 = 37,
    WallWires = 38,
    Wall3Vents = 39,
    FloorShadowTopFadeLeft = 40,
    FloorSplitShadowTop = 41,
    FloorShadowLeftVar2 = 42,
    FloorShadowLeft = 43,
    AlienBottom = 44,
    FloorDashRight = 45,
    NestedBottomRight = 46,
    Interface1 = 47,
    Interface2 = 48,
    LockerClosedBottom = 49,
    LockerOpenBottom = 50,
    // Row 4
    BottomLeftOuterCorner = 51,
    TopCapBottomVar2 = 52,
    TopCapBottomVar3 = 53,
    BottomRightOuterCorner = 54,
    TopLeftInnerCorner = 55,
    TopRightInnerCorner = 56,
    FloorMinorCracks = 57,
    FloorMajorCracks = 58,
    FloorScuffed = 59,
    FloorShadowLeftFadeUp = 60,
    UtilityBox = 61,
    FloorDashLeft = 62,
    FloorDashTopRight = 63,
    GardenBoxLeft = 64,
    GardenBoxRight = 65,
    PlantLampTop = 66,
    BookcaseTop = 67,
    // Row 5
    Fireplace = 68,
    FloorEndTop = 69,
    FloorEndLeft = 70,
    FloorDrain = 71,
    FloorEndRight = 72,
    FloorEndBottomRight = 73,
    FloorEndBottomLeft = 74,
    FloorEndRightShadowTop = 75,
    PipesLeft = 76,
    PipesStraight = 77,
    PipesRight = 78,
    ComBox = 79,
    LadderTop = 80,
    GardenBoxSmallVar1 = 81,
    GardenBoxSmallVar2 = 82,
    PlantLampBottom = 83,
    BookcaseBottom = 84,
    // Row 6
    // Row 7
    // Row 8
    #[default]
    DarkPink = 119,
    Pink = 120,
    LightPink = 121,
    MedSkin = 122,
    LightSkin = 123,
    LightYellow = 124,
    Peach = 125,
    Tangerine = 126,
    DarkFlesh = 127,
    Maroon = 128,
    Background = 129,
    DarkestBlue = 130,
    DarkTeal = 131,
    Teal = 132,
}

impl CosmicLegacyTiles {
    pub fn wall_tiles() -> Vec<CosmicLegacyTiles> {
        vec![
            Self::Wall,
            Self::Wall3Vents,
            Self::WallCircuits,
            Self::WallFan,
            Self::WallSplit,
            Self::WallWires,
        ]
    }

    pub fn floor_tiles() -> Vec<CosmicLegacyTiles> {
        vec![
            Self::Floor,
            Self::FloorMinorCracks,
            Self::FloorMajorCracks,
            Self::FloorScuffed,
        ]
    }
}

impl Into<u32> for CosmicLegacyTiles {
    fn into(self) -> u32 {
        self as u32
    }
}

pub struct Constraint {
    // TODO: weights
    pub up: Vec<CosmicLegacyTiles>,
    pub down: Vec<CosmicLegacyTiles>,
    pub left: Vec<CosmicLegacyTiles>,
    pub right: Vec<CosmicLegacyTiles>,
}

// TODO: contstraints about "edge" -- ie shadow top can't be on the top row
// This could be some kind of tile like "UpperBorder"

// TODO: constraints 2 hops away




// New architecture:
// room generator -- just floors/walls
// shadowizer -- adds shadows
// variety -- swaps in alternates of floor/wall tiles


lazy_static! {
    pub static ref CONSTRAINTS: HashMap<CosmicLegacyTiles, Constraint> = {
        let mut lookup = HashMap::new();

        lookup.insert(
            CosmicLegacyTiles::Floor,
            Constraint {
                up: vec![
                    CosmicLegacyTiles::Floor,
                    CosmicLegacyTiles::FloorMinorCracks,
                    CosmicLegacyTiles::FloorShadowTop,
                ],
                down: vec![
                    CosmicLegacyTiles::Floor,
                    CosmicLegacyTiles::FloorMinorCracks,
                    CosmicLegacyTiles::Wall,
                ],
                left: vec![
                    CosmicLegacyTiles::Floor,
                    CosmicLegacyTiles::FloorMinorCracks,
                    CosmicLegacyTiles::FloorShadowLeft,
                    CosmicLegacyTiles::Wall,
                ],
                right: vec![
                    CosmicLegacyTiles::Floor,
                    CosmicLegacyTiles::FloorMinorCracks,
                    CosmicLegacyTiles::Wall,
                ],
            },
        );

        lookup.insert(
            CosmicLegacyTiles::FloorMinorCracks,
            Constraint {
                up: vec![CosmicLegacyTiles::Floor, CosmicLegacyTiles::FloorShadowTop],
                down: vec![CosmicLegacyTiles::Floor, CosmicLegacyTiles::Wall],
                left: vec![
                    CosmicLegacyTiles::Floor,
                    CosmicLegacyTiles::FloorShadowLeft,
                    CosmicLegacyTiles::Wall,
                ],
                right: vec![CosmicLegacyTiles::Floor, CosmicLegacyTiles::Wall],
            },
        );

        lookup.insert(
            CosmicLegacyTiles::FloorShadowLeft,
            Constraint {
                up: vec![
                    CosmicLegacyTiles::FloorShadowLeft,
                    CosmicLegacyTiles::FloorShadowLeftFadeUp,
                ],
                down: vec![CosmicLegacyTiles::FloorShadowOuterCorner],
                left: vec![CosmicLegacyTiles::Wall],
                right: vec![
                    CosmicLegacyTiles::Floor,
                    CosmicLegacyTiles::FloorMinorCracks,
                ],
            },
        );

        lookup.insert(
            CosmicLegacyTiles::FloorShadowOuterCorner,
            Constraint {
                up: vec![
                    CosmicLegacyTiles::FloorShadowLeftFadeUp,
                    CosmicLegacyTiles::FloorShadowLeft,
                ],
                down: vec![
                    CosmicLegacyTiles::Floor,
                    CosmicLegacyTiles::FloorMinorCracks,
                ],
                left: vec![CosmicLegacyTiles::FloorShadowTop],
                right: vec![
                    CosmicLegacyTiles::Floor,
                    CosmicLegacyTiles::FloorMinorCracks,
                ],
            },
        );

        lookup.insert(
            CosmicLegacyTiles::FloorShadowLeftFadeUp,
            Constraint {
                up: vec![
                    CosmicLegacyTiles::Floor,
                    CosmicLegacyTiles::FloorMinorCracks,
                ],
                down: vec![CosmicLegacyTiles::FloorShadowOuterCorner],
                left: vec![CosmicLegacyTiles::Wall],
                right: vec![
                    CosmicLegacyTiles::Floor,
                    CosmicLegacyTiles::FloorMinorCracks,
                ],
            },
        );

        lookup.insert(
            CosmicLegacyTiles::FloorShadowTop,
            Constraint {
                up: vec![CosmicLegacyTiles::Wall],
                down: vec![
                    CosmicLegacyTiles::Floor,
                    CosmicLegacyTiles::FloorMinorCracks,
                ],
                left: vec![CosmicLegacyTiles::FloorShadowTop],
                right: vec![
                    CosmicLegacyTiles::FloorShadowTop,
                    CosmicLegacyTiles::FloorShadowOuterCorner,
                ],
            },
        );

        lookup.insert(
            CosmicLegacyTiles::Wall,
            Constraint {
                up: vec![
                    CosmicLegacyTiles::Wall,
                    CosmicLegacyTiles::Floor,
                    CosmicLegacyTiles::FloorMinorCracks,
                ],
                down: vec![CosmicLegacyTiles::FloorShadowTop, CosmicLegacyTiles::Wall],
                left: vec![
                    CosmicLegacyTiles::Floor,
                    CosmicLegacyTiles::FloorMinorCracks,
                    CosmicLegacyTiles::Wall,
                ],
                right: vec![
                    CosmicLegacyTiles::FloorShadowLeft,
                    CosmicLegacyTiles::FloorShadowLeftFadeUp,
                    CosmicLegacyTiles::Wall,
                ],
            },
        );

        lookup
    };
}
