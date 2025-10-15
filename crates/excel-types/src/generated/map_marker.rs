/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct MapMarker {
    pub row_id: u32,
    pub x: i16,
    pub y: i16,
    pub icon_id: u32,
    pub place_name_subtext_id: u32,
    pub place_name_subtext: RowRef<PlaceName>,
    pub subtext_orientation: u8,
    pub map_marker_region_id: u32,
    pub map_marker_region: RowRef<MapMarkerRegion>,
    pub _type: u8,
    pub data_type: u8,
    pub data_key_id: u32,
}

impl Sheet for MapMarker {
    const SHEET_NAME: &'static str = "MapMarker";
}

impl FromExcelRow for MapMarker {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            x: single_row.columns.get(0).to_i16(),
            y: single_row.columns.get(1).to_i16(),
            icon_id: single_row.columns.get(2).to_u32(),
            place_name_subtext_id: single_row.columns.get(3).to_u32(),
            place_name_subtext: RowRef::<PlaceName>::from(single_row.columns.get(3).to_u32()),
            subtext_orientation: single_row.columns.get(4).to_u8(),
            map_marker_region_id: single_row.columns.get(5).to_u32(),
            map_marker_region: RowRef::<MapMarkerRegion>::from(single_row.columns.get(5).to_u32()),
            _type: single_row.columns.get(6).to_u8(),
            data_type: single_row.columns.get(7).to_u8(),
            data_key_id: single_row.columns.get(8).to_u32(),
        })
    }
}

