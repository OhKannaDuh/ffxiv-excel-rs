/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct CompanyLeve {
    pub row_id: u32,
    pub rule_id: u32,
    pub rule: RowRef<CompanyLeveRule>,
    pub rule_param: u8,
}

impl Sheet for CompanyLeve {
    const SHEET_NAME: &'static str = "CompanyLeve";
}

impl FromExcelRow for CompanyLeve {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            rule_id: single_row.columns.get(176).to_u32(),
            rule: RowRef::<CompanyLeveRule>::from(single_row.columns.get(176).to_u32()),
            rule_param: single_row.columns.get(177).to_u8(),
        })
    }
}

