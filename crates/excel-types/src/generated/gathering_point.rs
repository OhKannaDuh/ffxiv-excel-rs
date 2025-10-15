/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct GatheringPoint {
    pub row_id: u32,
    pub _type: u8,
    pub gathering_point_base_id: u32,
    pub gathering_point_base: RowRef<GatheringPointBase>,
    pub count: i32,
    pub territory_type_id: u32,
    pub territory_type: RowRef<TerritoryType>,
    pub place_name_id: u32,
    pub place_name: RowRef<PlaceName>,
    pub gathering_sub_category_id: u32,
    pub gathering_sub_category: RowRef<GatheringSubCategory>,
}

impl Sheet for GatheringPoint {
    const SHEET_NAME: &'static str = "GatheringPoint";
}

impl FromExcelRow for GatheringPoint {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            _type: single_row.columns.get(0).to_u8(),
            gathering_point_base_id: single_row.columns.get(2).to_u32(),
            gathering_point_base: RowRef::<GatheringPointBase>::from(single_row.columns.get(2).to_u32()),
            count: single_row.columns.get(3).to_i32(),
            territory_type_id: single_row.columns.get(6).to_u32(),
            territory_type: RowRef::<TerritoryType>::from(single_row.columns.get(6).to_u32()),
            place_name_id: single_row.columns.get(7).to_u32(),
            place_name: RowRef::<PlaceName>::from(single_row.columns.get(7).to_u32()),
            gathering_sub_category_id: single_row.columns.get(8).to_u32(),
            gathering_sub_category: RowRef::<GatheringSubCategory>::from(single_row.columns.get(8).to_u32()),
        })
    }
}

