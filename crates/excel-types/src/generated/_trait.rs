/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct Trait {
    pub row_id: u32,
    pub name: String,
    pub icon_id: u32,
    pub class_job_id: u32,
    pub class_job: RowRef<ClassJob>,
    pub level: u8,
    pub quest_id: u32,
    pub quest: RowRef<Quest>,
    pub value: u32,
    pub class_job_category_id: u32,
    pub class_job_category: RowRef<ClassJobCategory>,
}

impl Sheet for Trait {
    const SHEET_NAME: &'static str = "Trait";
}

impl FromExcelRow for Trait {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            name: single_row.columns.get(0).to_owned_string(),
            icon_id: single_row.columns.get(1).to_u32(),
            class_job_id: single_row.columns.get(2).to_u32(),
            class_job: RowRef::<ClassJob>::from(single_row.columns.get(2).to_u32()),
            level: single_row.columns.get(4).to_u8(),
            quest_id: single_row.columns.get(5).to_u32(),
            quest: RowRef::<Quest>::from(single_row.columns.get(5).to_u32()),
            value: single_row.columns.get(6).to_u32(),
            class_job_category_id: single_row.columns.get(7).to_u32(),
            class_job_category: RowRef::<ClassJobCategory>::from(single_row.columns.get(7).to_u32()),
        })
    }
}

