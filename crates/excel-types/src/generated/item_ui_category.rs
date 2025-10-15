/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct ItemUICategory {
    pub row_id: u32,
    pub name: String,
    pub icon_id: u32,
    pub order_minor: u8,
    pub order_major: u8,
}

impl Sheet for ItemUICategory {
    const SHEET_NAME: &'static str = "ItemUICategory";
}

impl FromExcelRow for ItemUICategory {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            name: single_row.columns.get(0).to_owned_string(),
            icon_id: single_row.columns.get(1).to_u32(),
            order_minor: single_row.columns.get(2).to_u8(),
            order_major: single_row.columns.get(3).to_u8(),
        })
    }
}

