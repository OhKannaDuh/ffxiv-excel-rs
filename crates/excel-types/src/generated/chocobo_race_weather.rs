/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct ChocoboRaceWeather {
    pub row_id: u32,
    pub weather_type_1_id: u32,
    pub weather_type_1: RowRef<Weather>,
    pub weather_type_2_id: u32,
    pub weather_type_2: RowRef<Weather>,
}

impl Sheet for ChocoboRaceWeather {
    const SHEET_NAME: &'static str = "ChocoboRaceWeather";
}

impl FromExcelRow for ChocoboRaceWeather {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            weather_type_1_id: single_row.columns.get(0).to_u32(),
            weather_type_1: RowRef::<Weather>::from(single_row.columns.get(0).to_u32()),
            weather_type_2_id: single_row.columns.get(1).to_u32(),
            weather_type_2: RowRef::<Weather>::from(single_row.columns.get(1).to_u32()),
        })
    }
}

