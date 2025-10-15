/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct QuestClassJobReward {
    pub row_id: u32,
    pub class_job_category_id: u32,
    pub class_job_category: RowRef<ClassJobCategory>,
}

impl Sheet for QuestClassJobReward {
    const SHEET_NAME: &'static str = "QuestClassJobReward";
}

impl FromExcelRow for QuestClassJobReward {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            class_job_category_id: single_row.columns.get(0).to_u32(),
            class_job_category: RowRef::<ClassJobCategory>::from(single_row.columns.get(0).to_u32()),
        })
    }
}

