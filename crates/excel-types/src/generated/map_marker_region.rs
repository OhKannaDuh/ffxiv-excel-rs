/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct MapMarkerRegion {
    pub row_id: u32,
    pub x: i16,
}

impl Sheet for MapMarkerRegion {
    const SHEET_NAME: &'static str = "MapMarkerRegion";
}

impl FromExcelRow for MapMarkerRegion {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            x: single_row.columns.get(1).to_i16(),
        })
    }
}

