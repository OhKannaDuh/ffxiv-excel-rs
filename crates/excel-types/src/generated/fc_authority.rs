/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct FCAuthority {
    pub row_id: u32,
    pub name: String,
    pub fc_authority_category_id: u32,
    pub fc_authority_category: RowRef<FCAuthorityCategory>,
}

impl Sheet for FCAuthority {
    const SHEET_NAME: &'static str = "FCAuthority";
}

impl FromExcelRow for FCAuthority {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            name: single_row.columns.get(0).to_owned_string(),
            fc_authority_category_id: single_row.columns.get(1).to_u32(),
            fc_authority_category: RowRef::<FCAuthorityCategory>::from(single_row.columns.get(1).to_u32()),
        })
    }
}

