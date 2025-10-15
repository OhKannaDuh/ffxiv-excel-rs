/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct EurekaGrowData {
    pub row_id: u32,
    pub base_resistance: u16,
}

impl Sheet for EurekaGrowData {
    const SHEET_NAME: &'static str = "EurekaGrowData";
}

impl FromExcelRow for EurekaGrowData {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            base_resistance: single_row.columns.get(0).to_u16(),
        })
    }
}

