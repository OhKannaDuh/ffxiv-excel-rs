/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct ChocoboTaxi {
    pub row_id: u32,
    pub location_id: u32,
    pub location: RowRef<ChocoboTaxiStand>,
    pub fare: u8,
    pub time_required: u16,
}

impl Sheet for ChocoboTaxi {
    const SHEET_NAME: &'static str = "ChocoboTaxi";
}

impl FromExcelRow for ChocoboTaxi {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            location_id: single_row.columns.get(0).to_u32(),
            location: RowRef::<ChocoboTaxiStand>::from(single_row.columns.get(0).to_u32()),
            fare: single_row.columns.get(1).to_u8(),
            time_required: single_row.columns.get(2).to_u16(),
        })
    }
}

