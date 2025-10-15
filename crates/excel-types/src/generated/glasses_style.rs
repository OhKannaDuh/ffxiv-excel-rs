/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct GlassesStyle {
    pub row_id: u32,
    pub icon_id: u32,
    pub order: u16,
    pub singular: String,
    pub plural: String,
    pub name: String,
}

impl Sheet for GlassesStyle {
    const SHEET_NAME: &'static str = "GlassesStyle";
}

impl FromExcelRow for GlassesStyle {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            icon_id: single_row.columns.get(1).to_u32(),
            order: single_row.columns.get(2).to_u16(),
            singular: single_row.columns.get(15).to_owned_string(),
            plural: single_row.columns.get(17).to_owned_string(),
            name: single_row.columns.get(23).to_owned_string(),
        })
    }
}

