/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct HWDSharedGroup {
    pub row_id: u32,
    pub lgb_shared_group: u32,
    pub param_id: u32,
    pub param: RowRef<HWDSharedGroupControlParam>,
}

impl Sheet for HWDSharedGroup {
    const SHEET_NAME: &'static str = "HWDSharedGroup";
}

impl FromExcelRow for HWDSharedGroup {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            lgb_shared_group: single_row.columns.get(0).to_u32(),
            param_id: single_row.columns.get(1).to_u32(),
            param: RowRef::<HWDSharedGroupControlParam>::from(single_row.columns.get(1).to_u32()),
        })
    }
}

