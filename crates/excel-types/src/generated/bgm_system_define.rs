/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct BGMSystemDefine {
    pub row_id: u32,
    pub define: f32,
}

impl Sheet for BGMSystemDefine {
    const SHEET_NAME: &'static str = "BGMSystemDefine";
}

impl FromExcelRow for BGMSystemDefine {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            define: single_row.columns.get(0).to_f32(),
        })
    }
}

