/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct TreasureHuntRank {
    pub row_id: u32,
    pub icon_id: u32,
    pub item_name_id: u32,
    pub item_name: RowRef<Item>,
    pub key_item_name_id: u32,
    pub key_item_name: RowRef<EventItem>,
    pub instance_map_id: u32,
    pub instance_map: RowRef<EventItem>,
    pub max_party_size: u8,
    pub treasure_hunt_texture: u8,
}

impl Sheet for TreasureHuntRank {
    const SHEET_NAME: &'static str = "TreasureHuntRank";
}

impl FromExcelRow for TreasureHuntRank {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            icon_id: single_row.columns.get(1).to_u32(),
            item_name_id: single_row.columns.get(2).to_u32(),
            item_name: RowRef::<Item>::from(single_row.columns.get(2).to_u32()),
            key_item_name_id: single_row.columns.get(3).to_u32(),
            key_item_name: RowRef::<EventItem>::from(single_row.columns.get(3).to_u32()),
            instance_map_id: single_row.columns.get(4).to_u32(),
            instance_map: RowRef::<EventItem>::from(single_row.columns.get(4).to_u32()),
            max_party_size: single_row.columns.get(5).to_u8(),
            treasure_hunt_texture: single_row.columns.get(6).to_u8(),
        })
    }
}

