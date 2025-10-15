/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct CompanyCraftPart {
    pub row_id: u32,
    pub company_craft_type_id: u32,
    pub company_craft_type: RowRef<CompanyCraftType>,
}

impl Sheet for CompanyCraftPart {
    const SHEET_NAME: &'static str = "CompanyCraftPart";
}

impl FromExcelRow for CompanyCraftPart {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            company_craft_type_id: single_row.columns.get(1).to_u32(),
            company_craft_type: RowRef::<CompanyCraftType>::from(single_row.columns.get(1).to_u32()),
        })
    }
}

