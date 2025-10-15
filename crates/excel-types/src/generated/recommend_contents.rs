/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct RecommendContents {
    pub row_id: u32,
    pub level_id: u32,
    pub level: RowRef<Level>,
    pub class_job_id: u32,
    pub class_job: RowRef<ClassJob>,
    pub min_level: u8,
    pub max_level: u8,
}

impl Sheet for RecommendContents {
    const SHEET_NAME: &'static str = "RecommendContents";
}

impl FromExcelRow for RecommendContents {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            level_id: single_row.columns.get(0).to_u32(),
            level: RowRef::<Level>::from(single_row.columns.get(0).to_u32()),
            class_job_id: single_row.columns.get(1).to_u32(),
            class_job: RowRef::<ClassJob>::from(single_row.columns.get(1).to_u32()),
            min_level: single_row.columns.get(2).to_u8(),
            max_level: single_row.columns.get(3).to_u8(),
        })
    }
}

