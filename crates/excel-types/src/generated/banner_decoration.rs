/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct BannerDecoration {
    pub row_id: u32,
    pub image_id: u32,
    pub icon_id: u32,
    pub unlock_condition_id: u32,
    pub unlock_condition: RowRef<BannerCondition>,
    pub sort_key: u16,
    pub name: u16,
}

impl Sheet for BannerDecoration {
    const SHEET_NAME: &'static str = "BannerDecoration";
}

impl FromExcelRow for BannerDecoration {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            image_id: single_row.columns.get(0).to_u32(),
            icon_id: single_row.columns.get(1).to_u32(),
            unlock_condition_id: single_row.columns.get(3).to_u32(),
            unlock_condition: RowRef::<BannerCondition>::from(single_row.columns.get(3).to_u32()),
            sort_key: single_row.columns.get(6).to_u16(),
            name: single_row.columns.get(7).to_u16(),
        })
    }
}

