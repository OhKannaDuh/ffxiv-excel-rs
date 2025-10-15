/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct MiniGameRA {
    pub row_id: u32,
    pub icon_id: u32,
    pub image_id: u32,
    pub bgm_id: u32,
    pub bgm: RowRef<BGM>,
}

impl Sheet for MiniGameRA {
    const SHEET_NAME: &'static str = "MiniGameRA";
}

impl FromExcelRow for MiniGameRA {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            icon_id: single_row.columns.get(1).to_u32(),
            image_id: single_row.columns.get(2).to_u32(),
            bgm_id: single_row.columns.get(3).to_u32(),
            bgm: RowRef::<BGM>::from(single_row.columns.get(3).to_u32()),
        })
    }
}

