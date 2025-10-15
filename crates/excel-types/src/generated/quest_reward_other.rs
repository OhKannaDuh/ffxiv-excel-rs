/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct QuestRewardOther {
    pub row_id: u32,
    pub icon_id: u32,
    pub name: String,
}

impl Sheet for QuestRewardOther {
    const SHEET_NAME: &'static str = "QuestRewardOther";
}

impl FromExcelRow for QuestRewardOther {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            icon_id: single_row.columns.get(0).to_u32(),
            name: single_row.columns.get(1).to_owned_string(),
        })
    }
}

