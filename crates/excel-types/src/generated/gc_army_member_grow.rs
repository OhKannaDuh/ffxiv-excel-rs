/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct GcArmyMemberGrow {
    pub row_id: u32,
    pub class_job_id: u32,
    pub class_job: RowRef<ClassJob>,
    pub class_book_id: u32,
    pub class_book: RowRef<Item>,
}

impl Sheet for GcArmyMemberGrow {
    const SHEET_NAME: &'static str = "GcArmyMemberGrow";
}

impl FromExcelRow for GcArmyMemberGrow {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            class_job_id: single_row.columns.get(0).to_u32(),
            class_job: RowRef::<ClassJob>::from(single_row.columns.get(0).to_u32()),
            class_book_id: single_row.columns.get(1).to_u32(),
            class_book: RowRef::<Item>::from(single_row.columns.get(1).to_u32()),
        })
    }
}

