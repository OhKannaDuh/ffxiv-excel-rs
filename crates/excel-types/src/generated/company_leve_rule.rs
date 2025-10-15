/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct CompanyLeveRule {
    pub row_id: u32,
    pub _type: String,
    pub objective_id: u32,
    pub objective: RowRef<LeveString>,
    pub help_id: u32,
    pub help: RowRef<LeveString>,
}

impl Sheet for CompanyLeveRule {
    const SHEET_NAME: &'static str = "CompanyLeveRule";
}

impl FromExcelRow for CompanyLeveRule {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            _type: single_row.columns.get(0).to_owned_string(),
            objective_id: single_row.columns.get(1).to_u32(),
            objective: RowRef::<LeveString>::from(single_row.columns.get(1).to_u32()),
            help_id: single_row.columns.get(2).to_u32(),
            help: RowRef::<LeveString>::from(single_row.columns.get(2).to_u32()),
        })
    }
}

