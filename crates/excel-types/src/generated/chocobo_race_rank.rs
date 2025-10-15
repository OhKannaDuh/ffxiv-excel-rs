/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct ChocoboRaceRank {
    pub row_id: u32,
    pub rating_min: u16,
    pub rating_max: u16,
    pub name_id: u32,
    pub name: RowRef<GoldSaucerTextData>,
    pub fee: u16,
    pub icon_id: u32,
}

impl Sheet for ChocoboRaceRank {
    const SHEET_NAME: &'static str = "ChocoboRaceRank";
}

impl FromExcelRow for ChocoboRaceRank {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            rating_min: single_row.columns.get(0).to_u16(),
            rating_max: single_row.columns.get(1).to_u16(),
            name_id: single_row.columns.get(2).to_u32(),
            name: RowRef::<GoldSaucerTextData>::from(single_row.columns.get(2).to_u32()),
            fee: single_row.columns.get(3).to_u16(),
            icon_id: single_row.columns.get(4).to_u32(),
        })
    }
}

