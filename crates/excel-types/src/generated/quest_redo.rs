/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct QuestRedo {
    pub row_id: u32,
    pub final_quest_id: u32,
    pub final_quest: RowRef<Quest>,
    pub chapter_id: u32,
}

impl Sheet for QuestRedo {
    const SHEET_NAME: &'static str = "QuestRedo";
}

impl FromExcelRow for QuestRedo {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            final_quest_id: single_row.columns.get(0).to_u32(),
            final_quest: RowRef::<Quest>::from(single_row.columns.get(0).to_u32()),
            chapter_id: single_row.columns.get(3).to_u32(),
        })
    }
}

