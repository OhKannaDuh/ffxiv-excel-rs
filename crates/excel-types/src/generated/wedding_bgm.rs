/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct WeddingBGM {
    pub row_id: u32,
    pub song_id: u32,
    pub song: RowRef<BGM>,
    pub song_name: String,
}

impl Sheet for WeddingBGM {
    const SHEET_NAME: &'static str = "WeddingBGM";
}

impl FromExcelRow for WeddingBGM {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            song_id: single_row.columns.get(0).to_u32(),
            song: RowRef::<BGM>::from(single_row.columns.get(0).to_u32()),
            song_name: single_row.columns.get(1).to_owned_string(),
        })
    }
}

