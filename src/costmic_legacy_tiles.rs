#[derive(Copy, Clone, Default, Debug)]
#[allow(dead_code)]

pub enum CosmicLegacyTiles {
    // Row 1
    TopLeftOuterCorner = 0,
    TopCapTopVar1 = 1,
    TopCapTopVar2 = 2,
    TopRightOuterCorner = 3,
    PlateMesh = 4,
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
    FloorShadowTopLeft = 25,
    FloorShadowTopCorner = 26,
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
    FloorShadowTopRight = 40,
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
    TopLeftInerCorner = 55,
    TopRightInnerCorner = 56,
    FloorMinorCracks = 57,
    FloorMajorCracks = 58,
    FloorScuffed = 59,
    FloorShadowBottomRight = 60,
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
    FloorEndRightShadowTOp = 75,
    PipesLeft = 76,
    PipesStraight = 77,
    PipesRight = 78,
    ComBox = 79,
    LadderTop = 80,
    GardenBoxSmallVar1 = 81,
    GardenBoxSmallVar2 = 82,
    PlantLampBottom = 83,
    BookcaseBottom = 84,
    // Row 6 - 85
    // Row 7 - 101
    // Row 8 - 117
    #[default]
    Background = 127,
    DarkestBlue = 128,
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
            Self::FloorCutOut,
            Self::FloorDrain,
            Self::FloorMinorCracks,
            Self::FloorMajorCracks,
        ]
    }
}

impl Into<u32> for CosmicLegacyTiles {
    fn into(self) -> u32 {
        self as u32
    }
}
