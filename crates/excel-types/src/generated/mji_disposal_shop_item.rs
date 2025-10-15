/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct MJIDisposalShopItem {
    pub row_id: u32,
    pub item_id: u32,
    pub item: RowRef<MJIItemPouch>,
    pub currency: u8,
    pub count: u16,
    pub category_id: u32,
    pub category: RowRef<MJIDisposalShopUICategory>,
    pub sort: u8,
}

impl Sheet for MJIDisposalShopItem {
    const SHEET_NAME: &'static str = "MJIDisposalShopItem";
}

impl FromExcelRow for MJIDisposalShopItem {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            item_id: single_row.columns.get(0).to_u32(),
            item: RowRef::<MJIItemPouch>::from(single_row.columns.get(0).to_u32()),
            currency: single_row.columns.get(1).to_u8(),
            count: single_row.columns.get(2).to_u16(),
            category_id: single_row.columns.get(3).to_u32(),
            category: RowRef::<MJIDisposalShopUICategory>::from(single_row.columns.get(3).to_u32()),
            sort: single_row.columns.get(4).to_u8(),
        })
    }
}

