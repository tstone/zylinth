use crate::layout::functional_tiles::UtilityTile;
use rand::prelude::*;

#[derive(Copy, Clone, Default, Debug, PartialEq, Eq, Hash)]
#[allow(dead_code)]
pub enum CosmicLegacyTile {
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

    #[default]
    Empty = -1,
}

impl CosmicLegacyTile {
    pub fn wall_tiles() -> Vec<CosmicLegacyTile> {
        vec![
            Self::Wall,
            Self::Wall3Vents,
            Self::WallCircuits,
            Self::WallFan,
            Self::WallWires,
        ]
    }

    pub fn floor_tiles() -> Vec<CosmicLegacyTile> {
        vec![
            Self::Floor,
            Self::FloorMinorCracks,
            Self::FloorMajorCracks,
            Self::FloorScuffed,
        ]
    }
}

impl Into<u32> for CosmicLegacyTile {
    fn into(self) -> u32 {
        self as u32
    }
}

pub fn utility_to_cosmic(
    input: Vec<Vec<Option<UtilityTile>>>,
    rng: &mut ThreadRng,
) -> Vec<Vec<Option<CosmicLegacyTile>>> {
    let mut output: Vec<Vec<Option<CosmicLegacyTile>>> = vec![vec![]; input.len()];
    for x in 0..input.len() {
        for y in 0..input[x].len() {
            let tile = match input[x][y] {
                Some(UtilityTile::WallTop) => {
                    if rand::random_bool(0.9) {
                        Some(CosmicLegacyTile::Wall)
                    } else {
                        CosmicLegacyTile::wall_tiles().choose(rng).copied()
                    }
                }
                Some(UtilityTile::WallOutlineTopLeft) => Some(CosmicLegacyTile::TopLeftOuterCorner),
                Some(UtilityTile::WallOutlineTopRight) => {
                    Some(CosmicLegacyTile::TopRightOuterCorner)
                }
                Some(UtilityTile::WallLeft) => Some(CosmicLegacyTile::TopCapLeft),
                Some(UtilityTile::WallRight) => Some(CosmicLegacyTile::TopCapRight),
                Some(UtilityTile::WallOutlineTop) => Some(CosmicLegacyTile::TopCapTopSimple),
                Some(UtilityTile::WallOutlineOuterLeft) => {
                    Some(CosmicLegacyTile::BottomRightCorner)
                }
                Some(UtilityTile::WallOutlineOuterRight) => {
                    Some(CosmicLegacyTile::BottomLeftCorner)
                }
                Some(UtilityTile::Floor) => {
                    if rand::random_bool(0.975) {
                        Some(CosmicLegacyTile::Floor)
                    } else {
                        CosmicLegacyTile::floor_tiles().choose(rng).copied()
                    }
                }
                Some(UtilityTile::FloorShadowLeft) => Some(CosmicLegacyTile::FloorShadowLeft),
                Some(UtilityTile::FloorShadowTop) => Some(CosmicLegacyTile::FloorShadowTop),
                Some(UtilityTile::FloorShadowLeftTransition) => {
                    Some(CosmicLegacyTile::FloorShadowLeftFadeUp)
                }
                Some(UtilityTile::FloorShadowTopTransition) => {
                    Some(CosmicLegacyTile::FloorShadowTopFadeLeft)
                }
                Some(UtilityTile::FloorShadowOuterCorner) => {
                    Some(CosmicLegacyTile::FloorShadowOuterCorner)
                }
                Some(UtilityTile::FloorShadowInnerCorner) => {
                    Some(CosmicLegacyTile::FloorShadowInnerCorner)
                }
                Some(UtilityTile::Empty) => None,
                Some(UtilityTile::Test1) => Some(CosmicLegacyTile::Pink),
                Some(UtilityTile::Test2) => Some(CosmicLegacyTile::LightYellow),
                Some(UtilityTile::Test3) => Some(CosmicLegacyTile::DarkestBlue),
                None => None,
            };
            output[x].push(tile);
        }
    }
    output
}
