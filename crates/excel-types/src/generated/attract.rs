/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct Attract {
    pub row_id: u32,
    pub max_distance: u16,
    pub speed: u16,
    pub min_remaining_distance: i16,
    pub use_distance_between_hitboxes: bool,
    pub direction: u8,
}

impl Sheet for Attract {
    const SHEET_NAME: &'static str = "Attract";
}

impl FromExcelRow for Attract {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            max_distance: single_row.columns.get(0).to_u16(),
            speed: single_row.columns.get(1).to_u16(),
            min_remaining_distance: single_row.columns.get(2).to_i16(),
            use_distance_between_hitboxes: single_row.columns.get(3).to_bit(0),
            direction: single_row.columns.get(4).to_u8(),
        })
    }
}

