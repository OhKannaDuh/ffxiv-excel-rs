/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct GuildleveAssignment {
    pub row_id: u32,
    pub _type: String,
    pub assignment_talk_id: u32,
    pub assignment_talk: RowRef<GuildleveAssignmentTalk>,
}

impl Sheet for GuildleveAssignment {
    const SHEET_NAME: &'static str = "GuildleveAssignment";
}

impl FromExcelRow for GuildleveAssignment {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            _type: single_row.columns.get(0).to_owned_string(),
            assignment_talk_id: single_row.columns.get(2).to_u32(),
            assignment_talk: RowRef::<GuildleveAssignmentTalk>::from(single_row.columns.get(2).to_u32()),
        })
    }
}

