/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct HousingAethernet {
    pub row_id: u32,
    pub level_id: u32,
    pub level: RowRef<Level>,
    pub territory_type_id: u32,
    pub territory_type: RowRef<TerritoryType>,
    pub place_name_id: u32,
    pub place_name: RowRef<PlaceName>,
    pub order: u8,
}

impl Sheet for HousingAethernet {
    const SHEET_NAME: &'static str = "HousingAethernet";
}

impl FromExcelRow for HousingAethernet {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            level_id: single_row.columns.get(0).to_u32(),
            level: RowRef::<Level>::from(single_row.columns.get(0).to_u32()),
            territory_type_id: single_row.columns.get(1).to_u32(),
            territory_type: RowRef::<TerritoryType>::from(single_row.columns.get(1).to_u32()),
            place_name_id: single_row.columns.get(2).to_u32(),
            place_name: RowRef::<PlaceName>::from(single_row.columns.get(2).to_u32()),
            order: single_row.columns.get(3).to_u8(),
        })
    }
}

