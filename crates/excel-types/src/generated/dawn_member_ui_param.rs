/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct DawnMemberUIParam {
    pub row_id: u32,
    pub class_singular: String,
    pub voice_line: u32,
    pub class_plural: String,
}

impl Sheet for DawnMemberUIParam {
    const SHEET_NAME: &'static str = "DawnMemberUIParam";
}

impl FromExcelRow for DawnMemberUIParam {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            class_singular: single_row.columns.get(0).to_owned_string(),
            voice_line: single_row.columns.get(2).to_u32(),
            class_plural: single_row.columns.get(3).to_owned_string(),
        })
    }
}

