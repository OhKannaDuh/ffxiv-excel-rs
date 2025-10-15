/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct SwitchTalkVariation {
    pub row_id: u32,
    pub quest_0_id: u32,
    pub quest_0_quest: RowRef<Quest>,
    pub quest_1_id: u32,
    pub quest_1: RowRef<Quest>,
    pub default_talk_id: u32,
    pub default_talk: RowRef<DefaultTalk>,
}

impl Sheet for SwitchTalkVariation {
    const SHEET_NAME: &'static str = "SwitchTalkVariation";
}

impl FromExcelRow for SwitchTalkVariation {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            quest_0_id: single_row.columns.get(0).to_u32(),
            quest_0_quest: RowRef::<Quest>::from(single_row.columns.get(0).to_u32()),
            quest_1_id: single_row.columns.get(1).to_u32(),
            quest_1: RowRef::<Quest>::from(single_row.columns.get(1).to_u32()),
            default_talk_id: single_row.columns.get(3).to_u32(),
            default_talk: RowRef::<DefaultTalk>::from(single_row.columns.get(3).to_u32()),
        })
    }
}

