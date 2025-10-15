/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct FCCrestSymbol {
    pub row_id: u32,
    pub color_num: u8,
    pub fc_right: u8,
}

impl Sheet for FCCrestSymbol {
    const SHEET_NAME: &'static str = "FCCrestSymbol";
}

impl FromExcelRow for FCCrestSymbol {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            color_num: single_row.columns.get(0).to_u8(),
            fc_right: single_row.columns.get(1).to_u8(),
        })
    }
}

