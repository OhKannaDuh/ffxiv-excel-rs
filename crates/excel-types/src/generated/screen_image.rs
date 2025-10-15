/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct ScreenImage {
    pub row_id: u32,
    pub image_id: u32,
    pub jingle_id: u32,
    pub jingle: RowRef<Jingle>,
    pub _type: i8,
    pub lang: bool,
}

impl Sheet for ScreenImage {
    const SHEET_NAME: &'static str = "ScreenImage";
}

impl FromExcelRow for ScreenImage {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            image_id: single_row.columns.get(0).to_u32(),
            jingle_id: single_row.columns.get(1).to_u32(),
            jingle: RowRef::<Jingle>::from(single_row.columns.get(1).to_u32()),
            _type: single_row.columns.get(2).to_i8(),
            lang: single_row.columns.get(3).to_bit(0),
        })
    }
}

