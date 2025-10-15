/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct BuddySkill {
    pub row_id: u32,
    pub buddy_level: u8,
    pub is_active: bool,
    pub defender_id: u32,
    pub defender__trait: RowRef<Trait>,
    pub defender_action: RowRef<Action>,
    pub attacker_id: u32,
    pub attacker__trait: RowRef<Trait>,
    pub attacker_action: RowRef<Action>,
    pub healer_id: u32,
    pub healer__trait: RowRef<Trait>,
    pub healer_action: RowRef<Action>,
}

impl Sheet for BuddySkill {
    const SHEET_NAME: &'static str = "BuddySkill";
}

impl FromExcelRow for BuddySkill {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            buddy_level: single_row.columns.get(0).to_u8(),
            is_active: single_row.columns.get(1).to_bit(0),
            defender_id: single_row.columns.get(2).to_u32(),
            defender__trait: RowRef::<Trait>::from(single_row.columns.get(2).to_u32()),
            defender_action: RowRef::<Action>::from(single_row.columns.get(2).to_u32()),
            attacker_id: single_row.columns.get(3).to_u32(),
            attacker__trait: RowRef::<Trait>::from(single_row.columns.get(3).to_u32()),
            attacker_action: RowRef::<Action>::from(single_row.columns.get(3).to_u32()),
            healer_id: single_row.columns.get(4).to_u32(),
            healer__trait: RowRef::<Trait>::from(single_row.columns.get(4).to_u32()),
            healer_action: RowRef::<Action>::from(single_row.columns.get(4).to_u32()),
        })
    }
}

