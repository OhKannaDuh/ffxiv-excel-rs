/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct OnlineStatus {
    pub row_id: u32,
    pub list: bool,
    pub priority: u8,
    pub icon_id: u32,
    pub name: String,
}

impl Sheet for OnlineStatus {
    const SHEET_NAME: &'static str = "OnlineStatus";
}

impl FromExcelRow for OnlineStatus {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            list: single_row.columns.get(1).to_bit(1),
            priority: single_row.columns.get(3).to_u8(),
            icon_id: single_row.columns.get(4).to_u32(),
            name: single_row.columns.get(6).to_owned_string(),
        })
    }
}

