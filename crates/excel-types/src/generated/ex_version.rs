/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct ExVersion {
    pub row_id: u32,
    pub name: String,
    pub accept_jingle_id: u32,
    pub accept_jingle: RowRef<ScreenImage>,
    pub complete_jingle_id: u32,
    pub complete_jingle: RowRef<ScreenImage>,
}

impl Sheet for ExVersion {
    const SHEET_NAME: &'static str = "ExVersion";
}

impl FromExcelRow for ExVersion {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            name: single_row.columns.get(0).to_owned_string(),
            accept_jingle_id: single_row.columns.get(1).to_u32(),
            accept_jingle: RowRef::<ScreenImage>::from(single_row.columns.get(1).to_u32()),
            complete_jingle_id: single_row.columns.get(2).to_u32(),
            complete_jingle: RowRef::<ScreenImage>::from(single_row.columns.get(2).to_u32()),
        })
    }
}

