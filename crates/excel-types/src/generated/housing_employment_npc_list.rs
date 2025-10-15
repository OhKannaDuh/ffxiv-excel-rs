/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct HousingEmploymentNpcList {
    pub row_id: u32,
    pub race_id: u32,
    pub race: RowRef<HousingEmploymentNpcRace>,
}

impl Sheet for HousingEmploymentNpcList {
    const SHEET_NAME: &'static str = "HousingEmploymentNpcList";
}

impl FromExcelRow for HousingEmploymentNpcList {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            race_id: single_row.columns.get(0).to_u32(),
            race: RowRef::<HousingEmploymentNpcRace>::from(single_row.columns.get(0).to_u32()),
        })
    }
}

