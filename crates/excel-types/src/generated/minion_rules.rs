/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct MinionRules {
    pub row_id: u32,
    pub rule: String,
    pub description: String,
}

impl Sheet for MinionRules {
    const SHEET_NAME: &'static str = "MinionRules";
}

impl FromExcelRow for MinionRules {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            rule: single_row.columns.get(0).to_owned_string(),
            description: single_row.columns.get(1).to_owned_string(),
        })
    }
}

