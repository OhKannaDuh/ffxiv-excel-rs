/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct TerritoryTypeTelepo {
    pub row_id: u32,
    pub x: u16,
    pub y: u16,
    pub expansion: u16,
    pub telepo_relay_id: u32,
    pub telepo_relay: RowRef<TelepoRelay>,
}

impl Sheet for TerritoryTypeTelepo {
    const SHEET_NAME: &'static str = "TerritoryTypeTelepo";
}

impl FromExcelRow for TerritoryTypeTelepo {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            x: single_row.columns.get(0).to_u16(),
            y: single_row.columns.get(1).to_u16(),
            expansion: single_row.columns.get(2).to_u16(),
            telepo_relay_id: single_row.columns.get(3).to_u32(),
            telepo_relay: RowRef::<TelepoRelay>::from(single_row.columns.get(3).to_u32()),
        })
    }
}

