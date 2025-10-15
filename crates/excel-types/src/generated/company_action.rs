/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct CompanyAction {
    pub row_id: u32,
    pub name: String,
    pub description: String,
    pub icon_id: u32,
    pub fc_rank_id: u32,
    pub fc_rank: RowRef<FCRank>,
    pub cost: u32,
    pub order: u8,
    pub purchasable: bool,
}

impl Sheet for CompanyAction {
    const SHEET_NAME: &'static str = "CompanyAction";
}

impl FromExcelRow for CompanyAction {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            name: single_row.columns.get(0).to_owned_string(),
            description: single_row.columns.get(1).to_owned_string(),
            icon_id: single_row.columns.get(2).to_u32(),
            fc_rank_id: single_row.columns.get(3).to_u32(),
            fc_rank: RowRef::<FCRank>::from(single_row.columns.get(3).to_u32()),
            cost: single_row.columns.get(4).to_u32(),
            order: single_row.columns.get(5).to_u8(),
            purchasable: single_row.columns.get(6).to_bit(0),
        })
    }
}

