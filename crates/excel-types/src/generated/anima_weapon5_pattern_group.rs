/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct AnimaWeapon5PatternGroup {
    pub row_id: u32,
    pub name: String,
}

impl Sheet for AnimaWeapon5PatternGroup {
    const SHEET_NAME: &'static str = "AnimaWeapon5PatternGroup";
}

impl FromExcelRow for AnimaWeapon5PatternGroup {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            name: single_row.columns.get(0).to_owned_string(),
        })
    }
}

