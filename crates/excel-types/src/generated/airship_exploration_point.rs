/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct AirshipExplorationPoint {
    pub row_id: u32,
    pub name: String,
    pub name_short: String,
    pub passengers: bool,
    pub x: i16,
    pub y: i16,
    pub rank_req: u8,
    pub ceruleum_tank_req: u16,
    pub survey_duration_min: u16,
    pub survey_distance: u16,
    pub surveillance_req: u8,
    pub exp_reward: u32,
}

impl Sheet for AirshipExplorationPoint {
    const SHEET_NAME: &'static str = "AirshipExplorationPoint";
}

impl FromExcelRow for AirshipExplorationPoint {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            name: single_row.columns.get(0).to_owned_string(),
            name_short: single_row.columns.get(1).to_owned_string(),
            passengers: single_row.columns.get(2).to_bit(0),
            x: single_row.columns.get(3).to_i16(),
            y: single_row.columns.get(4).to_i16(),
            rank_req: single_row.columns.get(5).to_u8(),
            ceruleum_tank_req: single_row.columns.get(6).to_u16(),
            survey_duration_min: single_row.columns.get(7).to_u16(),
            survey_distance: single_row.columns.get(8).to_u16(),
            surveillance_req: single_row.columns.get(10).to_u8(),
            exp_reward: single_row.columns.get(13).to_u32(),
        })
    }
}

