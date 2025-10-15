/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct BannerCondition {
    pub row_id: u32,
    pub unlock_type1: u8,
    pub unlock_type2: u32,
    pub unlock_criteria2_id: u32,
    pub unlock_criteria3_id: u32,
    pub unlock_criteria4_id: u32,
    pub prerequisite_type: bool,
    pub prerequisite_id: u32,
    pub unlock_hint_id: u32,
    pub unlock_hint: RowRef<BannerObtainHintType>,
}

impl Sheet for BannerCondition {
    const SHEET_NAME: &'static str = "BannerCondition";
}

impl FromExcelRow for BannerCondition {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            unlock_type1: single_row.columns.get(0).to_u8(),
            unlock_type2: single_row.columns.get(7).to_u32(),
            unlock_criteria2_id: single_row.columns.get(8).to_u32(),
            unlock_criteria3_id: single_row.columns.get(9).to_u32(),
            unlock_criteria4_id: single_row.columns.get(10).to_u32(),
            prerequisite_type: single_row.columns.get(11).to_bit(0),
            prerequisite_id: single_row.columns.get(12).to_u32(),
            unlock_hint_id: single_row.columns.get(13).to_u32(),
            unlock_hint: RowRef::<BannerObtainHintType>::from(single_row.columns.get(13).to_u32()),
        })
    }
}

