/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct GcArmyExpeditionType {
    pub row_id: u32,
    pub name: String,
}

impl Sheet for GcArmyExpeditionType {
    const SHEET_NAME: &'static str = "GcArmyExpeditionType";
}

impl FromExcelRow for GcArmyExpeditionType {
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

