/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct InclusionShopCategory {
    pub row_id: u32,
    pub name: String,
    pub class_job_category_id: u32,
    pub class_job_category: RowRef<ClassJobCategory>,
    pub inclusion_shop_series_id: u32,
    pub inclusion_shop_series: RowRef<InclusionShopSeries>,
}

impl Sheet for InclusionShopCategory {
    const SHEET_NAME: &'static str = "InclusionShopCategory";
}

impl FromExcelRow for InclusionShopCategory {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            name: single_row.columns.get(0).to_owned_string(),
            class_job_category_id: single_row.columns.get(1).to_u32(),
            class_job_category: RowRef::<ClassJobCategory>::from(single_row.columns.get(1).to_u32()),
            inclusion_shop_series_id: single_row.columns.get(2).to_u32(),
            inclusion_shop_series: RowRef::<InclusionShopSeries>::from(single_row.columns.get(2).to_u32()),
        })
    }
}

