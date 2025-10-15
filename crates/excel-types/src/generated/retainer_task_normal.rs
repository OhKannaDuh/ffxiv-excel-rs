/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct RetainerTaskNormal {
    pub row_id: u32,
    pub item_id: u32,
    pub item: RowRef<Item>,
    pub gathering_log_id: u32,
    pub gathering_log: RowRef<GatheringItem>,
    pub fishing_log_id: u32,
    pub fishing_log_spearfishing_item: RowRef<SpearfishingItem>,
    pub fishing_log_fish_parameter: RowRef<FishParameter>,
}

impl Sheet for RetainerTaskNormal {
    const SHEET_NAME: &'static str = "RetainerTaskNormal";
}

impl FromExcelRow for RetainerTaskNormal {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            item_id: single_row.columns.get(0).to_u32(),
            item: RowRef::<Item>::from(single_row.columns.get(0).to_u32()),
            gathering_log_id: single_row.columns.get(6).to_u32(),
            gathering_log: RowRef::<GatheringItem>::from(single_row.columns.get(6).to_u32()),
            fishing_log_id: single_row.columns.get(7).to_u32(),
            fishing_log_spearfishing_item: RowRef::<SpearfishingItem>::from(single_row.columns.get(7).to_u32()),
            fishing_log_fish_parameter: RowRef::<FishParameter>::from(single_row.columns.get(7).to_u32()),
        })
    }
}

