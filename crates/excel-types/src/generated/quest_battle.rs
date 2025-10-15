/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct QuestBattle {
    pub row_id: u32,
    pub quest_id: u32,
    pub quest_array_event_handler: RowRef<ArrayEventHandler>,
    pub quest: RowRef<Quest>,
    pub quest_battle_scene: u8,
    pub time_limit: u16,
    pub level_sync: u16,
}

impl Sheet for QuestBattle {
    const SHEET_NAME: &'static str = "QuestBattle";
}

impl FromExcelRow for QuestBattle {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            quest_id: single_row.columns.get(0).to_u32(),
            quest_array_event_handler: RowRef::<ArrayEventHandler>::from(single_row.columns.get(0).to_u32()),
            quest: RowRef::<Quest>::from(single_row.columns.get(0).to_u32()),
            quest_battle_scene: single_row.columns.get(1).to_u8(),
            time_limit: single_row.columns.get(2).to_u16(),
            level_sync: single_row.columns.get(3).to_u16(),
        })
    }
}

