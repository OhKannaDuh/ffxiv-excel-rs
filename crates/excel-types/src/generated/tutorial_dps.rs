/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct TutorialDPS {
    pub row_id: u32,
    pub objective_id: u32,
    pub objective: RowRef<Tutorial>,
}

impl Sheet for TutorialDPS {
    const SHEET_NAME: &'static str = "TutorialDPS";
}

impl FromExcelRow for TutorialDPS {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            objective_id: single_row.columns.get(0).to_u32(),
            objective: RowRef::<Tutorial>::from(single_row.columns.get(0).to_u32()),
        })
    }
}

