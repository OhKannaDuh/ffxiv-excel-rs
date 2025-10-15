/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct BaseParam {
    pub row_id: u32,
    pub packet_index: i8,
    pub name: String,
    pub description: String,
    pub order_priority: u8,
    pub _1_h_wpnpercent: u16,
    pub oh_percent: u16,
    pub head_percent: u16,
    pub chest_percent: u16,
    pub hands_percent: u16,
    pub waist_percent: u16,
    pub legs_percent: u16,
    pub feet_percent: u16,
    pub earring_percent: u16,
    pub necklace_percent: u16,
    pub bracelet_percent: u16,
    pub ring_percent: u16,
    pub _2_h_wpn_percent: u16,
    pub under_armor_percent: u16,
    pub chest_head_percent: u16,
    pub chest_head_legs_feet_percent: u16,
    pub legs_feet_percent: u16,
    pub head_chest_hands_legs_feet_percent: u16,
    pub chest_legs_gloves_percent: u16,
    pub chest_legs_feet_percent: u16,
}

impl Sheet for BaseParam {
    const SHEET_NAME: &'static str = "BaseParam";
}

impl FromExcelRow for BaseParam {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            packet_index: single_row.columns.get(0).to_i8(),
            name: single_row.columns.get(1).to_owned_string(),
            description: single_row.columns.get(2).to_owned_string(),
            order_priority: single_row.columns.get(3).to_u8(),
            _1_h_wpnpercent: single_row.columns.get(4).to_u16(),
            oh_percent: single_row.columns.get(5).to_u16(),
            head_percent: single_row.columns.get(6).to_u16(),
            chest_percent: single_row.columns.get(7).to_u16(),
            hands_percent: single_row.columns.get(8).to_u16(),
            waist_percent: single_row.columns.get(9).to_u16(),
            legs_percent: single_row.columns.get(10).to_u16(),
            feet_percent: single_row.columns.get(11).to_u16(),
            earring_percent: single_row.columns.get(12).to_u16(),
            necklace_percent: single_row.columns.get(13).to_u16(),
            bracelet_percent: single_row.columns.get(14).to_u16(),
            ring_percent: single_row.columns.get(15).to_u16(),
            _2_h_wpn_percent: single_row.columns.get(16).to_u16(),
            under_armor_percent: single_row.columns.get(17).to_u16(),
            chest_head_percent: single_row.columns.get(18).to_u16(),
            chest_head_legs_feet_percent: single_row.columns.get(19).to_u16(),
            legs_feet_percent: single_row.columns.get(21).to_u16(),
            head_chest_hands_legs_feet_percent: single_row.columns.get(22).to_u16(),
            chest_legs_gloves_percent: single_row.columns.get(23).to_u16(),
            chest_legs_feet_percent: single_row.columns.get(24).to_u16(),
        })
    }
}

