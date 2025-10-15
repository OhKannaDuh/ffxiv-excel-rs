/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct Adventure {
    pub row_id: u32,
    pub level_id: u32,
    pub level: RowRef<Level>,
    pub min_level: i32,
    pub max_level: u8,
    pub emote_id: u32,
    pub emote: RowRef<Emote>,
    pub min_time: u16,
    pub max_time: u16,
    pub place_name_id: u32,
    pub place_name: RowRef<PlaceName>,
    pub icon_list_id: u32,
    pub icon_discovered_id: u32,
    pub icon_undiscovered_id: u32,
    pub is_initial: bool,
    pub name: String,
    pub impression: String,
    pub description: String,
}

impl Sheet for Adventure {
    const SHEET_NAME: &'static str = "Adventure";
}

impl FromExcelRow for Adventure {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            level_id: single_row.columns.get(0).to_u32(),
            level: RowRef::<Level>::from(single_row.columns.get(0).to_u32()),
            min_level: single_row.columns.get(1).to_i32(),
            max_level: single_row.columns.get(2).to_u8(),
            emote_id: single_row.columns.get(3).to_u32(),
            emote: RowRef::<Emote>::from(single_row.columns.get(3).to_u32()),
            min_time: single_row.columns.get(4).to_u16(),
            max_time: single_row.columns.get(5).to_u16(),
            place_name_id: single_row.columns.get(6).to_u32(),
            place_name: RowRef::<PlaceName>::from(single_row.columns.get(6).to_u32()),
            icon_list_id: single_row.columns.get(7).to_u32(),
            icon_discovered_id: single_row.columns.get(8).to_u32(),
            icon_undiscovered_id: single_row.columns.get(9).to_u32(),
            is_initial: single_row.columns.get(10).to_bit(0),
            name: single_row.columns.get(11).to_owned_string(),
            impression: single_row.columns.get(12).to_owned_string(),
            description: single_row.columns.get(13).to_owned_string(),
        })
    }
}

