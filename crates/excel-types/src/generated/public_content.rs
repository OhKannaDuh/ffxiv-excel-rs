/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct PublicContent {
    pub row_id: u32,
    pub _type: u8,
    pub time_limit: u16,
    pub map_icon_id: u32,
    pub name: String,
    pub text_data_start_id: u32,
    pub text_data_start: RowRef<PublicContentTextData>,
    pub text_data_end_id: u32,
    pub text_data_end: RowRef<PublicContentTextData>,
    pub start_cutscene_id: u32,
    pub start_cutscene: RowRef<PublicContentCutscene>,
    pub lgb_event_range: u32,
    pub lgb_pop_range: u32,
    pub content_finder_condition_id: u32,
    pub content_finder_condition: RowRef<ContentFinderCondition>,
    pub additional_data_id: u32,
    pub end_cutscene_id: u32,
    pub end_cutscene: RowRef<PublicContentCutscene>,
}

impl Sheet for PublicContent {
    const SHEET_NAME: &'static str = "PublicContent";
}

impl FromExcelRow for PublicContent {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            _type: single_row.columns.get(0).to_u8(),
            time_limit: single_row.columns.get(1).to_u16(),
            map_icon_id: single_row.columns.get(2).to_u32(),
            name: single_row.columns.get(3).to_owned_string(),
            text_data_start_id: single_row.columns.get(4).to_u32(),
            text_data_start: RowRef::<PublicContentTextData>::from(single_row.columns.get(4).to_u32()),
            text_data_end_id: single_row.columns.get(5).to_u32(),
            text_data_end: RowRef::<PublicContentTextData>::from(single_row.columns.get(5).to_u32()),
            start_cutscene_id: single_row.columns.get(6).to_u32(),
            start_cutscene: RowRef::<PublicContentCutscene>::from(single_row.columns.get(6).to_u32()),
            lgb_event_range: single_row.columns.get(7).to_u32(),
            lgb_pop_range: single_row.columns.get(8).to_u32(),
            content_finder_condition_id: single_row.columns.get(9).to_u32(),
            content_finder_condition: RowRef::<ContentFinderCondition>::from(single_row.columns.get(9).to_u32()),
            additional_data_id: single_row.columns.get(10).to_u32(),
            end_cutscene_id: single_row.columns.get(16).to_u32(),
            end_cutscene: RowRef::<PublicContentCutscene>::from(single_row.columns.get(16).to_u32()),
        })
    }
}

