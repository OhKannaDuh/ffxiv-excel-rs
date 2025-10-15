/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct ChocoboRaceStatus {
    pub row_id: u32,
    pub status_id: u32,
    pub status: RowRef<Status>,
}

impl Sheet for ChocoboRaceStatus {
    const SHEET_NAME: &'static str = "ChocoboRaceStatus";
}

impl FromExcelRow for ChocoboRaceStatus {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            status_id: single_row.columns.get(0).to_u32(),
            status: RowRef::<Status>::from(single_row.columns.get(0).to_u32()),
        })
    }
}

