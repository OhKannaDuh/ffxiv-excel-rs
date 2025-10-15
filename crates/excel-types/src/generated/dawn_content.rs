/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct DawnContent {
    pub row_id: u32,
    pub content_id: u32,
    pub content: RowRef<ContentFinderCondition>,
    pub exp_below_ex_max_lvl: bool,
    pub exp_above_ex_max_lvl: bool,
}

impl Sheet for DawnContent {
    const SHEET_NAME: &'static str = "DawnContent";
}

impl FromExcelRow for DawnContent {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            content_id: single_row.columns.get(0).to_u32(),
            content: RowRef::<ContentFinderCondition>::from(single_row.columns.get(0).to_u32()),
            exp_below_ex_max_lvl: single_row.columns.get(4).to_bit(2),
            exp_above_ex_max_lvl: single_row.columns.get(5).to_bit(3),
        })
    }
}

