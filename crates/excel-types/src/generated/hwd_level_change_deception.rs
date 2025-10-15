/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct HWDLevelChangeDeception {
    pub row_id: u32,
    pub image_id: u32,
    pub image: RowRef<ScreenImage>,
}

impl Sheet for HWDLevelChangeDeception {
    const SHEET_NAME: &'static str = "HWDLevelChangeDeception";
}

impl FromExcelRow for HWDLevelChangeDeception {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            image_id: single_row.columns.get(0).to_u32(),
            image: RowRef::<ScreenImage>::from(single_row.columns.get(0).to_u32()),
        })
    }
}

