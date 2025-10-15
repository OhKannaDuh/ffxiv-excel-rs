/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct Knockback {
    pub row_id: u32,
    pub distance: u8,
    pub speed: u8,
    pub motion: bool,
    pub near_distance: u8,
    pub direction: u8,
    pub direction_arg: u8,
    pub cancel_move: bool,
}

impl Sheet for Knockback {
    const SHEET_NAME: &'static str = "Knockback";
}

impl FromExcelRow for Knockback {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            distance: single_row.columns.get(0).to_u8(),
            speed: single_row.columns.get(1).to_u8(),
            motion: single_row.columns.get(2).to_bit(0),
            near_distance: single_row.columns.get(3).to_u8(),
            direction: single_row.columns.get(4).to_u8(),
            direction_arg: single_row.columns.get(5).to_u8(),
            cancel_move: single_row.columns.get(6).to_bit(1),
        })
    }
}

