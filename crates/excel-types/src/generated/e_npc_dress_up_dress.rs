/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct ENpcDressUpDress {
    pub row_id: u32,
    pub e_npc_id: u32,
    pub e_npc: RowRef<ENpcResident>,
    pub behavior_id: u32,
    pub behavior: RowRef<Behavior>,
    pub model_main_hand: u64,
    pub dye_main_hand_id: u32,
    pub dye_main_hand: RowRef<Stain>,
    pub dye2_main_hand_id: u32,
    pub dye2_main_hand: RowRef<Stain>,
    pub model_off_hand: u64,
    pub dye_off_hand_id: u32,
    pub dye_off_hand: RowRef<Stain>,
    pub dye2_off_hand_id: u32,
    pub dye2_off_hand: RowRef<Stain>,
    pub model_head: u32,
    pub dye_head_id: u32,
    pub dye_head: RowRef<Stain>,
    pub dye2_head_id: u32,
    pub dye2_head: RowRef<Stain>,
    pub model_body: u32,
    pub dye_body_id: u32,
    pub dye_body: RowRef<Stain>,
    pub dye2_body_id: u32,
    pub dye2_body: RowRef<Stain>,
    pub model_hands: u32,
    pub dye_hands_id: u32,
    pub dye_hands: RowRef<Stain>,
    pub dye2_hands_id: u32,
    pub dye2_hands: RowRef<Stain>,
    pub model_legs: u32,
    pub dye_legs_id: u32,
    pub dye_legs: RowRef<Stain>,
    pub dye2_legs_id: u32,
    pub dye2_legs: RowRef<Stain>,
    pub model_feet: u32,
    pub dye_feet_id: u32,
    pub dye_feet: RowRef<Stain>,
    pub dye2_feet_id: u32,
    pub dye2_feet: RowRef<Stain>,
}

impl Sheet for ENpcDressUpDress {
    const SHEET_NAME: &'static str = "ENpcDressUpDress";
}

impl FromExcelRow for ENpcDressUpDress {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            e_npc_id: single_row.columns.get(7).to_u32(),
            e_npc: RowRef::<ENpcResident>::from(single_row.columns.get(7).to_u32()),
            behavior_id: single_row.columns.get(9).to_u32(),
            behavior: RowRef::<Behavior>::from(single_row.columns.get(9).to_u32()),
            model_main_hand: single_row.columns.get(37).to_u64(),
            dye_main_hand_id: single_row.columns.get(38).to_u32(),
            dye_main_hand: RowRef::<Stain>::from(single_row.columns.get(38).to_u32()),
            dye2_main_hand_id: single_row.columns.get(39).to_u32(),
            dye2_main_hand: RowRef::<Stain>::from(single_row.columns.get(39).to_u32()),
            model_off_hand: single_row.columns.get(40).to_u64(),
            dye_off_hand_id: single_row.columns.get(41).to_u32(),
            dye_off_hand: RowRef::<Stain>::from(single_row.columns.get(41).to_u32()),
            dye2_off_hand_id: single_row.columns.get(42).to_u32(),
            dye2_off_hand: RowRef::<Stain>::from(single_row.columns.get(42).to_u32()),
            model_head: single_row.columns.get(43).to_u32(),
            dye_head_id: single_row.columns.get(44).to_u32(),
            dye_head: RowRef::<Stain>::from(single_row.columns.get(44).to_u32()),
            dye2_head_id: single_row.columns.get(45).to_u32(),
            dye2_head: RowRef::<Stain>::from(single_row.columns.get(45).to_u32()),
            model_body: single_row.columns.get(46).to_u32(),
            dye_body_id: single_row.columns.get(47).to_u32(),
            dye_body: RowRef::<Stain>::from(single_row.columns.get(47).to_u32()),
            dye2_body_id: single_row.columns.get(48).to_u32(),
            dye2_body: RowRef::<Stain>::from(single_row.columns.get(48).to_u32()),
            model_hands: single_row.columns.get(49).to_u32(),
            dye_hands_id: single_row.columns.get(50).to_u32(),
            dye_hands: RowRef::<Stain>::from(single_row.columns.get(50).to_u32()),
            dye2_hands_id: single_row.columns.get(51).to_u32(),
            dye2_hands: RowRef::<Stain>::from(single_row.columns.get(51).to_u32()),
            model_legs: single_row.columns.get(52).to_u32(),
            dye_legs_id: single_row.columns.get(53).to_u32(),
            dye_legs: RowRef::<Stain>::from(single_row.columns.get(53).to_u32()),
            dye2_legs_id: single_row.columns.get(54).to_u32(),
            dye2_legs: RowRef::<Stain>::from(single_row.columns.get(54).to_u32()),
            model_feet: single_row.columns.get(55).to_u32(),
            dye_feet_id: single_row.columns.get(56).to_u32(),
            dye_feet: RowRef::<Stain>::from(single_row.columns.get(56).to_u32()),
            dye2_feet_id: single_row.columns.get(57).to_u32(),
            dye2_feet: RowRef::<Stain>::from(single_row.columns.get(57).to_u32()),
        })
    }
}

