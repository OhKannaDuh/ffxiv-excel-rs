/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct Story {
    pub row_id: u32,
    pub script: String,
    pub layer_set_territory_type_0_id: u32,
    pub layer_set_territory_type_0: RowRef<TerritoryType>,
    pub layer_set_territory_type_1_id: u32,
    pub layer_set_territory_type_1: RowRef<TerritoryType>,
}

impl Sheet for Story {
    const SHEET_NAME: &'static str = "Story";
}

impl FromExcelRow for Story {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            script: single_row.columns.get(0).to_owned_string(),
            layer_set_territory_type_0_id: single_row.columns.get(1861).to_u32(),
            layer_set_territory_type_0: RowRef::<TerritoryType>::from(single_row.columns.get(1861).to_u32()),
            layer_set_territory_type_1_id: single_row.columns.get(1862).to_u32(),
            layer_set_territory_type_1: RowRef::<TerritoryType>::from(single_row.columns.get(1862).to_u32()),
        })
    }
}

