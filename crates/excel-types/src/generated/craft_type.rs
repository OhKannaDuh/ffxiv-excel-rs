/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct CraftType {
    pub row_id: u32,
    pub main_physical: u8,
    pub sub_physical: u8,
    pub name: String,
}

impl Sheet for CraftType {
    const SHEET_NAME: &'static str = "CraftType";
}

impl FromExcelRow for CraftType {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            main_physical: single_row.columns.get(0).to_u8(),
            sub_physical: single_row.columns.get(1).to_u8(),
            name: single_row.columns.get(2).to_owned_string(),
        })
    }
}

