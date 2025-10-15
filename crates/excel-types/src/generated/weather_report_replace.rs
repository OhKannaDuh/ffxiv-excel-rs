/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct WeatherReportReplace {
    pub row_id: u32,
    pub place_name_sub_id: u32,
    pub place_name_sub: RowRef<PlaceName>,
    pub place_name_parent_id: u32,
    pub place_name_parent: RowRef<PlaceName>,
}

impl Sheet for WeatherReportReplace {
    const SHEET_NAME: &'static str = "WeatherReportReplace";
}

impl FromExcelRow for WeatherReportReplace {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            place_name_sub_id: single_row.columns.get(0).to_u32(),
            place_name_sub: RowRef::<PlaceName>::from(single_row.columns.get(0).to_u32()),
            place_name_parent_id: single_row.columns.get(1).to_u32(),
            place_name_parent: RowRef::<PlaceName>::from(single_row.columns.get(1).to_u32()),
        })
    }
}

