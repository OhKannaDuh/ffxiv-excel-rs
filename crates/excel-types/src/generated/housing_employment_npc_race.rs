/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct HousingEmploymentNpcRace {
    pub row_id: u32,
    pub race: String,
}

impl Sheet for HousingEmploymentNpcRace {
    const SHEET_NAME: &'static str = "HousingEmploymentNpcRace";
}

impl FromExcelRow for HousingEmploymentNpcRace {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            race: single_row.columns.get(0).to_owned_string(),
        })
    }
}

