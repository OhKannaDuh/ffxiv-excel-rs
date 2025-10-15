/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct QuestRepeatFlag {
    pub row_id: u32,
    pub quest_id: u32,
    pub quest_array_event_handler: RowRef<ArrayEventHandler>,
    pub quest: RowRef<Quest>,
}

impl Sheet for QuestRepeatFlag {
    const SHEET_NAME: &'static str = "QuestRepeatFlag";
}

impl FromExcelRow for QuestRepeatFlag {
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
        })
    }
}

