/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct HousingFurniture {
    pub row_id: u32,
    pub model_key: u16,
    pub housing_item_category: u8,
    pub usage_type: u8,
    pub usage_parameter: u32,
    pub aquarium_tier: u8,
    pub custom_talk_id: u32,
    pub custom_talk: RowRef<CustomTalk>,
    pub item_id: u32,
    pub item: RowRef<Item>,
    pub destroy_on_removal: bool,
}

impl Sheet for HousingFurniture {
    const SHEET_NAME: &'static str = "HousingFurniture";
}

impl FromExcelRow for HousingFurniture {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            model_key: single_row.columns.get(0).to_u16(),
            housing_item_category: single_row.columns.get(1).to_u8(),
            usage_type: single_row.columns.get(2).to_u8(),
            usage_parameter: single_row.columns.get(3).to_u32(),
            aquarium_tier: single_row.columns.get(5).to_u8(),
            custom_talk_id: single_row.columns.get(6).to_u32(),
            custom_talk: RowRef::<CustomTalk>::from(single_row.columns.get(6).to_u32()),
            item_id: single_row.columns.get(7).to_u32(),
            item: RowRef::<Item>::from(single_row.columns.get(7).to_u32()),
            destroy_on_removal: single_row.columns.get(8).to_bit(0),
        })
    }
}

