/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct DisposalShopItem {
    pub row_id: u32,
    pub item_disposed_id: u32,
    pub item_disposed: RowRef<Item>,
    pub item_received_id: u32,
    pub item_received: RowRef<Item>,
    pub quantity_received: u32,
}

impl Sheet for DisposalShopItem {
    const SHEET_NAME: &'static str = "DisposalShopItem";
}

impl FromExcelRow for DisposalShopItem {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            item_disposed_id: single_row.columns.get(0).to_u32(),
            item_disposed: RowRef::<Item>::from(single_row.columns.get(0).to_u32()),
            item_received_id: single_row.columns.get(2).to_u32(),
            item_received: RowRef::<Item>::from(single_row.columns.get(2).to_u32()),
            quantity_received: single_row.columns.get(4).to_u32(),
        })
    }
}

