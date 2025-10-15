/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct GimmickRect {
    pub row_id: u32,
    pub layout_id: u32,
    pub trigger_in: u8,
    pub param0: u32,
    pub trigger_out: u8,
    pub param1: u32,
}

impl Sheet for GimmickRect {
    const SHEET_NAME: &'static str = "GimmickRect";
}

impl FromExcelRow for GimmickRect {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            layout_id: single_row.columns.get(0).to_u32(),
            trigger_in: single_row.columns.get(1).to_u8(),
            param0: single_row.columns.get(2).to_u32(),
            trigger_out: single_row.columns.get(6).to_u8(),
            param1: single_row.columns.get(7).to_u32(),
        })
    }
}

