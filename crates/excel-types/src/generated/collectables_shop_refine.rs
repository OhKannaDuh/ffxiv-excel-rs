/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct CollectablesShopRefine {
    pub row_id: u32,
    pub low_collectability: u16,
    pub mid_collectability: u16,
    pub high_collectability: u16,
}

impl Sheet for CollectablesShopRefine {
    const SHEET_NAME: &'static str = "CollectablesShopRefine";
}

impl FromExcelRow for CollectablesShopRefine {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            low_collectability: single_row.columns.get(0).to_u16(),
            mid_collectability: single_row.columns.get(1).to_u16(),
            high_collectability: single_row.columns.get(2).to_u16(),
        })
    }
}

