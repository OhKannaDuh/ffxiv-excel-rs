/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct IKDSpot {
    pub row_id: u32,
    pub spot_main_id: u32,
    pub spot_main: RowRef<FishingSpot>,
    pub spot_sub_id: u32,
    pub spot_sub: RowRef<FishingSpot>,
    pub place_name_id: u32,
    pub place_name: RowRef<PlaceName>,
}

impl Sheet for IKDSpot {
    const SHEET_NAME: &'static str = "IKDSpot";
}

impl FromExcelRow for IKDSpot {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            spot_main_id: single_row.columns.get(0).to_u32(),
            spot_main: RowRef::<FishingSpot>::from(single_row.columns.get(0).to_u32()),
            spot_sub_id: single_row.columns.get(1).to_u32(),
            spot_sub: RowRef::<FishingSpot>::from(single_row.columns.get(1).to_u32()),
            place_name_id: single_row.columns.get(2).to_u32(),
            place_name: RowRef::<PlaceName>::from(single_row.columns.get(2).to_u32()),
        })
    }
}

