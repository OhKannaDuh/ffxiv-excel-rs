/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct Achievement {
    pub row_id: u32,
    pub achievement_category_id: u32,
    pub achievement_category: RowRef<AchievementCategory>,
    pub name: String,
    pub description: String,
    pub achievement_target_id: u32,
    pub achievement_target: RowRef<AchievementTarget>,
    pub points: u8,
    pub title_id: u32,
    pub title: RowRef<Title>,
    pub item_id: u32,
    pub item: RowRef<Item>,
    pub icon_id: u32,
    pub _type: u8,
    pub key_id: u32,
    pub order: u16,
    pub achievement_hide_condition_id: u32,
    pub achievement_hide_condition: RowRef<AchievementHideCondition>,
}

impl Sheet for Achievement {
    const SHEET_NAME: &'static str = "Achievement";
}

impl FromExcelRow for Achievement {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            achievement_category_id: single_row.columns.get(0).to_u32(),
            achievement_category: RowRef::<AchievementCategory>::from(single_row.columns.get(0).to_u32()),
            name: single_row.columns.get(1).to_owned_string(),
            description: single_row.columns.get(2).to_owned_string(),
            achievement_target_id: single_row.columns.get(3).to_u32(),
            achievement_target: RowRef::<AchievementTarget>::from(single_row.columns.get(3).to_u32()),
            points: single_row.columns.get(5).to_u8(),
            title_id: single_row.columns.get(6).to_u32(),
            title: RowRef::<Title>::from(single_row.columns.get(6).to_u32()),
            item_id: single_row.columns.get(7).to_u32(),
            item: RowRef::<Item>::from(single_row.columns.get(7).to_u32()),
            icon_id: single_row.columns.get(11).to_u32(),
            _type: single_row.columns.get(13).to_u8(),
            key_id: single_row.columns.get(14).to_u32(),
            order: single_row.columns.get(23).to_u16(),
            achievement_hide_condition_id: single_row.columns.get(25).to_u32(),
            achievement_hide_condition: RowRef::<AchievementHideCondition>::from(single_row.columns.get(25).to_u32()),
        })
    }
}

