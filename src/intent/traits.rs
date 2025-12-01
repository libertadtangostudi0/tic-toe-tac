// src/intent/traits.rs

use bevy::prelude::Vec2;

use crate::intent::components::{
    ClickIntent,
    CellClickIntent,
    UiClickIntent,
};


/// Useful for logging and debugging.
pub trait NamedIntent {
    fn intent_name(&self) -> &'static str;
}

/// Intent that has a position in world space.
pub trait PositionalIntent {
    fn world_pos(&self) -> Vec2;
}

/// Intent that targets a particular board cell.
pub trait CellIntent {
    fn cell(&self) -> (u8, u8);
}

// Implementations for your current intent types

impl NamedIntent for ClickIntent {
    fn intent_name(&self) -> &'static str {
        "ClickIntent"
    }
}

impl NamedIntent for CellClickIntent {
    fn intent_name(&self) -> &'static str {
        "CellClickIntent"
    }
}

impl NamedIntent for UiClickIntent {
    fn intent_name(&self) -> &'static str {
        "UiClickIntent"
    }
}

impl PositionalIntent for ClickIntent {
    fn world_pos(&self) -> Vec2 {
        self.world_pos
    }
}

impl CellIntent for CellClickIntent {
    fn cell(&self) -> (u8, u8) {
        (self.row, self.col)
    }
}
