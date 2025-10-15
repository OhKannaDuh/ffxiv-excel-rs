/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct FishParameter {
    pub row_id: u32,
    pub text: String,
    pub item_id: u32,
    pub item_event_item: RowRef<EventItem>,
    pub item: RowRef<Item>,
    pub gathering_item_level_id: u32,
    pub gathering_item_level: RowRef<GatheringItemLevelConvertTable>,
    pub ocean_stars: u8,
    pub is_hidden: bool,
    pub fishing_record_type_id: u32,
    pub fishing_record_type: RowRef<FishingRecordType>,
    pub fishing_spot_id: u32,
    pub fishing_spot: RowRef<FishingSpot>,
    pub gathering_sub_category_id: u32,
    pub gathering_sub_category: RowRef<GatheringSubCategory>,
    pub is_in_log: bool,
    pub achievement_credit: u32,
}

impl Sheet for FishParameter {
    const SHEET_NAME: &'static str = "FishParameter";
}

impl FromExcelRow for FishParameter {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            text: single_row.columns.get(0).to_owned_string(),
            item_id: single_row.columns.get(4).to_u32(),
            item_event_item: RowRef::<EventItem>::from(single_row.columns.get(4).to_u32()),
            item: RowRef::<Item>::from(single_row.columns.get(4).to_u32()),
            gathering_item_level_id: single_row.columns.get(5).to_u32(),
            gathering_item_level: RowRef::<GatheringItemLevelConvertTable>::from(single_row.columns.get(5).to_u32()),
            ocean_stars: single_row.columns.get(6).to_u8(),
            is_hidden: single_row.columns.get(8).to_bit(0),
            fishing_record_type_id: single_row.columns.get(9).to_u32(),
            fishing_record_type: RowRef::<FishingRecordType>::from(single_row.columns.get(9).to_u32()),
            fishing_spot_id: single_row.columns.get(10).to_u32(),
            fishing_spot: RowRef::<FishingSpot>::from(single_row.columns.get(10).to_u32()),
            gathering_sub_category_id: single_row.columns.get(11).to_u32(),
            gathering_sub_category: RowRef::<GatheringSubCategory>::from(single_row.columns.get(11).to_u32()),
            is_in_log: single_row.columns.get(12).to_bit(1),
            achievement_credit: single_row.columns.get(13).to_u32(),
        })
    }
}

