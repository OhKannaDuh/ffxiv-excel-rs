/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct TerritoryTypeTransient {
    pub row_id: u32,
    pub offset_z: i16,
}

impl Sheet for TerritoryTypeTransient {
    const SHEET_NAME: &'static str = "TerritoryTypeTransient";
}

impl FromExcelRow for TerritoryTypeTransient {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            offset_z: single_row.columns.get(0).to_i16(),
        })
    }
}

