/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct TiltParam {
    pub row_id: u32,
    pub rotation_origin_offset: u8,
    pub max_angle: u8,
    pub tilt_rate: f32,
    pub reverse_rotation: bool,
}

impl Sheet for TiltParam {
    const SHEET_NAME: &'static str = "TiltParam";
}

impl FromExcelRow for TiltParam {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            rotation_origin_offset: single_row.columns.get(0).to_u8(),
            max_angle: single_row.columns.get(1).to_u8(),
            tilt_rate: single_row.columns.get(3).to_f32(),
            reverse_rotation: single_row.columns.get(2).to_bit(0),
        })
    }
}

