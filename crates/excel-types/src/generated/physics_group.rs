/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct PhysicsGroup {
    pub row_id: u32,
    pub reset_by_look_at: bool,
    pub root_following_game: f32,
    pub root_following_cut_scene: f32,
    pub force_attract_by_physics_off: bool,
}

impl Sheet for PhysicsGroup {
    const SHEET_NAME: &'static str = "PhysicsGroup";
}

impl FromExcelRow for PhysicsGroup {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            reset_by_look_at: single_row.columns.get(12).to_bit(0),
            root_following_game: single_row.columns.get(13).to_f32(),
            root_following_cut_scene: single_row.columns.get(14).to_f32(),
            force_attract_by_physics_off: single_row.columns.get(18).to_bit(1),
        })
    }
}

