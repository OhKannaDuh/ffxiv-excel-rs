/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct SkyIsland2Mission {
    pub row_id: u32,
    pub item_1_id: u32,
    pub item_1: RowRef<EventItem>,
    pub item_2_id: u32,
    pub item_2: RowRef<EventItem>,
    pub place_name_id: u32,
    pub place_name: RowRef<PlaceName>,
    pub objective_1_id: u32,
    pub objective_1: RowRef<SkyIsland2MissionDetail>,
    pub pop_range_0: u32,
    pub required_amount_1: u8,
    pub objective2_id: u32,
    pub objective2: RowRef<SkyIsland2MissionDetail>,
    pub pop_range_1: u32,
    pub required_amount_2: u8,
    pub objective3_id: u32,
    pub objective3: RowRef<SkyIsland2MissionDetail>,
    pub pop_range_2: u32,
    pub image_id: u32,
}

impl Sheet for SkyIsland2Mission {
    const SHEET_NAME: &'static str = "SkyIsland2Mission";
}

impl FromExcelRow for SkyIsland2Mission {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            item_1_id: single_row.columns.get(0).to_u32(),
            item_1: RowRef::<EventItem>::from(single_row.columns.get(0).to_u32()),
            item_2_id: single_row.columns.get(1).to_u32(),
            item_2: RowRef::<EventItem>::from(single_row.columns.get(1).to_u32()),
            place_name_id: single_row.columns.get(2).to_u32(),
            place_name: RowRef::<PlaceName>::from(single_row.columns.get(2).to_u32()),
            objective_1_id: single_row.columns.get(4).to_u32(),
            objective_1: RowRef::<SkyIsland2MissionDetail>::from(single_row.columns.get(4).to_u32()),
            pop_range_0: single_row.columns.get(5).to_u32(),
            required_amount_1: single_row.columns.get(6).to_u8(),
            objective2_id: single_row.columns.get(9).to_u32(),
            objective2: RowRef::<SkyIsland2MissionDetail>::from(single_row.columns.get(9).to_u32()),
            pop_range_1: single_row.columns.get(10).to_u32(),
            required_amount_2: single_row.columns.get(11).to_u8(),
            objective3_id: single_row.columns.get(14).to_u32(),
            objective3: RowRef::<SkyIsland2MissionDetail>::from(single_row.columns.get(14).to_u32()),
            pop_range_2: single_row.columns.get(15).to_u32(),
            image_id: single_row.columns.get(20).to_u32(),
        })
    }
}

