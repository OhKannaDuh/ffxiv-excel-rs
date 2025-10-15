/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct GroupPoseStamp {
    pub row_id: u32,
    pub stamp_icon_id: u32,
    pub category_id: u32,
    pub category: RowRef<GroupPoseStampCategory>,
    pub name: String,
}

impl Sheet for GroupPoseStamp {
    const SHEET_NAME: &'static str = "GroupPoseStamp";
}

impl FromExcelRow for GroupPoseStamp {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            stamp_icon_id: single_row.columns.get(0).to_u32(),
            category_id: single_row.columns.get(2).to_u32(),
            category: RowRef::<GroupPoseStampCategory>::from(single_row.columns.get(2).to_u32()),
            name: single_row.columns.get(10).to_owned_string(),
        })
    }
}

