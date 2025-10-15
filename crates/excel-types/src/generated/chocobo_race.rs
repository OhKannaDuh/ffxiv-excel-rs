/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct ChocoboRace {
    pub row_id: u32,
    pub chocobo_race_rank_id: u32,
    pub chocobo_race_rank: RowRef<ChocoboRaceRank>,
    pub chocobo_race_territory_id: u32,
    pub chocobo_race_territory: RowRef<ChocoboRaceTerritory>,
}

impl Sheet for ChocoboRace {
    const SHEET_NAME: &'static str = "ChocoboRace";
}

impl FromExcelRow for ChocoboRace {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            chocobo_race_rank_id: single_row.columns.get(0).to_u32(),
            chocobo_race_rank: RowRef::<ChocoboRaceRank>::from(single_row.columns.get(0).to_u32()),
            chocobo_race_territory_id: single_row.columns.get(1).to_u32(),
            chocobo_race_territory: RowRef::<ChocoboRaceTerritory>::from(single_row.columns.get(1).to_u32()),
        })
    }
}

