/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct GCScripShopItem {
    pub row_id: u32,
    pub item_id: u32,
    pub item: RowRef<Item>,
    pub required_grand_company_rank_id: u32,
    pub required_grand_company_rank: RowRef<GrandCompanyRank>,
    pub cost_gc_seals: u32,
    pub sort_key: u8,
}

impl Sheet for GCScripShopItem {
    const SHEET_NAME: &'static str = "GCScripShopItem";
}

impl FromExcelRow for GCScripShopItem {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            item_id: single_row.columns.get(0).to_u32(),
            item: RowRef::<Item>::from(single_row.columns.get(0).to_u32()),
            required_grand_company_rank_id: single_row.columns.get(1).to_u32(),
            required_grand_company_rank: RowRef::<GrandCompanyRank>::from(single_row.columns.get(1).to_u32()),
            cost_gc_seals: single_row.columns.get(2).to_u32(),
            sort_key: single_row.columns.get(3).to_u8(),
        })
    }
}

