/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct MKDLore {
    pub row_id: u32,
    pub image_id: u32,
    pub name: String,
    pub description: String,
}

impl Sheet for MKDLore {
    const SHEET_NAME: &'static str = "MKDLore";
}

impl FromExcelRow for MKDLore {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            image_id: single_row.columns.get(1).to_u32(),
            name: single_row.columns.get(5).to_owned_string(),
            description: single_row.columns.get(6).to_owned_string(),
        })
    }
}

