/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct DisposalShop {
    pub row_id: u32,
    pub shop_name: String,
}

impl Sheet for DisposalShop {
    const SHEET_NAME: &'static str = "DisposalShop";
}

impl FromExcelRow for DisposalShop {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            shop_name: single_row.columns.get(0).to_owned_string(),
        })
    }
}

