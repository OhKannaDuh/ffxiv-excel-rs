/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct MacroIconRedirectOld {
    pub row_id: u32,
    pub icon_old_id: u32,
    pub icon_new_id: u32,
}

impl Sheet for MacroIconRedirectOld {
    const SHEET_NAME: &'static str = "MacroIconRedirectOld";
}

impl FromExcelRow for MacroIconRedirectOld {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            icon_old_id: single_row.columns.get(0).to_u32(),
            icon_new_id: single_row.columns.get(1).to_u32(),
        })
    }
}

