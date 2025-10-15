/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct IKDContentBonus {
    pub row_id: u32,
    pub objective: String,
    pub requirement: String,
    pub image_id: u32,
    pub order: u8,
}

impl Sheet for IKDContentBonus {
    const SHEET_NAME: &'static str = "IKDContentBonus";
}

impl FromExcelRow for IKDContentBonus {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            objective: single_row.columns.get(0).to_owned_string(),
            requirement: single_row.columns.get(1).to_owned_string(),
            image_id: single_row.columns.get(3).to_u32(),
            order: single_row.columns.get(4).to_u8(),
        })
    }
}

