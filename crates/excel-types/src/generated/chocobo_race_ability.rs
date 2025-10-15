/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct ChocoboRaceAbility {
    pub row_id: u32,
    pub name: String,
    pub description: String,
    pub icon_id: u32,
    pub chocobo_race_ability_type_id: u32,
    pub chocobo_race_ability_type: RowRef<ChocoboRaceAbilityType>,
    pub value: u8,
}

impl Sheet for ChocoboRaceAbility {
    const SHEET_NAME: &'static str = "ChocoboRaceAbility";
}

impl FromExcelRow for ChocoboRaceAbility {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            name: single_row.columns.get(0).to_owned_string(),
            description: single_row.columns.get(1).to_owned_string(),
            icon_id: single_row.columns.get(2).to_u32(),
            chocobo_race_ability_type_id: single_row.columns.get(3).to_u32(),
            chocobo_race_ability_type: RowRef::<ChocoboRaceAbilityType>::from(single_row.columns.get(3).to_u32()),
            value: single_row.columns.get(4).to_u8(),
        })
    }
}

