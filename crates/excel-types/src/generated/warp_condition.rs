/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct WarpCondition {
    pub row_id: u32,
    pub gil: u16,
    pub complete_param: u8,
    pub required_quest_1_id: u32,
    pub required_quest_1: RowRef<Quest>,
    pub required_quest_2_id: u32,
    pub required_quest_2: RowRef<Quest>,
    pub required_quest_3_id: u32,
    pub required_quest_3: RowRef<Quest>,
    pub required_quest_4_id: u32,
    pub required_quest_4: RowRef<Quest>,
    pub quest_reward: u16,
    pub class_level: u16,
}

impl Sheet for WarpCondition {
    const SHEET_NAME: &'static str = "WarpCondition";
}

impl FromExcelRow for WarpCondition {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            gil: single_row.columns.get(0).to_u16(),
            complete_param: single_row.columns.get(1).to_u8(),
            required_quest_1_id: single_row.columns.get(2).to_u32(),
            required_quest_1: RowRef::<Quest>::from(single_row.columns.get(2).to_u32()),
            required_quest_2_id: single_row.columns.get(3).to_u32(),
            required_quest_2: RowRef::<Quest>::from(single_row.columns.get(3).to_u32()),
            required_quest_3_id: single_row.columns.get(4).to_u32(),
            required_quest_3: RowRef::<Quest>::from(single_row.columns.get(4).to_u32()),
            required_quest_4_id: single_row.columns.get(5).to_u32(),
            required_quest_4: RowRef::<Quest>::from(single_row.columns.get(5).to_u32()),
            quest_reward: single_row.columns.get(6).to_u16(),
            class_level: single_row.columns.get(7).to_u16(),
        })
    }
}

