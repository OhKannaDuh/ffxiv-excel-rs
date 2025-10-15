/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct ContentTalkParam {
    pub row_id: u32,
    pub param: bool,
    pub test_action_id: u32,
    pub test_action: RowRef<ActionTimeline>,
}

impl Sheet for ContentTalkParam {
    const SHEET_NAME: &'static str = "ContentTalkParam";
}

impl FromExcelRow for ContentTalkParam {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            param: single_row.columns.get(0).to_bit(0),
            test_action_id: single_row.columns.get(2).to_u32(),
            test_action: RowRef::<ActionTimeline>::from(single_row.columns.get(2).to_u32()),
        })
    }
}

