/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct MovieSubtitle {
    pub row_id: u32,
    pub start_time: f32,
    pub end_time: f32,
}

impl Sheet for MovieSubtitle {
    const SHEET_NAME: &'static str = "MovieSubtitle";
}

impl FromExcelRow for MovieSubtitle {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            start_time: single_row.columns.get(0).to_f32(),
            end_time: single_row.columns.get(1).to_f32(),
        })
    }
}

