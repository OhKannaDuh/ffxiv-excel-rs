/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct HousingExterior {
    pub row_id: u32,
    pub place_name_id: u32,
    pub place_name: RowRef<PlaceName>,
    pub housing_size: u8,
    pub model: String,
}

impl Sheet for HousingExterior {
    const SHEET_NAME: &'static str = "HousingExterior";
}

impl FromExcelRow for HousingExterior {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            place_name_id: single_row.columns.get(2).to_u32(),
            place_name: RowRef::<PlaceName>::from(single_row.columns.get(2).to_u32()),
            housing_size: single_row.columns.get(3).to_u8(),
            model: single_row.columns.get(4).to_owned_string(),
        })
    }
}

