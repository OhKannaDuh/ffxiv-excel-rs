/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct Lobby {
    pub row_id: u32,
    pub _type: u32,
    pub param: u32,
    pub link: u32,
    pub text: String,
}

impl Sheet for Lobby {
    const SHEET_NAME: &'static str = "Lobby";
}

impl FromExcelRow for Lobby {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            _type: single_row.columns.get(0).to_u32(),
            param: single_row.columns.get(1).to_u32(),
            link: single_row.columns.get(2).to_u32(),
            text: single_row.columns.get(3).to_owned_string(),
        })
    }
}

