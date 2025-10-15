/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct ScenarioTree {
    pub row_id: u32,
    pub _type_id: u32,
    pub _type: RowRef<ScenarioType>,
    pub addon_id: u32,
    pub addon: RowRef<Addon>,
    pub quest_chapter_id: u32,
    pub quest_chapter: RowRef<QuestChapter>,
    pub name: String,
}

impl Sheet for ScenarioTree {
    const SHEET_NAME: &'static str = "ScenarioTree";
}

impl FromExcelRow for ScenarioTree {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            _type_id: single_row.columns.get(0).to_u32(),
            _type: RowRef::<ScenarioType>::from(single_row.columns.get(0).to_u32()),
            addon_id: single_row.columns.get(2).to_u32(),
            addon: RowRef::<Addon>::from(single_row.columns.get(2).to_u32()),
            quest_chapter_id: single_row.columns.get(3).to_u32(),
            quest_chapter: RowRef::<QuestChapter>::from(single_row.columns.get(3).to_u32()),
            name: single_row.columns.get(4).to_owned_string(),
        })
    }
}

