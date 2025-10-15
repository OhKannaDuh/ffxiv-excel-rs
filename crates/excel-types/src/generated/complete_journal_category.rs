/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct CompleteJournalCategory {
    pub row_id: u32,
    pub first_quest_id: u32,
    pub first_quest: RowRef<CompleteJournal>,
    pub last_quest_id: u32,
    pub last_quest: RowRef<CompleteJournal>,
}

impl Sheet for CompleteJournalCategory {
    const SHEET_NAME: &'static str = "CompleteJournalCategory";
}

impl FromExcelRow for CompleteJournalCategory {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            first_quest_id: single_row.columns.get(0).to_u32(),
            first_quest: RowRef::<CompleteJournal>::from(single_row.columns.get(0).to_u32()),
            last_quest_id: single_row.columns.get(1).to_u32(),
            last_quest: RowRef::<CompleteJournal>::from(single_row.columns.get(1).to_u32()),
        })
    }
}

