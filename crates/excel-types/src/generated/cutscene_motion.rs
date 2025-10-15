/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct CutsceneMotion {
    pub row_id: u32,
    pub walk_loop_speed: f32,
    pub run_loop_speed: f32,
    pub slowwalk_loop_speed: f32,
    pub slowrun_loop_speed: f32,
    pub battlewalk_loop_speed: f32,
    pub battlerun_loop_speed: f32,
    pub dash_loop_speed: f32,
    pub turn_cw90_frame: u8,
    pub turn_ccw90_frame: u8,
    pub turn_cw180_frame: u8,
    pub turn_ccw180_frame: u8,
}

impl Sheet for CutsceneMotion {
    const SHEET_NAME: &'static str = "CutsceneMotion";
}

impl FromExcelRow for CutsceneMotion {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            walk_loop_speed: single_row.columns.get(0).to_f32(),
            run_loop_speed: single_row.columns.get(1).to_f32(),
            slowwalk_loop_speed: single_row.columns.get(2).to_f32(),
            slowrun_loop_speed: single_row.columns.get(3).to_f32(),
            battlewalk_loop_speed: single_row.columns.get(4).to_f32(),
            battlerun_loop_speed: single_row.columns.get(5).to_f32(),
            dash_loop_speed: single_row.columns.get(6).to_f32(),
            turn_cw90_frame: single_row.columns.get(7).to_u8(),
            turn_ccw90_frame: single_row.columns.get(8).to_u8(),
            turn_cw180_frame: single_row.columns.get(9).to_u8(),
            turn_ccw180_frame: single_row.columns.get(10).to_u8(),
        })
    }
}

