/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct ActionTimelineReplace {
    pub row_id: u32,
    pub old_id: u32,
    pub old: RowRef<ActionTimeline>,
    pub new_id: u32,
    pub new: RowRef<ActionTimeline>,
}

impl Sheet for ActionTimelineReplace {
    const SHEET_NAME: &'static str = "ActionTimelineReplace";
}

impl FromExcelRow for ActionTimelineReplace {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            old_id: single_row.columns.get(0).to_u32(),
            old: RowRef::<ActionTimeline>::from(single_row.columns.get(0).to_u32()),
            new_id: single_row.columns.get(1).to_u32(),
            new: RowRef::<ActionTimeline>::from(single_row.columns.get(1).to_u32()),
        })
    }
}

