/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct AchievementHideCondition {
    pub row_id: u32,
    pub hide_achievement: bool,
    pub hide_name: bool,
    pub hide_conditions: bool,
}

impl Sheet for AchievementHideCondition {
    const SHEET_NAME: &'static str = "AchievementHideCondition";
}

impl FromExcelRow for AchievementHideCondition {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            hide_achievement: single_row.columns.get(0).to_bit(0),
            hide_name: single_row.columns.get(1).to_bit(1),
            hide_conditions: single_row.columns.get(2).to_bit(2),
        })
    }
}

