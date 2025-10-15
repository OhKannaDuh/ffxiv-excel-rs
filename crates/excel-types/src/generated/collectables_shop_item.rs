/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct CollectablesShopItem {
    pub row_id: u32,
    pub item_id: u32,
    pub item: RowRef<Item>,
    pub collectables_shop_item_group_id: u32,
    pub collectables_shop_item_group: RowRef<CollectablesShopItemGroup>,
    pub level_min: u16,
    pub level_max: u16,
    pub stars: u8,
    pub key: u8,
    pub collectables_shop_refine_id: u32,
    pub collectables_shop_refine: RowRef<CollectablesShopRefine>,
    pub collectables_shop_reward_scrip_id: u32,
    pub collectables_shop_reward_scrip: RowRef<CollectablesShopRewardScrip>,
}

impl Sheet for CollectablesShopItem {
    const SHEET_NAME: &'static str = "CollectablesShopItem";
}

impl FromExcelRow for CollectablesShopItem {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            item_id: single_row.columns.get(0).to_u32(),
            item: RowRef::<Item>::from(single_row.columns.get(0).to_u32()),
            collectables_shop_item_group_id: single_row.columns.get(1).to_u32(),
            collectables_shop_item_group: RowRef::<CollectablesShopItemGroup>::from(single_row.columns.get(1).to_u32()),
            level_min: single_row.columns.get(2).to_u16(),
            level_max: single_row.columns.get(4).to_u16(),
            stars: single_row.columns.get(5).to_u8(),
            key: single_row.columns.get(6).to_u8(),
            collectables_shop_refine_id: single_row.columns.get(7).to_u32(),
            collectables_shop_refine: RowRef::<CollectablesShopRefine>::from(single_row.columns.get(7).to_u32()),
            collectables_shop_reward_scrip_id: single_row.columns.get(8).to_u32(),
            collectables_shop_reward_scrip: RowRef::<CollectablesShopRewardScrip>::from(single_row.columns.get(8).to_u32()),
        })
    }
}

