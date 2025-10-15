/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct QuestRedoChapterUI {
    pub row_id: u32,
    pub quest_id: u32,
    pub quest: RowRef<Quest>,
    pub ui_tab_id: u32,
    pub ui_tab: RowRef<QuestRedoChapterUITab>,
    pub category_id: u32,
    pub category: RowRef<QuestRedoChapterUICategory>,
    pub quest_redo_ui_small_id: u32,
    pub quest_redo_ui_large_id: u32,
    pub quest_redo_ui_wide_id: u32,
    pub chapter_name: String,
    pub chapter_part: String,
    pub transient: String,
}

impl Sheet for QuestRedoChapterUI {
    const SHEET_NAME: &'static str = "QuestRedoChapterUI";
}

impl FromExcelRow for QuestRedoChapterUI {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            quest_id: single_row.columns.get(0).to_u32(),
            quest: RowRef::<Quest>::from(single_row.columns.get(0).to_u32()),
            ui_tab_id: single_row.columns.get(2).to_u32(),
            ui_tab: RowRef::<QuestRedoChapterUITab>::from(single_row.columns.get(2).to_u32()),
            category_id: single_row.columns.get(3).to_u32(),
            category: RowRef::<QuestRedoChapterUICategory>::from(single_row.columns.get(3).to_u32()),
            quest_redo_ui_small_id: single_row.columns.get(5).to_u32(),
            quest_redo_ui_large_id: single_row.columns.get(6).to_u32(),
            quest_redo_ui_wide_id: single_row.columns.get(7).to_u32(),
            chapter_name: single_row.columns.get(8).to_owned_string(),
            chapter_part: single_row.columns.get(9).to_owned_string(),
            transient: single_row.columns.get(10).to_owned_string(),
        })
    }
}

