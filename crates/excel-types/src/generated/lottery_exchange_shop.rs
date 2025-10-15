/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct LotteryExchangeShop {
    pub row_id: u32,
    pub lua: String,
}

impl Sheet for LotteryExchangeShop {
    const SHEET_NAME: &'static str = "LotteryExchangeShop";
}

impl FromExcelRow for LotteryExchangeShop {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            lua: single_row.columns.get(129).to_owned_string(),
        })
    }
}

