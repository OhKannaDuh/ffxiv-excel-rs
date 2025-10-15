/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct DeepDungeonDemiclone {
    pub row_id: u32,
    pub icon_id: u32,
    pub singular: String,
    pub plural: String,
    pub title_case: String,
    pub description: String,
}

impl Sheet for DeepDungeonDemiclone {
    const SHEET_NAME: &'static str = "DeepDungeonDemiclone";
}

impl FromExcelRow for DeepDungeonDemiclone {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            icon_id: single_row.columns.get(0).to_u32(),
            singular: single_row.columns.get(1).to_owned_string(),
            plural: single_row.columns.get(3).to_owned_string(),
            title_case: single_row.columns.get(9).to_owned_string(),
            description: single_row.columns.get(10).to_owned_string(),
        })
    }
}

