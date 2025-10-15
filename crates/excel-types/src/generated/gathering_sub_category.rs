/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct GatheringSubCategory {
    pub row_id: u32,
    pub gathering_type_id: u32,
    pub gathering_type: RowRef<GatheringType>,
    pub class_job_id: u32,
    pub class_job: RowRef<ClassJob>,
    pub quest_id: u32,
    pub quest: RowRef<Quest>,
    pub division: u16,
    pub item_id: u32,
    pub item: RowRef<Item>,
    pub folklore_book: String,
}

impl Sheet for GatheringSubCategory {
    const SHEET_NAME: &'static str = "GatheringSubCategory";
}

impl FromExcelRow for GatheringSubCategory {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            gathering_type_id: single_row.columns.get(0).to_u32(),
            gathering_type: RowRef::<GatheringType>::from(single_row.columns.get(0).to_u32()),
            class_job_id: single_row.columns.get(1).to_u32(),
            class_job: RowRef::<ClassJob>::from(single_row.columns.get(1).to_u32()),
            quest_id: single_row.columns.get(2).to_u32(),
            quest: RowRef::<Quest>::from(single_row.columns.get(2).to_u32()),
            division: single_row.columns.get(3).to_u16(),
            item_id: single_row.columns.get(4).to_u32(),
            item: RowRef::<Item>::from(single_row.columns.get(4).to_u32()),
            folklore_book: single_row.columns.get(5).to_owned_string(),
        })
    }
}

