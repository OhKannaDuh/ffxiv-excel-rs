/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct EObj {
    pub row_id: u32,
    pub pop_type: u8,
    pub data: u32,
    pub invisibility: u8,
    pub sgb_path_id: u32,
    pub sgb_path: RowRef<ExportedSG>,
    pub eye_collision: bool,
    pub director_control: bool,
    pub target: bool,
    pub event_high_addition: u8,
    pub added_in_5_3: bool,
}

impl Sheet for EObj {
    const SHEET_NAME: &'static str = "EObj";
}

impl FromExcelRow for EObj {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            pop_type: single_row.columns.get(8).to_u8(),
            data: single_row.columns.get(9).to_u32(),
            invisibility: single_row.columns.get(10).to_u8(),
            sgb_path_id: single_row.columns.get(11).to_u32(),
            sgb_path: RowRef::<ExportedSG>::from(single_row.columns.get(11).to_u32()),
            eye_collision: single_row.columns.get(12).to_bit(0),
            director_control: single_row.columns.get(13).to_bit(1),
            target: single_row.columns.get(14).to_bit(2),
            event_high_addition: single_row.columns.get(15).to_u8(),
            added_in_5_3: single_row.columns.get(18).to_bit(4),
        })
    }
}

