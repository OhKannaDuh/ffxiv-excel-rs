/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct GCScripShopCategory {
    pub row_id: u32,
    pub grand_company_id: u32,
    pub grand_company: RowRef<GrandCompany>,
    pub tier: i8,
    pub sub_category: i8,
}

impl Sheet for GCScripShopCategory {
    const SHEET_NAME: &'static str = "GCScripShopCategory";
}

impl FromExcelRow for GCScripShopCategory {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            grand_company_id: single_row.columns.get(0).to_u32(),
            grand_company: RowRef::<GrandCompany>::from(single_row.columns.get(0).to_u32()),
            tier: single_row.columns.get(1).to_i8(),
            sub_category: single_row.columns.get(2).to_i8(),
        })
    }
}

