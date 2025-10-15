/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct DynamicEvent {
    pub row_id: u32,
    pub event_type_id: u32,
    pub event_type: RowRef<DynamicEventType>,
    pub enemy_type_id: u32,
    pub enemy_type: RowRef<DynamicEventEnemyType>,
    pub lgb_event_object: bool,
    pub lgb_map_range: u32,
    pub quest_id: u32,
    pub quest: RowRef<Quest>,
    pub single_battle_id: u32,
    pub single_battle: RowRef<DynamicEventSingleBattle>,
    pub announce_id: u32,
    pub announce: RowRef<LogMessage>,
    pub name: u32,
    pub description: String,
}

impl Sheet for DynamicEvent {
    const SHEET_NAME: &'static str = "DynamicEvent";
}

impl FromExcelRow for DynamicEvent {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            event_type_id: single_row.columns.get(0).to_u32(),
            event_type: RowRef::<DynamicEventType>::from(single_row.columns.get(0).to_u32()),
            enemy_type_id: single_row.columns.get(1).to_u32(),
            enemy_type: RowRef::<DynamicEventEnemyType>::from(single_row.columns.get(1).to_u32()),
            lgb_event_object: single_row.columns.get(4).to_bit(0),
            lgb_map_range: single_row.columns.get(5).to_u32(),
            quest_id: single_row.columns.get(6).to_u32(),
            quest: RowRef::<Quest>::from(single_row.columns.get(6).to_u32()),
            single_battle_id: single_row.columns.get(8).to_u32(),
            single_battle: RowRef::<DynamicEventSingleBattle>::from(single_row.columns.get(8).to_u32()),
            announce_id: single_row.columns.get(9).to_u32(),
            announce: RowRef::<LogMessage>::from(single_row.columns.get(9).to_u32()),
            name: single_row.columns.get(10).to_u32(),
            description: single_row.columns.get(11).to_owned_string(),
        })
    }
}

