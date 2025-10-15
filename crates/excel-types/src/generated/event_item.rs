/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct EventItem {
    pub row_id: u32,
    pub singular: String,
    pub adjective: i8,
    pub plural: String,
    pub possessive_pronoun: i8,
    pub starts_with_vowel: i8,
    pub pronoun: i8,
    pub article: i8,
    pub name: String,
    pub icon_id: u32,
    pub action_id: u32,
    pub action: RowRef<Action>,
    pub stack_size: u8,
    pub quest_id: u32,
    pub quest: RowRef<Quest>,
    pub cast_time: u8,
    pub cast_timeline_id: u32,
    pub cast_timeline: RowRef<EventItemCastTimeline>,
    pub timeline: u8,
}

impl Sheet for EventItem {
    const SHEET_NAME: &'static str = "EventItem";
}

impl FromExcelRow for EventItem {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            singular: single_row.columns.get(0).to_owned_string(),
            adjective: single_row.columns.get(1).to_i8(),
            plural: single_row.columns.get(2).to_owned_string(),
            possessive_pronoun: single_row.columns.get(3).to_i8(),
            starts_with_vowel: single_row.columns.get(4).to_i8(),
            pronoun: single_row.columns.get(6).to_i8(),
            article: single_row.columns.get(7).to_i8(),
            name: single_row.columns.get(9).to_owned_string(),
            icon_id: single_row.columns.get(10).to_u32(),
            action_id: single_row.columns.get(11).to_u32(),
            action: RowRef::<Action>::from(single_row.columns.get(11).to_u32()),
            stack_size: single_row.columns.get(12).to_u8(),
            quest_id: single_row.columns.get(14).to_u32(),
            quest: RowRef::<Quest>::from(single_row.columns.get(14).to_u32()),
            cast_time: single_row.columns.get(15).to_u8(),
            cast_timeline_id: single_row.columns.get(16).to_u32(),
            cast_timeline: RowRef::<EventItemCastTimeline>::from(single_row.columns.get(16).to_u32()),
            timeline: single_row.columns.get(17).to_u8(),
        })
    }
}

