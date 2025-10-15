/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct EurekaMagiciteItem {
    pub row_id: u32,
    pub eureka_magicite_item_type_id: u32,
    pub eureka_magicite_item_type: RowRef<EurekaMagiciteItemType>,
    pub class_job_category_id: u32,
    pub class_job_category: RowRef<ClassJobCategory>,
    pub item_id: u32,
    pub item: RowRef<Item>,
}

impl Sheet for EurekaMagiciteItem {
    const SHEET_NAME: &'static str = "EurekaMagiciteItem";
}

impl FromExcelRow for EurekaMagiciteItem {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            eureka_magicite_item_type_id: single_row.columns.get(0).to_u32(),
            eureka_magicite_item_type: RowRef::<EurekaMagiciteItemType>::from(single_row.columns.get(0).to_u32()),
            class_job_category_id: single_row.columns.get(1).to_u32(),
            class_job_category: RowRef::<ClassJobCategory>::from(single_row.columns.get(1).to_u32()),
            item_id: single_row.columns.get(2).to_u32(),
            item: RowRef::<Item>::from(single_row.columns.get(2).to_u32()),
        })
    }
}

