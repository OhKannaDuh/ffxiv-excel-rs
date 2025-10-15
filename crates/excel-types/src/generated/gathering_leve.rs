/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct GatheringLeve {
    pub row_id: u32,
    pub required_item_0_id: u32,
    pub required_item_0: RowRef<EventItem>,
    pub required_item_qty_0: u8,
    pub required_item_1_id: u32,
    pub required_item_1: RowRef<EventItem>,
    pub required_item_qty_1: u8,
    pub required_item_2_id: u32,
    pub required_item_2: RowRef<EventItem>,
    pub required_item_qty_2: u8,
    pub required_item_3_id: u32,
    pub required_item_3: RowRef<EventItem>,
    pub required_item_qty_3: u8,
    pub item_number: u8,
    pub rule_id: u32,
    pub rule: RowRef<GatheringLeveRule>,
    pub varient: u8,
    pub objective_0: u16,
    pub objective_1: u16,
    pub b_npc_entry_id: u32,
    pub use_secondary_tool: bool,
}

impl Sheet for GatheringLeve {
    const SHEET_NAME: &'static str = "GatheringLeve";
}

impl FromExcelRow for GatheringLeve {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            required_item_0_id: single_row.columns.get(4).to_u32(),
            required_item_0: RowRef::<EventItem>::from(single_row.columns.get(4).to_u32()),
            required_item_qty_0: single_row.columns.get(5).to_u8(),
            required_item_1_id: single_row.columns.get(6).to_u32(),
            required_item_1: RowRef::<EventItem>::from(single_row.columns.get(6).to_u32()),
            required_item_qty_1: single_row.columns.get(7).to_u8(),
            required_item_2_id: single_row.columns.get(8).to_u32(),
            required_item_2: RowRef::<EventItem>::from(single_row.columns.get(8).to_u32()),
            required_item_qty_2: single_row.columns.get(9).to_u8(),
            required_item_3_id: single_row.columns.get(10).to_u32(),
            required_item_3: RowRef::<EventItem>::from(single_row.columns.get(10).to_u32()),
            required_item_qty_3: single_row.columns.get(11).to_u8(),
            item_number: single_row.columns.get(12).to_u8(),
            rule_id: single_row.columns.get(13).to_u32(),
            rule: RowRef::<GatheringLeveRule>::from(single_row.columns.get(13).to_u32()),
            varient: single_row.columns.get(14).to_u8(),
            objective_0: single_row.columns.get(15).to_u16(),
            objective_1: single_row.columns.get(16).to_u16(),
            b_npc_entry_id: single_row.columns.get(17).to_u32(),
            use_secondary_tool: single_row.columns.get(18).to_bit(0),
        })
    }
}

