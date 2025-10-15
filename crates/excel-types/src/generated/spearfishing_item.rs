/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct SpearfishingItem {
    pub row_id: u32,
    pub description: String,
    pub item_id: u32,
    pub item: RowRef<Item>,
    pub gathering_item_level_id: u32,
    pub gathering_item_level: RowRef<GatheringItemLevelConvertTable>,
    pub fishing_record_type_id: u32,
    pub fishing_record_type: RowRef<FishingRecordType>,
    pub territory_type_id: u32,
    pub territory_type: RowRef<TerritoryType>,
    pub is_visible: bool,
}

impl Sheet for SpearfishingItem {
    const SHEET_NAME: &'static str = "SpearfishingItem";
}

impl FromExcelRow for SpearfishingItem {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            description: single_row.columns.get(0).to_owned_string(),
            item_id: single_row.columns.get(1).to_u32(),
            item: RowRef::<Item>::from(single_row.columns.get(1).to_u32()),
            gathering_item_level_id: single_row.columns.get(2).to_u32(),
            gathering_item_level: RowRef::<GatheringItemLevelConvertTable>::from(single_row.columns.get(2).to_u32()),
            fishing_record_type_id: single_row.columns.get(5).to_u32(),
            fishing_record_type: RowRef::<FishingRecordType>::from(single_row.columns.get(5).to_u32()),
            territory_type_id: single_row.columns.get(6).to_u32(),
            territory_type: RowRef::<TerritoryType>::from(single_row.columns.get(6).to_u32()),
            is_visible: single_row.columns.get(8).to_bit(1),
        })
    }
}

