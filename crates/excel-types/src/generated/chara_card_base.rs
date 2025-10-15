/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct CharaCardBase {
    pub row_id: u32,
    pub image_id: u32,
    pub font_color: u8,
    pub unlock_condition_id: u32,
    pub unlock_condition: RowRef<BannerCondition>,
    pub sort_key: u8,
    pub name: u16,
}

impl Sheet for CharaCardBase {
    const SHEET_NAME: &'static str = "CharaCardBase";
}

impl FromExcelRow for CharaCardBase {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            image_id: single_row.columns.get(0).to_u32(),
            font_color: single_row.columns.get(1).to_u8(),
            unlock_condition_id: single_row.columns.get(5).to_u32(),
            unlock_condition: RowRef::<BannerCondition>::from(single_row.columns.get(5).to_u32()),
            sort_key: single_row.columns.get(9).to_u8(),
            name: single_row.columns.get(10).to_u16(),
        })
    }
}

