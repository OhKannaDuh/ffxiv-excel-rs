/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct CutSceneIncompQuest {
    pub row_id: u32,
    pub quest_id: u32,
    pub quest: RowRef<Quest>,
}

impl Sheet for CutSceneIncompQuest {
    const SHEET_NAME: &'static str = "CutSceneIncompQuest";
}

impl FromExcelRow for CutSceneIncompQuest {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            quest_id: single_row.columns.get(0).to_u32(),
            quest: RowRef::<Quest>::from(single_row.columns.get(0).to_u32()),
        })
    }
}

