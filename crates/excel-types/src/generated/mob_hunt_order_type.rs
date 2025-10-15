/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct MobHuntOrderType {
    pub row_id: u32,
    pub _type: u8,
    pub quest_id: u32,
    pub quest: RowRef<Quest>,
    pub event_item_id: u32,
    pub event_item: RowRef<EventItem>,
    pub order_start_id: u32,
    pub order_start: RowRef<MobHuntOrder>,
    pub order_amount: u8,
}

impl Sheet for MobHuntOrderType {
    const SHEET_NAME: &'static str = "MobHuntOrderType";
}

impl FromExcelRow for MobHuntOrderType {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            _type: single_row.columns.get(0).to_u8(),
            quest_id: single_row.columns.get(1).to_u32(),
            quest: RowRef::<Quest>::from(single_row.columns.get(1).to_u32()),
            event_item_id: single_row.columns.get(2).to_u32(),
            event_item: RowRef::<EventItem>::from(single_row.columns.get(2).to_u32()),
            order_start_id: single_row.columns.get(3).to_u32(),
            order_start: RowRef::<MobHuntOrder>::from(single_row.columns.get(3).to_u32()),
            order_amount: single_row.columns.get(4).to_u8(),
        })
    }
}

