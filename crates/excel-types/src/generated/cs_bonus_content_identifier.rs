/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct CSBonusContentIdentifier {
    pub row_id: u32,
    pub content_link_type: u8,
    pub content_id: u32,
    pub map_id: u32,
}

impl Sheet for CSBonusContentIdentifier {
    const SHEET_NAME: &'static str = "CSBonusContentIdentifier";
}

impl FromExcelRow for CSBonusContentIdentifier {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            content_link_type: single_row.columns.get(0).to_u8(),
            content_id: single_row.columns.get(1).to_u32(),
            map_id: single_row.columns.get(7).to_u32(),
        })
    }
}

