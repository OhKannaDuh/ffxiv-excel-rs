/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct GatheringItem {
    pub row_id: u32,
    pub item_id: u32,
    pub item: RowRef<Item>,
    pub item_event_item: RowRef<EventItem>,
    pub gathering_item_level_id: u32,
    pub gathering_item_level: RowRef<GatheringItemLevelConvertTable>,
    pub quest_id: u32,
    pub quest: RowRef<Quest>,
    pub is_hidden: u32,
}

impl Sheet for GatheringItem {
    const SHEET_NAME: &'static str = "GatheringItem";
}

impl FromExcelRow for GatheringItem {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            item_id: single_row.columns.get(0).to_u32(),
            item: RowRef::<Item>::from(single_row.columns.get(0).to_u32()),
            item_event_item: RowRef::<EventItem>::from(single_row.columns.get(0).to_u32()),
            gathering_item_level_id: single_row.columns.get(1).to_u32(),
            gathering_item_level: RowRef::<GatheringItemLevelConvertTable>::from(single_row.columns.get(1).to_u32()),
            quest_id: single_row.columns.get(3).to_u32(),
            quest: RowRef::<Quest>::from(single_row.columns.get(3).to_u32()),
            is_hidden: single_row.columns.get(5).to_u32(),
        })
    }
}

