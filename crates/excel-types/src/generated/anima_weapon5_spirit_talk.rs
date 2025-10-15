/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct AnimaWeapon5SpiritTalk {
    pub row_id: u32,
    pub dialogue_id: u32,
    pub dialogue: RowRef<AnimaWeapon5SpiritTalkParam>,
}

impl Sheet for AnimaWeapon5SpiritTalk {
    const SHEET_NAME: &'static str = "AnimaWeapon5SpiritTalk";
}

impl FromExcelRow for AnimaWeapon5SpiritTalk {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            dialogue_id: single_row.columns.get(0).to_u32(),
            dialogue: RowRef::<AnimaWeapon5SpiritTalkParam>::from(single_row.columns.get(0).to_u32()),
        })
    }
}

