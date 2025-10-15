/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct MapSymbol {
    pub row_id: u32,
    pub icon_id: u32,
    pub place_name_id: u32,
    pub place_name: RowRef<PlaceName>,
    pub display_navi: bool,
}

impl Sheet for MapSymbol {
    const SHEET_NAME: &'static str = "MapSymbol";
}

impl FromExcelRow for MapSymbol {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            icon_id: single_row.columns.get(0).to_u32(),
            place_name_id: single_row.columns.get(1).to_u32(),
            place_name: RowRef::<PlaceName>::from(single_row.columns.get(1).to_u32()),
            display_navi: single_row.columns.get(2).to_bit(0),
        })
    }
}

