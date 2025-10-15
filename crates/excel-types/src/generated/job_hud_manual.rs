/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct JobHudManual {
    pub row_id: u32,
    pub action_id: u32,
    pub action: RowRef<Action>,
    pub guide_id: u32,
    pub guide: RowRef<Guide>,
}

impl Sheet for JobHudManual {
    const SHEET_NAME: &'static str = "JobHudManual";
}

impl FromExcelRow for JobHudManual {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            action_id: single_row.columns.get(3).to_u32(),
            action: RowRef::<Action>::from(single_row.columns.get(3).to_u32()),
            guide_id: single_row.columns.get(6).to_u32(),
            guide: RowRef::<Guide>::from(single_row.columns.get(6).to_u32()),
        })
    }
}

