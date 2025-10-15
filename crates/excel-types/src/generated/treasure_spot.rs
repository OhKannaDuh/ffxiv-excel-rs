/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct TreasureSpot {
    pub row_id: u32,
    pub location_id: u32,
    pub location: RowRef<Level>,
    pub map_offset_x: f32,
    pub map_offset_y: f32,
}

impl Sheet for TreasureSpot {
    const SHEET_NAME: &'static str = "TreasureSpot";
}

impl FromExcelRow for TreasureSpot {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            location_id: single_row.columns.get(0).to_u32(),
            location: RowRef::<Level>::from(single_row.columns.get(0).to_u32()),
            map_offset_x: single_row.columns.get(1).to_f32(),
            map_offset_y: single_row.columns.get(2).to_f32(),
        })
    }
}

