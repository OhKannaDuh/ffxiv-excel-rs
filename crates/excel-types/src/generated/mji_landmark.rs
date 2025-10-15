/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct MJILandmark {
    pub row_id: u32,
    pub sgb_0_id: u32,
    pub sgb_0: RowRef<ExportedSG>,
    pub sgb_1_id: u32,
    pub sgb_1: RowRef<ExportedSG>,
    pub sgb_2_id: u32,
    pub sgb_2: RowRef<ExportedSG>,
    pub sgb_3_id: u32,
    pub sgb_3: RowRef<ExportedSG>,
    pub sgb_4_id: u32,
    pub sgb_4: RowRef<ExportedSG>,
    pub sgb_5_id: u32,
    pub sgb_5: RowRef<ExportedSG>,
    pub sgb_6_id: u32,
    pub sgb_6: RowRef<ExportedSG>,
    pub name_id: u32,
    pub name: RowRef<MJIText>,
    pub icon_id: u32,
}

impl Sheet for MJILandmark {
    const SHEET_NAME: &'static str = "MJILandmark";
}

impl FromExcelRow for MJILandmark {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            sgb_0_id: single_row.columns.get(3).to_u32(),
            sgb_0: RowRef::<ExportedSG>::from(single_row.columns.get(3).to_u32()),
            sgb_1_id: single_row.columns.get(4).to_u32(),
            sgb_1: RowRef::<ExportedSG>::from(single_row.columns.get(4).to_u32()),
            sgb_2_id: single_row.columns.get(5).to_u32(),
            sgb_2: RowRef::<ExportedSG>::from(single_row.columns.get(5).to_u32()),
            sgb_3_id: single_row.columns.get(7).to_u32(),
            sgb_3: RowRef::<ExportedSG>::from(single_row.columns.get(7).to_u32()),
            sgb_4_id: single_row.columns.get(9).to_u32(),
            sgb_4: RowRef::<ExportedSG>::from(single_row.columns.get(9).to_u32()),
            sgb_5_id: single_row.columns.get(11).to_u32(),
            sgb_5: RowRef::<ExportedSG>::from(single_row.columns.get(11).to_u32()),
            sgb_6_id: single_row.columns.get(13).to_u32(),
            sgb_6: RowRef::<ExportedSG>::from(single_row.columns.get(13).to_u32()),
            name_id: single_row.columns.get(30).to_u32(),
            name: RowRef::<MJIText>::from(single_row.columns.get(30).to_u32()),
            icon_id: single_row.columns.get(32).to_u32(),
        })
    }
}

