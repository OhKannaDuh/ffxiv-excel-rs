/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct SpearfishingNotebook {
    pub row_id: u32,
    pub gathering_level: u8,
    pub is_shadow_node: bool,
    pub territory_type_id: u32,
    pub territory_type: RowRef<TerritoryType>,
    pub x: i16,
    pub y: i16,
    pub radius: u16,
    pub place_name_id: u32,
    pub place_name: RowRef<PlaceName>,
    pub gathering_point_base_id: u32,
    pub gathering_point_base: RowRef<GatheringPointBase>,
}

impl Sheet for SpearfishingNotebook {
    const SHEET_NAME: &'static str = "SpearfishingNotebook";
}

impl FromExcelRow for SpearfishingNotebook {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            gathering_level: single_row.columns.get(0).to_u8(),
            is_shadow_node: single_row.columns.get(1).to_bit(0),
            territory_type_id: single_row.columns.get(2).to_u32(),
            territory_type: RowRef::<TerritoryType>::from(single_row.columns.get(2).to_u32()),
            x: single_row.columns.get(3).to_i16(),
            y: single_row.columns.get(4).to_i16(),
            radius: single_row.columns.get(5).to_u16(),
            place_name_id: single_row.columns.get(7).to_u32(),
            place_name: RowRef::<PlaceName>::from(single_row.columns.get(7).to_u32()),
            gathering_point_base_id: single_row.columns.get(9).to_u32(),
            gathering_point_base: RowRef::<GatheringPointBase>::from(single_row.columns.get(9).to_u32()),
        })
    }
}

