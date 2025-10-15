/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct PetAction {
    pub row_id: u32,
    pub name: String,
    pub description: String,
    pub icon_id: u32,
    pub action_id: u32,
    pub action: RowRef<Action>,
    pub pet_id: u32,
    pub pet: RowRef<Pet>,
    pub master_order: bool,
    pub disable_order: bool,
}

impl Sheet for PetAction {
    const SHEET_NAME: &'static str = "PetAction";
}

impl FromExcelRow for PetAction {
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
            action_id: single_row.columns.get(3).to_u32(),
            action: RowRef::<Action>::from(single_row.columns.get(3).to_u32()),
            pet_id: single_row.columns.get(4).to_u32(),
            pet: RowRef::<Pet>::from(single_row.columns.get(4).to_u32()),
            master_order: single_row.columns.get(5).to_bit(0),
            disable_order: single_row.columns.get(6).to_bit(1),
        })
    }
}

