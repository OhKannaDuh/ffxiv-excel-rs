/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct RideShootingTargetType {
    pub row_id: u32,
    pub e_obj_id: u32,
    pub e_obj: RowRef<EObj>,
    pub score: i16,
}

impl Sheet for RideShootingTargetType {
    const SHEET_NAME: &'static str = "RideShootingTargetType";
}

impl FromExcelRow for RideShootingTargetType {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            e_obj_id: single_row.columns.get(0).to_u32(),
            e_obj: RowRef::<EObj>::from(single_row.columns.get(0).to_u32()),
            score: single_row.columns.get(1).to_i16(),
        })
    }
}

