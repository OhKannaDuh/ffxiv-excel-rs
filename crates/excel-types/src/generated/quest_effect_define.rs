/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct QuestEffectDefine {
    pub row_id: u32,
    pub effect_id: u32,
}

impl Sheet for QuestEffectDefine {
    const SHEET_NAME: &'static str = "QuestEffectDefine";
}

impl FromExcelRow for QuestEffectDefine {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            effect_id: single_row.columns.get(0).to_u32(),
        })
    }
}

