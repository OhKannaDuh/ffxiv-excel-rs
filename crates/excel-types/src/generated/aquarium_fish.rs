/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct AquariumFish {
    pub row_id: u32,
    pub aquarium_water_id: u32,
    pub aquarium_water: RowRef<AquariumWater>,
    pub size: u8,
    pub item_id: u32,
    pub item: RowRef<Item>,
}

impl Sheet for AquariumFish {
    const SHEET_NAME: &'static str = "AquariumFish";
}

impl FromExcelRow for AquariumFish {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            aquarium_water_id: single_row.columns.get(0).to_u32(),
            aquarium_water: RowRef::<AquariumWater>::from(single_row.columns.get(0).to_u32()),
            size: single_row.columns.get(1).to_u8(),
            item_id: single_row.columns.get(2).to_u32(),
            item: RowRef::<Item>::from(single_row.columns.get(2).to_u32()),
        })
    }
}

