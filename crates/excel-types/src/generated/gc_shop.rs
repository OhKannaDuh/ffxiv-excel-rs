/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct GCShop {
    pub row_id: u32,
    pub grand_company_id: u32,
    pub grand_company: RowRef<GrandCompany>,
}

impl Sheet for GCShop {
    const SHEET_NAME: &'static str = "GCShop";
}

impl FromExcelRow for GCShop {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            grand_company_id: single_row.columns.get(0).to_u32(),
            grand_company: RowRef::<GrandCompany>::from(single_row.columns.get(0).to_u32()),
        })
    }
}

