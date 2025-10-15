/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct LogFilter {
    pub row_id: u32,
    pub log_kind: u8,
    pub caster: u16,
    pub target: u16,
    pub category: u8,
    pub display_order: u8,
    pub preset: u8,
    pub name: String,
    pub example: String,
}

impl Sheet for LogFilter {
    const SHEET_NAME: &'static str = "LogFilter";
}

impl FromExcelRow for LogFilter {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            log_kind: single_row.columns.get(0).to_u8(),
            caster: single_row.columns.get(1).to_u16(),
            target: single_row.columns.get(2).to_u16(),
            category: single_row.columns.get(3).to_u8(),
            display_order: single_row.columns.get(4).to_u8(),
            preset: single_row.columns.get(5).to_u8(),
            name: single_row.columns.get(6).to_owned_string(),
            example: single_row.columns.get(7).to_owned_string(),
        })
    }
}

