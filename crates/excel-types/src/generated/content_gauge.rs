/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct ContentGauge {
    pub row_id: u32,
    pub name: String,
    pub color_id: u32,
    pub color: RowRef<ContentGaugeColor>,
    pub text_string: String,
}

impl Sheet for ContentGauge {
    const SHEET_NAME: &'static str = "ContentGauge";
}

impl FromExcelRow for ContentGauge {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            name: single_row.columns.get(1).to_owned_string(),
            color_id: single_row.columns.get(2).to_u32(),
            color: RowRef::<ContentGaugeColor>::from(single_row.columns.get(2).to_u32()),
            text_string: single_row.columns.get(4).to_owned_string(),
        })
    }
}

