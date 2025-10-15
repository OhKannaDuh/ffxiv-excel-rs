/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct CharaMakeType {
    pub row_id: u32,
    pub race_id: u32,
    pub race: RowRef<Race>,
    pub tribe_id: u32,
    pub tribe: RowRef<Tribe>,
    pub gender: i8,
}

impl Sheet for CharaMakeType {
    const SHEET_NAME: &'static str = "CharaMakeType";
}

impl FromExcelRow for CharaMakeType {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            race_id: single_row.columns.get(0).to_u32(),
            race: RowRef::<Race>::from(single_row.columns.get(0).to_u32()),
            tribe_id: single_row.columns.get(1).to_u32(),
            tribe: RowRef::<Tribe>::from(single_row.columns.get(1).to_u32()),
            gender: single_row.columns.get(2).to_i8(),
        })
    }
}

