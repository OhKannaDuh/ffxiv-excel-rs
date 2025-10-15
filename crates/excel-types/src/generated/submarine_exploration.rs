/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct SubmarineExploration {
    pub row_id: u32,
    pub destination: String,
    pub location: String,
    pub x: i16,
    pub y: i16,
    pub z: i16,
    pub map_id: u32,
    pub map: RowRef<SubmarineMap>,
    pub starting_point: bool,
    pub stars: u8,
    pub rank_req: u8,
    pub ceruleum_tank_req: u8,
    pub survey_duration_min: u16,
    pub survey_distance: u8,
    pub exp_reward: u32,
}

impl Sheet for SubmarineExploration {
    const SHEET_NAME: &'static str = "SubmarineExploration";
}

impl FromExcelRow for SubmarineExploration {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            destination: single_row.columns.get(0).to_owned_string(),
            location: single_row.columns.get(1).to_owned_string(),
            x: single_row.columns.get(2).to_i16(),
            y: single_row.columns.get(3).to_i16(),
            z: single_row.columns.get(4).to_i16(),
            map_id: single_row.columns.get(5).to_u32(),
            map: RowRef::<SubmarineMap>::from(single_row.columns.get(5).to_u32()),
            starting_point: single_row.columns.get(6).to_bit(0),
            stars: single_row.columns.get(7).to_u8(),
            rank_req: single_row.columns.get(8).to_u8(),
            ceruleum_tank_req: single_row.columns.get(9).to_u8(),
            survey_duration_min: single_row.columns.get(10).to_u16(),
            survey_distance: single_row.columns.get(11).to_u8(),
            exp_reward: single_row.columns.get(12).to_u32(),
        })
    }
}

