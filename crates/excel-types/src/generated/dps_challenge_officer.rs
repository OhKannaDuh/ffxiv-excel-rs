/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct DpsChallengeOfficer {
    pub row_id: u32,
    pub unlock_quest_id: u32,
    pub unlock_quest: RowRef<Quest>,
}

impl Sheet for DpsChallengeOfficer {
    const SHEET_NAME: &'static str = "DpsChallengeOfficer";
}

impl FromExcelRow for DpsChallengeOfficer {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            unlock_quest_id: single_row.columns.get(0).to_u32(),
            unlock_quest: RowRef::<Quest>::from(single_row.columns.get(0).to_u32()),
        })
    }
}

