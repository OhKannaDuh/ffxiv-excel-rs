/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct QuestAcceptAdditionCondition {
    pub row_id: u32,
    pub requirement_0_id: u32,
    pub requirement_0: RowRef<Quest>,
    pub requirement_1_id: u32,
    pub requirement_1: RowRef<Quest>,
}

impl Sheet for QuestAcceptAdditionCondition {
    const SHEET_NAME: &'static str = "QuestAcceptAdditionCondition";
}

impl FromExcelRow for QuestAcceptAdditionCondition {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            requirement_0_id: single_row.columns.get(0).to_u32(),
            requirement_0: RowRef::<Quest>::from(single_row.columns.get(0).to_u32()),
            requirement_1_id: single_row.columns.get(1).to_u32(),
            requirement_1: RowRef::<Quest>::from(single_row.columns.get(1).to_u32()),
        })
    }
}

