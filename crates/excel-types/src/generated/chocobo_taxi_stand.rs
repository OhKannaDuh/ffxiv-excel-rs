/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct ChocoboTaxiStand {
    pub row_id: u32,
    pub place_name: String,
}

impl Sheet for ChocoboTaxiStand {
    const SHEET_NAME: &'static str = "ChocoboTaxiStand";
}

impl FromExcelRow for ChocoboTaxiStand {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            place_name: single_row.columns.get(8).to_owned_string(),
        })
    }
}

