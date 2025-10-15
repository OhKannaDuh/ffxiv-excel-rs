/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct Treasure {
    pub row_id: u32,
    pub sgb_id: u32,
    pub sgb: RowRef<ExportedSG>,
}

impl Sheet for Treasure {
    const SHEET_NAME: &'static str = "Treasure";
}

impl FromExcelRow for Treasure {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            sgb_id: single_row.columns.get(8).to_u32(),
            sgb: RowRef::<ExportedSG>::from(single_row.columns.get(8).to_u32()),
        })
    }
}

