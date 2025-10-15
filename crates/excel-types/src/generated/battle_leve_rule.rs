/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct BattleLeveRule {
    pub row_id: u32,
    pub rule: String,
}

impl Sheet for BattleLeveRule {
    const SHEET_NAME: &'static str = "BattleLeveRule";
}

impl FromExcelRow for BattleLeveRule {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            rule: single_row.columns.get(0).to_owned_string(),
        })
    }
}

