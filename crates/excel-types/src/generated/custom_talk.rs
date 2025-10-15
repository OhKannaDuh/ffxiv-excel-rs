/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct CustomTalk {
    pub row_id: u32,
    pub icon_actor_id: u32,
    pub icon_map_id: u32,
    pub name: String,
    pub main_option: String,
    pub sub_option: String,
    pub special_links_id: u32,
    pub special_links_collectables_shop: RowRef<CollectablesShop>,
    pub special_links_special_shop: RowRef<SpecialShop>,
    pub special_links_custom_talk_nest_handlers: RowRef<CustomTalkNestHandlers>,
}

impl Sheet for CustomTalk {
    const SHEET_NAME: &'static str = "CustomTalk";
}

impl FromExcelRow for CustomTalk {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            icon_actor_id: single_row.columns.get(0).to_u32(),
            icon_map_id: single_row.columns.get(1).to_u32(),
            name: single_row.columns.get(2).to_owned_string(),
            main_option: single_row.columns.get(64).to_owned_string(),
            sub_option: single_row.columns.get(65).to_owned_string(),
            special_links_id: single_row.columns.get(75).to_u32(),
            special_links_collectables_shop: RowRef::<CollectablesShop>::from(single_row.columns.get(75).to_u32()),
            special_links_special_shop: RowRef::<SpecialShop>::from(single_row.columns.get(75).to_u32()),
            special_links_custom_talk_nest_handlers: RowRef::<CustomTalkNestHandlers>::from(single_row.columns.get(75).to_u32()),
        })
    }
}

