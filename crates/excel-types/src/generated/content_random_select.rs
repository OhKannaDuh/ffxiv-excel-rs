/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct ContentRandomSelect {
    pub row_id: u32,
    pub name_id: u32,
    pub name: RowRef<ContentFinderCondition>,
}

impl Sheet for ContentRandomSelect {
    const SHEET_NAME: &'static str = "ContentRandomSelect";
}

impl FromExcelRow for ContentRandomSelect {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            name_id: single_row.columns.get(0).to_u32(),
            name: RowRef::<ContentFinderCondition>::from(single_row.columns.get(0).to_u32()),
        })
    }
}

