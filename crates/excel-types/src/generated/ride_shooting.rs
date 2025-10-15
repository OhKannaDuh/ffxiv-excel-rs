/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct RideShooting {
    pub row_id: u32,
    pub g_fate_ride_shooting_id: u32,
    pub g_fate_ride_shooting: RowRef<GFateRideShooting>,
    pub start_text_id: u32,
    pub start_text: RowRef<RideShootingTextData>,
}

impl Sheet for RideShooting {
    const SHEET_NAME: &'static str = "RideShooting";
}

impl FromExcelRow for RideShooting {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            g_fate_ride_shooting_id: single_row.columns.get(0).to_u32(),
            g_fate_ride_shooting: RowRef::<GFateRideShooting>::from(single_row.columns.get(0).to_u32()),
            start_text_id: single_row.columns.get(5).to_u32(),
            start_text: RowRef::<RideShootingTextData>::from(single_row.columns.get(5).to_u32()),
        })
    }
}

