/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct PartyContent {
    pub row_id: u32,
    pub key: u8,
    pub time_limit: u16,
    pub name: bool,
    pub text_data_start_id: u32,
    pub text_data_start: RowRef<PartyContentTextData>,
    pub text_data_end_id: u32,
    pub text_data_end: RowRef<PartyContentTextData>,
    pub content_finder_condition_id: u32,
    pub content_finder_condition: RowRef<ContentFinderCondition>,
    pub image_id: u32,
}

impl Sheet for PartyContent {
    const SHEET_NAME: &'static str = "PartyContent";
}

impl FromExcelRow for PartyContent {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            key: single_row.columns.get(0).to_u8(),
            time_limit: single_row.columns.get(1).to_u16(),
            name: single_row.columns.get(2).to_bit(0),
            text_data_start_id: single_row.columns.get(3).to_u32(),
            text_data_start: RowRef::<PartyContentTextData>::from(single_row.columns.get(3).to_u32()),
            text_data_end_id: single_row.columns.get(4).to_u32(),
            text_data_end: RowRef::<PartyContentTextData>::from(single_row.columns.get(4).to_u32()),
            content_finder_condition_id: single_row.columns.get(33).to_u32(),
            content_finder_condition: RowRef::<ContentFinderCondition>::from(single_row.columns.get(33).to_u32()),
            image_id: single_row.columns.get(34).to_u32(),
        })
    }
}

