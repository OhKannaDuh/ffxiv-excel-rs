/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct PreHandler {
    pub row_id: u32,
    pub image_id: u32,
    pub target_id: u32,
    pub target_collectables_shop: RowRef<CollectablesShop>,
    pub target_inclusion_shop: RowRef<InclusionShop>,
    pub target_gil_shop: RowRef<GilShop>,
    pub target_special_shop: RowRef<SpecialShop>,
    pub target_description: RowRef<Description>,
    pub unlock_quest_id: u32,
    pub unlock_quest: RowRef<Quest>,
    pub accept_message_id: u32,
    pub accept_message: RowRef<DefaultTalk>,
    pub deny_message_id: u32,
    pub deny_message: RowRef<DefaultTalk>,
}

impl Sheet for PreHandler {
    const SHEET_NAME: &'static str = "PreHandler";
}

impl FromExcelRow for PreHandler {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            image_id: single_row.columns.get(1).to_u32(),
            target_id: single_row.columns.get(2).to_u32(),
            target_collectables_shop: RowRef::<CollectablesShop>::from(single_row.columns.get(2).to_u32()),
            target_inclusion_shop: RowRef::<InclusionShop>::from(single_row.columns.get(2).to_u32()),
            target_gil_shop: RowRef::<GilShop>::from(single_row.columns.get(2).to_u32()),
            target_special_shop: RowRef::<SpecialShop>::from(single_row.columns.get(2).to_u32()),
            target_description: RowRef::<Description>::from(single_row.columns.get(2).to_u32()),
            unlock_quest_id: single_row.columns.get(3).to_u32(),
            unlock_quest: RowRef::<Quest>::from(single_row.columns.get(3).to_u32()),
            accept_message_id: single_row.columns.get(4).to_u32(),
            accept_message: RowRef::<DefaultTalk>::from(single_row.columns.get(4).to_u32()),
            deny_message_id: single_row.columns.get(5).to_u32(),
            deny_message: RowRef::<DefaultTalk>::from(single_row.columns.get(5).to_u32()),
        })
    }
}

