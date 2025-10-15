/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct ScenarioType {
    pub row_id: u32,
    pub _type: String,
}

impl Sheet for ScenarioType {
    const SHEET_NAME: &'static str = "ScenarioType";
}

impl FromExcelRow for ScenarioType {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            _type: single_row.columns.get(0).to_owned_string(),
        })
    }
}

