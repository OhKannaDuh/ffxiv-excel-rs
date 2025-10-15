/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct ContentMemberType {
    pub row_id: u32,
    pub tanks_per_party: u8,
    pub healers_per_party: u8,
    pub melees_per_party: u8,
    pub ranged_per_party: u8,
}

impl Sheet for ContentMemberType {
    const SHEET_NAME: &'static str = "ContentMemberType";
}

impl FromExcelRow for ContentMemberType {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            tanks_per_party: single_row.columns.get(10).to_u8(),
            healers_per_party: single_row.columns.get(11).to_u8(),
            melees_per_party: single_row.columns.get(12).to_u8(),
            ranged_per_party: single_row.columns.get(13).to_u8(),
        })
    }
}

