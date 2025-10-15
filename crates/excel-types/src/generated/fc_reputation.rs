/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct FCReputation {
    pub row_id: u32,
    pub points_to_next: u32,
    pub required_points: u32,
    pub discount_rate: u8,
    pub color_id: u32,
    pub color: RowRef<UIColor>,
    pub name: String,
}

impl Sheet for FCReputation {
    const SHEET_NAME: &'static str = "FCReputation";
}

impl FromExcelRow for FCReputation {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            points_to_next: single_row.columns.get(0).to_u32(),
            required_points: single_row.columns.get(1).to_u32(),
            discount_rate: single_row.columns.get(2).to_u8(),
            color_id: single_row.columns.get(3).to_u32(),
            color: RowRef::<UIColor>::from(single_row.columns.get(3).to_u32()),
            name: single_row.columns.get(4).to_owned_string(),
        })
    }
}

