/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct UIColor {
    pub row_id: u32,
    pub ui_foreground: u32,
    pub ui_glow: u32,
}

impl Sheet for UIColor {
    const SHEET_NAME: &'static str = "UIColor";
}

impl FromExcelRow for UIColor {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            ui_foreground: single_row.columns.get(0).to_u32(),
            ui_glow: single_row.columns.get(1).to_u32(),
        })
    }
}

