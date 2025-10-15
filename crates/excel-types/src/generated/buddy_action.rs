/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct BuddyAction {
    pub row_id: u32,
    pub name: String,
    pub description: String,
    pub icon_id: u32,
    pub icon_status_id: u32,
    pub reward: u16,
    pub sort: u8,
}

impl Sheet for BuddyAction {
    const SHEET_NAME: &'static str = "BuddyAction";
}

impl FromExcelRow for BuddyAction {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            name: single_row.columns.get(0).to_owned_string(),
            description: single_row.columns.get(1).to_owned_string(),
            icon_id: single_row.columns.get(2).to_u32(),
            icon_status_id: single_row.columns.get(3).to_u32(),
            reward: single_row.columns.get(4).to_u16(),
            sort: single_row.columns.get(5).to_u8(),
        })
    }
}

