/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct BGMFade {
    pub row_id: u32,
    pub scene_out: i32,
    pub scene_in: i32,
    pub bgm_fade_type_id: u32,
    pub bgm_fade_type: RowRef<BGMFadeType>,
}

impl Sheet for BGMFade {
    const SHEET_NAME: &'static str = "BGMFade";
}

impl FromExcelRow for BGMFade {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            scene_out: single_row.columns.get(0).to_i32(),
            scene_in: single_row.columns.get(1).to_i32(),
            bgm_fade_type_id: single_row.columns.get(2).to_u32(),
            bgm_fade_type: RowRef::<BGMFadeType>::from(single_row.columns.get(2).to_u32()),
        })
    }
}

