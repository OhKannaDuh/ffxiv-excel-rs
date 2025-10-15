/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct ItemSearchCategory {
    pub row_id: u32,
    pub name: String,
    pub icon_id: u32,
    pub category: u8,
    pub order: u8,
    pub class_job_id: u32,
    pub class_job: RowRef<ClassJob>,
}

impl Sheet for ItemSearchCategory {
    const SHEET_NAME: &'static str = "ItemSearchCategory";
}

impl FromExcelRow for ItemSearchCategory {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            name: single_row.columns.get(0).to_owned_string(),
            icon_id: single_row.columns.get(1).to_u32(),
            category: single_row.columns.get(2).to_u8(),
            order: single_row.columns.get(3).to_u8(),
            class_job_id: single_row.columns.get(4).to_u32(),
            class_job: RowRef::<ClassJob>::from(single_row.columns.get(4).to_u32()),
        })
    }
}

