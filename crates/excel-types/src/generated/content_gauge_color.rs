/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct ContentGaugeColor {
    pub row_id: u32,
    pub android_color_1: u32,
    pub android_color_2: u32,
    pub android_color_3: u32,
}

impl Sheet for ContentGaugeColor {
    const SHEET_NAME: &'static str = "ContentGaugeColor";
}

impl FromExcelRow for ContentGaugeColor {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            android_color_1: single_row.columns.get(0).to_u32(),
            android_color_2: single_row.columns.get(1).to_u32(),
            android_color_3: single_row.columns.get(2).to_u32(),
        })
    }
}

