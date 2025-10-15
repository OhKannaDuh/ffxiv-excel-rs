/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct AnimationLOD {
    pub row_id: u32,
    pub camera_distance: f32,
    pub sample_interval: f32,
    pub bone_lod: i8,
}

impl Sheet for AnimationLOD {
    const SHEET_NAME: &'static str = "AnimationLOD";
}

impl FromExcelRow for AnimationLOD {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            camera_distance: single_row.columns.get(0).to_f32(),
            sample_interval: single_row.columns.get(1).to_f32(),
            bone_lod: single_row.columns.get(2).to_i8(),
        })
    }
}

