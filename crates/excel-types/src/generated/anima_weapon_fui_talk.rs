/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct AnimaWeaponFUITalk {
    pub row_id: u32,
    pub dialogue_id: u32,
    pub dialogue: RowRef<AnimaWeaponFUITalkParam>,
}

impl Sheet for AnimaWeaponFUITalk {
    const SHEET_NAME: &'static str = "AnimaWeaponFUITalk";
}

impl FromExcelRow for AnimaWeaponFUITalk {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            dialogue_id: single_row.columns.get(0).to_u32(),
            dialogue: RowRef::<AnimaWeaponFUITalkParam>::from(single_row.columns.get(0).to_u32()),
        })
    }
}

