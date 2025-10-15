/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct SecretRecipeBook {
    pub row_id: u32,
    pub item_id: u32,
    pub item: RowRef<Item>,
    pub name: String,
}

impl Sheet for SecretRecipeBook {
    const SHEET_NAME: &'static str = "SecretRecipeBook";
}

impl FromExcelRow for SecretRecipeBook {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            item_id: single_row.columns.get(0).to_u32(),
            item: RowRef::<Item>::from(single_row.columns.get(0).to_u32()),
            name: single_row.columns.get(1).to_owned_string(),
        })
    }
}

