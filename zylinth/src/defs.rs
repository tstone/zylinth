use avian2d::prelude::*;
use bevy::prelude::*;

#[derive(PhysicsLayer, Clone, Copy, Debug, Default)]
pub enum GameLayer {
    #[default]
    Default,
    Player,
    Interactables,
}

#[derive(Component, Debug, Clone)]
pub struct ControlSource {
    pub id: u8,
    pub on: bool,
    pub connected: bool,
}

impl ControlSource {
    pub fn new(id: u8, on: bool, connected: bool) -> Self {
        Self { id, on, connected }
    }

    /// Create a new control target that starts in the 'on' state
    pub fn on(id: u8, connected: bool) -> Self {
        Self::new(id, true, connected)
    }

    /// Create a new control target that starts in the 'off' state
    pub fn off(id: u8, connected: bool) -> Self {
        Self::new(id, false, connected)
    }
}

#[derive(Component, Debug, Clone)]
pub struct ControlTarget {
    pub id: u8,
    pub activated: bool,
    pub connected: bool,
}

impl ControlTarget {
    pub fn new(id: u8, activated: bool, connected: bool) -> Self {
        Self {
            id,
            activated,
            connected,
        }
    }

    /// Create a new control target that starts in the 'activated' state
    pub fn on(id: u8, connected: bool) -> Self {
        Self::new(id, true, connected)
    }

    /// Create a new control target that starts in the 'deactivated' state
    pub fn off(id: u8, connected: bool) -> Self {
        Self::new(id, false, connected)
    }
}

#[derive(Component)]
pub struct ControlLink {
    pub target: u8,
    pub source: u8,
}

impl ControlLink {
    pub fn new(source_id: u8, target_id: u8) -> Self {
        Self {
            target: target_id,
            source: source_id,
        }
    }
}
