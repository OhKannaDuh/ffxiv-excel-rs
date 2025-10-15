/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct MobHuntOrder {
    pub row_id: u32,
    pub target_id: u32,
    pub target: RowRef<MobHuntTarget>,
    pub needed_kills: u8,
    pub _type: u8,
    pub rank: u8,
    pub mob_hunt_reward_id: u32,
    pub mob_hunt_reward: RowRef<MobHuntReward>,
}

impl Sheet for MobHuntOrder {
    const SHEET_NAME: &'static str = "MobHuntOrder";
}

impl FromExcelRow for MobHuntOrder {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            target_id: single_row.columns.get(0).to_u32(),
            target: RowRef::<MobHuntTarget>::from(single_row.columns.get(0).to_u32()),
            needed_kills: single_row.columns.get(1).to_u8(),
            _type: single_row.columns.get(2).to_u8(),
            rank: single_row.columns.get(3).to_u8(),
            mob_hunt_reward_id: single_row.columns.get(4).to_u32(),
            mob_hunt_reward: RowRef::<MobHuntReward>::from(single_row.columns.get(4).to_u32()),
        })
    }
}

