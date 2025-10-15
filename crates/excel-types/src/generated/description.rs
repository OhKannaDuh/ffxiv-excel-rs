/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct Description {
    pub row_id: u32,
    pub quest_id: u32,
    pub quest: RowRef<Quest>,
    pub text_long: String,
    pub text_short: String,
    pub text_commentary: String,
    pub section_id: u32,
    pub section: RowRef<DescriptionSection>,
}

impl Sheet for Description {
    const SHEET_NAME: &'static str = "Description";
}

impl FromExcelRow for Description {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            quest_id: single_row.columns.get(1).to_u32(),
            quest: RowRef::<Quest>::from(single_row.columns.get(1).to_u32()),
            text_long: single_row.columns.get(2).to_owned_string(),
            text_short: single_row.columns.get(3).to_owned_string(),
            text_commentary: single_row.columns.get(4).to_owned_string(),
            section_id: single_row.columns.get(6).to_u32(),
            section: RowRef::<DescriptionSection>::from(single_row.columns.get(6).to_u32()),
        })
    }
}

