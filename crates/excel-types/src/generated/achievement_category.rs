/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct AchievementCategory {
    pub row_id: u32,
    pub name: String,
    pub achievement_kind_id: u32,
    pub achievement_kind: RowRef<AchievementKind>,
    pub show_complete: bool,
    pub hide_category: bool,
    pub order: u8,
}

impl Sheet for AchievementCategory {
    const SHEET_NAME: &'static str = "AchievementCategory";
}

impl FromExcelRow for AchievementCategory {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            name: single_row.columns.get(0).to_owned_string(),
            achievement_kind_id: single_row.columns.get(1).to_u32(),
            achievement_kind: RowRef::<AchievementKind>::from(single_row.columns.get(1).to_u32()),
            show_complete: single_row.columns.get(2).to_bit(0),
            hide_category: single_row.columns.get(3).to_bit(1),
            order: single_row.columns.get(4).to_u8(),
        })
    }
}

