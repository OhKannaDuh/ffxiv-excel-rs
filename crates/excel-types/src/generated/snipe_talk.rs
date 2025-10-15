/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct SnipeTalk {
    pub row_id: u32,
    pub name_id: u32,
    pub name: RowRef<SnipeTalkName>,
    pub text: String,
}

impl Sheet for SnipeTalk {
    const SHEET_NAME: &'static str = "SnipeTalk";
}

impl FromExcelRow for SnipeTalk {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            name_id: single_row.columns.get(2).to_u32(),
            name: RowRef::<SnipeTalkName>::from(single_row.columns.get(2).to_u32()),
            text: single_row.columns.get(3).to_owned_string(),
        })
    }
}

