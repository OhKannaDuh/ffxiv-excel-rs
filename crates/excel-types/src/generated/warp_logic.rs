/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct WarpLogic {
    pub row_id: u32,
    pub warp_name: String,
    pub can_skip_cutscene: bool,
    pub question: String,
    pub response_yes: String,
    pub response_no: String,
}

impl Sheet for WarpLogic {
    const SHEET_NAME: &'static str = "WarpLogic";
}

impl FromExcelRow for WarpLogic {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            warp_name: single_row.columns.get(1).to_owned_string(),
            can_skip_cutscene: single_row.columns.get(2).to_bit(0),
            question: single_row.columns.get(23).to_owned_string(),
            response_yes: single_row.columns.get(24).to_owned_string(),
            response_no: single_row.columns.get(25).to_owned_string(),
        })
    }
}

