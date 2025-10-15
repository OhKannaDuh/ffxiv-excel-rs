/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct TextCommand {
    pub row_id: u32,
    pub command: String,
    pub short_command: String,
    pub description: String,
    pub alias: String,
    pub short_alias: String,
    pub param_id: u32,
    pub param: RowRef<TextCommandParam>,
}

impl Sheet for TextCommand {
    const SHEET_NAME: &'static str = "TextCommand";
}

impl FromExcelRow for TextCommand {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            command: single_row.columns.get(5).to_owned_string(),
            short_command: single_row.columns.get(6).to_owned_string(),
            description: single_row.columns.get(7).to_owned_string(),
            alias: single_row.columns.get(8).to_owned_string(),
            short_alias: single_row.columns.get(9).to_owned_string(),
            param_id: single_row.columns.get(10).to_u32(),
            param: RowRef::<TextCommandParam>::from(single_row.columns.get(10).to_u32()),
        })
    }
}

