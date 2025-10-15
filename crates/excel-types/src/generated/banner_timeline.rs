/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct BannerTimeline {
    pub row_id: u32,
    pub _type: u8,
    pub additional_data_id: u32,
    pub accept_class_job_category_id: u32,
    pub accept_class_job_category: RowRef<ClassJobCategory>,
    pub category: u8,
    pub unlock_condition_id: u32,
    pub unlock_condition: RowRef<BannerCondition>,
    pub sort_key: u16,
    pub icon_id: u32,
    pub name: i32,
}

impl Sheet for BannerTimeline {
    const SHEET_NAME: &'static str = "BannerTimeline";
}

impl FromExcelRow for BannerTimeline {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            _type: single_row.columns.get(0).to_u8(),
            additional_data_id: single_row.columns.get(1).to_u32(),
            accept_class_job_category_id: single_row.columns.get(2).to_u32(),
            accept_class_job_category: RowRef::<ClassJobCategory>::from(single_row.columns.get(2).to_u32()),
            category: single_row.columns.get(3).to_u8(),
            unlock_condition_id: single_row.columns.get(4).to_u32(),
            unlock_condition: RowRef::<BannerCondition>::from(single_row.columns.get(4).to_u32()),
            sort_key: single_row.columns.get(7).to_u16(),
            icon_id: single_row.columns.get(8).to_u32(),
            name: single_row.columns.get(9).to_i32(),
        })
    }
}

