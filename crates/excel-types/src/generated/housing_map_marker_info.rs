/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct HousingMapMarkerInfo {
    pub row_id: u32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub map_id: u32,
    pub map: RowRef<Map>,
}

impl Sheet for HousingMapMarkerInfo {
    const SHEET_NAME: &'static str = "HousingMapMarkerInfo";
}

impl FromExcelRow for HousingMapMarkerInfo {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            x: single_row.columns.get(0).to_f32(),
            y: single_row.columns.get(1).to_f32(),
            z: single_row.columns.get(2).to_f32(),
            map_id: single_row.columns.get(4).to_u32(),
            map: RowRef::<Map>::from(single_row.columns.get(4).to_u32()),
        })
    }
}

