/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct ModelSkeleton {
    pub row_id: u32,
    pub radius: f32,
    pub height: f32,
    pub vfx_scale: f32,
    pub float_height: f32,
    pub float_down: f32,
    pub float_up: u16,
    pub motion_blend_type: bool,
    pub loop_fly_se: u8,
}

impl Sheet for ModelSkeleton {
    const SHEET_NAME: &'static str = "ModelSkeleton";
}

impl FromExcelRow for ModelSkeleton {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            radius: single_row.columns.get(0).to_f32(),
            height: single_row.columns.get(1).to_f32(),
            vfx_scale: single_row.columns.get(2).to_f32(),
            float_height: single_row.columns.get(11).to_f32(),
            float_down: single_row.columns.get(12).to_f32(),
            float_up: single_row.columns.get(13).to_u16(),
            motion_blend_type: single_row.columns.get(15).to_bit(0),
            loop_fly_se: single_row.columns.get(16).to_u8(),
        })
    }
}

