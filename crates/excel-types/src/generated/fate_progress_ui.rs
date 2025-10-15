/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct FateProgressUI {
    pub row_id: u32,
    pub location_id: u32,
    pub location: RowRef<TerritoryType>,
    pub req_fates_to_rank2: u8,
    pub req_fates_to_rank3: u8,
    pub req_fates_to_rank4: u8,
    pub display_order: u8,
}

impl Sheet for FateProgressUI {
    const SHEET_NAME: &'static str = "FateProgressUI";
}

impl FromExcelRow for FateProgressUI {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            location_id: single_row.columns.get(0).to_u32(),
            location: RowRef::<TerritoryType>::from(single_row.columns.get(0).to_u32()),
            req_fates_to_rank2: single_row.columns.get(1).to_u8(),
            req_fates_to_rank3: single_row.columns.get(2).to_u8(),
            req_fates_to_rank4: single_row.columns.get(3).to_u8(),
            display_order: single_row.columns.get(5).to_u8(),
        })
    }
}

