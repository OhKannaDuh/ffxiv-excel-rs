/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct InclusionShopSeries {
    pub row_id: u32,
    pub special_shop_id: u32,
    pub special_shop: RowRef<SpecialShop>,
}

impl Sheet for InclusionShopSeries {
    const SHEET_NAME: &'static str = "InclusionShopSeries";
}

impl FromExcelRow for InclusionShopSeries {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            special_shop_id: single_row.columns.get(0).to_u32(),
            special_shop: RowRef::<SpecialShop>::from(single_row.columns.get(0).to_u32()),
        })
    }
}

