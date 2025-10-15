/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct ActionTimeline {
    pub row_id: u32,
    pub _type: u8,
    pub priority: u8,
    pub pause: bool,
    pub stance: u8,
    pub slot: u8,
    pub look_at_mode: u8,
    pub key: String,
    pub action_timeline_id_mode: u8,
    pub weapon_timeline_id: u32,
    pub weapon_timeline: RowRef<WeaponTimeline>,
    pub load_type: u8,
    pub start_attach: u8,
    pub resident_pap: u8,
    pub kill_upper: bool,
    pub is_motion_canceled_by_moving: bool,
    pub is_loop: bool,
}

impl Sheet for ActionTimeline {
    const SHEET_NAME: &'static str = "ActionTimeline";
}

impl FromExcelRow for ActionTimeline {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            _type: single_row.columns.get(0).to_u8(),
            priority: single_row.columns.get(1).to_u8(),
            pause: single_row.columns.get(2).to_bit(0),
            stance: single_row.columns.get(3).to_u8(),
            slot: single_row.columns.get(4).to_u8(),
            look_at_mode: single_row.columns.get(5).to_u8(),
            key: single_row.columns.get(6).to_owned_string(),
            action_timeline_id_mode: single_row.columns.get(7).to_u8(),
            weapon_timeline_id: single_row.columns.get(8).to_u32(),
            weapon_timeline: RowRef::<WeaponTimeline>::from(single_row.columns.get(8).to_u32()),
            load_type: single_row.columns.get(9).to_u8(),
            start_attach: single_row.columns.get(10).to_u8(),
            resident_pap: single_row.columns.get(11).to_u8(),
            kill_upper: single_row.columns.get(13).to_bit(1),
            is_motion_canceled_by_moving: single_row.columns.get(14).to_bit(2),
            is_loop: single_row.columns.get(18).to_bit(5),
        })
    }
}

