/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct CharaCardDecoration {
    pub row_id: u32,
    pub category: u8,
    pub sub_type: u8,
    pub image_id: u32,
    pub unlock_condition_id: u32,
    pub unlock_condition: RowRef<BannerCondition>,
    pub sort_key: u8,
    pub name: u16,
}

impl Sheet for CharaCardDecoration {
    const SHEET_NAME: &'static str = "CharaCardDecoration";
}

impl FromExcelRow for CharaCardDecoration {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            category: single_row.columns.get(0).to_u8(),
            sub_type: single_row.columns.get(1).to_u8(),
            image_id: single_row.columns.get(2).to_u32(),
            unlock_condition_id: single_row.columns.get(4).to_u32(),
            unlock_condition: RowRef::<BannerCondition>::from(single_row.columns.get(4).to_u32()),
            sort_key: single_row.columns.get(8).to_u8(),
            name: single_row.columns.get(9).to_u16(),
        })
    }
}

