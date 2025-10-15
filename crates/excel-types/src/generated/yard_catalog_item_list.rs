/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct YardCatalogItemList {
    pub row_id: u32,
    pub category_id: u32,
    pub category: RowRef<YardCatalogCategory>,
    pub item_id: u32,
    pub item: RowRef<Item>,
    pub patch: u16,
}

impl Sheet for YardCatalogItemList {
    const SHEET_NAME: &'static str = "YardCatalogItemList";
}

impl FromExcelRow for YardCatalogItemList {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            category_id: single_row.columns.get(0).to_u32(),
            category: RowRef::<YardCatalogCategory>::from(single_row.columns.get(0).to_u32()),
            item_id: single_row.columns.get(1).to_u32(),
            item: RowRef::<Item>::from(single_row.columns.get(1).to_u32()),
            patch: single_row.columns.get(2).to_u16(),
        })
    }
}

