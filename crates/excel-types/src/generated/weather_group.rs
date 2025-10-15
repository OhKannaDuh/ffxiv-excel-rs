/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct WeatherGroup {
    pub row_id: u32,
    pub weather_rate_id: u32,
    pub weather_rate: RowRef<WeatherRate>,
}

impl Sheet for WeatherGroup {
    const SHEET_NAME: &'static str = "WeatherGroup";
}

impl FromExcelRow for WeatherGroup {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            weather_rate_id: single_row.columns.get(1).to_u32(),
            weather_rate: RowRef::<WeatherRate>::from(single_row.columns.get(1).to_u32()),
        })
    }
}

