/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct ChocoboRaceTerritory {
    pub row_id: u32,
    pub name_id: u32,
    pub name: RowRef<GoldSaucerTextData>,
    pub icon_id: u32,
}

impl Sheet for ChocoboRaceTerritory {
    const SHEET_NAME: &'static str = "ChocoboRaceTerritory";
}

impl FromExcelRow for ChocoboRaceTerritory {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            name_id: single_row.columns.get(0).to_u32(),
            name: RowRef::<GoldSaucerTextData>::from(single_row.columns.get(0).to_u32()),
            icon_id: single_row.columns.get(1).to_u32(),
        })
    }
}

