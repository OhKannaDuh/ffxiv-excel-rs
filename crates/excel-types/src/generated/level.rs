/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct Level {
    pub row_id: u32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub yaw: f32,
    pub radius: f32,
    pub _type: u8,
    pub object_id: u32,
    pub map_id: u32,
    pub map: RowRef<Map>,
    pub event_id_id: u32,
    pub event_id_triple_triad: RowRef<TripleTriad>,
    pub event_id_adventure: RowRef<Adventure>,
    pub event_id_opening: RowRef<Opening>,
    pub event_id_quest: RowRef<Quest>,
    pub territory_id: u32,
    pub territory: RowRef<TerritoryType>,
}

impl Sheet for Level {
    const SHEET_NAME: &'static str = "Level";
}

impl FromExcelRow for Level {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            x: single_row.columns.get(0).to_f32(),
            y: single_row.columns.get(1).to_f32(),
            z: single_row.columns.get(2).to_f32(),
            yaw: single_row.columns.get(3).to_f32(),
            radius: single_row.columns.get(4).to_f32(),
            _type: single_row.columns.get(5).to_u8(),
            object_id: single_row.columns.get(6).to_u32(),
            map_id: single_row.columns.get(7).to_u32(),
            map: RowRef::<Map>::from(single_row.columns.get(7).to_u32()),
            event_id_id: single_row.columns.get(8).to_u32(),
            event_id_triple_triad: RowRef::<TripleTriad>::from(single_row.columns.get(8).to_u32()),
            event_id_adventure: RowRef::<Adventure>::from(single_row.columns.get(8).to_u32()),
            event_id_opening: RowRef::<Opening>::from(single_row.columns.get(8).to_u32()),
            event_id_quest: RowRef::<Quest>::from(single_row.columns.get(8).to_u32()),
            territory_id: single_row.columns.get(9).to_u32(),
            territory: RowRef::<TerritoryType>::from(single_row.columns.get(9).to_u32()),
        })
    }
}

