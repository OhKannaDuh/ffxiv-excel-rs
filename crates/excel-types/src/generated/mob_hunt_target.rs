/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct MobHuntTarget {
    pub row_id: u32,
    pub name_id: u32,
    pub name: RowRef<BNpcName>,
    pub fate_id: u32,
    pub fate: RowRef<Fate>,
    pub icon_id: u32,
    pub territory_type_id: u32,
    pub territory_type: RowRef<Map>,
    pub place_name_id: u32,
    pub place_name: RowRef<PlaceName>,
}

impl Sheet for MobHuntTarget {
    const SHEET_NAME: &'static str = "MobHuntTarget";
}

impl FromExcelRow for MobHuntTarget {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            name_id: single_row.columns.get(0).to_u32(),
            name: RowRef::<BNpcName>::from(single_row.columns.get(0).to_u32()),
            fate_id: single_row.columns.get(1).to_u32(),
            fate: RowRef::<Fate>::from(single_row.columns.get(1).to_u32()),
            icon_id: single_row.columns.get(2).to_u32(),
            territory_type_id: single_row.columns.get(3).to_u32(),
            territory_type: RowRef::<Map>::from(single_row.columns.get(3).to_u32()),
            place_name_id: single_row.columns.get(4).to_u32(),
            place_name: RowRef::<PlaceName>::from(single_row.columns.get(4).to_u32()),
        })
    }
}

