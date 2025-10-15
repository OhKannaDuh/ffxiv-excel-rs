/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct GilShopItem {
    pub row_id: u32,
    pub item_id: u32,
    pub item: RowRef<Item>,
    pub achievement_required_id: u32,
    pub achievement_required: RowRef<Achievement>,
    pub state_required: u16,
    pub patch: u16,
}

impl Sheet for GilShopItem {
    const SHEET_NAME: &'static str = "GilShopItem";
}

impl FromExcelRow for GilShopItem {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            item_id: single_row.columns.get(0).to_u32(),
            item: RowRef::<Item>::from(single_row.columns.get(0).to_u32()),
            achievement_required_id: single_row.columns.get(6).to_u32(),
            achievement_required: RowRef::<Achievement>::from(single_row.columns.get(6).to_u32()),
            state_required: single_row.columns.get(8).to_u16(),
            patch: single_row.columns.get(9).to_u16(),
        })
    }
}

