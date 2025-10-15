/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct AdventureExPhase {
    pub row_id: u32,
    pub quest_id: u32,
    pub quest: RowRef<Quest>,
    pub adventure_begin_id: u32,
    pub adventure_begin: RowRef<Adventure>,
    pub adventure_end_id: u32,
    pub adventure_end: RowRef<Adventure>,
    pub expansion_id: u32,
    pub expansion: RowRef<ExVersion>,
}

impl Sheet for AdventureExPhase {
    const SHEET_NAME: &'static str = "AdventureExPhase";
}

impl FromExcelRow for AdventureExPhase {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            quest_id: single_row.columns.get(0).to_u32(),
            quest: RowRef::<Quest>::from(single_row.columns.get(0).to_u32()),
            adventure_begin_id: single_row.columns.get(1).to_u32(),
            adventure_begin: RowRef::<Adventure>::from(single_row.columns.get(1).to_u32()),
            adventure_end_id: single_row.columns.get(2).to_u32(),
            adventure_end: RowRef::<Adventure>::from(single_row.columns.get(2).to_u32()),
            expansion_id: single_row.columns.get(3).to_u32(),
            expansion: RowRef::<ExVersion>::from(single_row.columns.get(3).to_u32()),
        })
    }
}

