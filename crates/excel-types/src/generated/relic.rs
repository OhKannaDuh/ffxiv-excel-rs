/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct Relic {
    pub row_id: u32,
    pub item_atma_id: u32,
    pub item_atma: RowRef<Item>,
    pub item_animus_id: u32,
    pub item_animus: RowRef<Item>,
    pub icon_id: u32,
    pub materia_0_id: u32,
    pub materia_0: RowRef<Materia>,
    pub note_main_0_id: u32,
    pub note_main_0: RowRef<RelicNote>,
    pub note_sub_0_id: u32,
    pub note_sub_0: RowRef<RelicNote>,
    pub note_selection_1_0_id: u32,
    pub note_selection_1_0: RowRef<RelicNote>,
    pub materia_1_id: u32,
    pub materia_1: RowRef<Materia>,
    pub note_main_1_id: u32,
    pub note_main_1: RowRef<RelicNote>,
    pub note_sub_1_id: u32,
    pub note_sub_1: RowRef<RelicNote>,
    pub note_selection_1_id: u32,
    pub note_selection_1: RowRef<RelicNote>,
    pub materia_2_id: u32,
    pub materia_2: RowRef<Materia>,
    pub note_main_2_id: u32,
    pub note_main_2: RowRef<RelicNote>,
    pub note_sub_2_id: u32,
    pub note_sub_2: RowRef<RelicNote>,
    pub materia_3_id: u32,
    pub materia_3: RowRef<Materia>,
    pub note_selection_3_id: u32,
    pub note_selection_3: RowRef<RelicNote>,
}

impl Sheet for Relic {
    const SHEET_NAME: &'static str = "Relic";
}

impl FromExcelRow for Relic {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            item_atma_id: single_row.columns.get(0).to_u32(),
            item_atma: RowRef::<Item>::from(single_row.columns.get(0).to_u32()),
            item_animus_id: single_row.columns.get(1).to_u32(),
            item_animus: RowRef::<Item>::from(single_row.columns.get(1).to_u32()),
            icon_id: single_row.columns.get(2).to_u32(),
            materia_0_id: single_row.columns.get(3).to_u32(),
            materia_0: RowRef::<Materia>::from(single_row.columns.get(3).to_u32()),
            note_main_0_id: single_row.columns.get(4).to_u32(),
            note_main_0: RowRef::<RelicNote>::from(single_row.columns.get(4).to_u32()),
            note_sub_0_id: single_row.columns.get(5).to_u32(),
            note_sub_0: RowRef::<RelicNote>::from(single_row.columns.get(5).to_u32()),
            note_selection_1_0_id: single_row.columns.get(6).to_u32(),
            note_selection_1_0: RowRef::<RelicNote>::from(single_row.columns.get(6).to_u32()),
            materia_1_id: single_row.columns.get(7).to_u32(),
            materia_1: RowRef::<Materia>::from(single_row.columns.get(7).to_u32()),
            note_main_1_id: single_row.columns.get(8).to_u32(),
            note_main_1: RowRef::<RelicNote>::from(single_row.columns.get(8).to_u32()),
            note_sub_1_id: single_row.columns.get(9).to_u32(),
            note_sub_1: RowRef::<RelicNote>::from(single_row.columns.get(9).to_u32()),
            note_selection_1_id: single_row.columns.get(10).to_u32(),
            note_selection_1: RowRef::<RelicNote>::from(single_row.columns.get(10).to_u32()),
            materia_2_id: single_row.columns.get(11).to_u32(),
            materia_2: RowRef::<Materia>::from(single_row.columns.get(11).to_u32()),
            note_main_2_id: single_row.columns.get(12).to_u32(),
            note_main_2: RowRef::<RelicNote>::from(single_row.columns.get(12).to_u32()),
            note_sub_2_id: single_row.columns.get(13).to_u32(),
            note_sub_2: RowRef::<RelicNote>::from(single_row.columns.get(13).to_u32()),
            materia_3_id: single_row.columns.get(14).to_u32(),
            materia_3: RowRef::<Materia>::from(single_row.columns.get(14).to_u32()),
            note_selection_3_id: single_row.columns.get(15).to_u32(),
            note_selection_3: RowRef::<RelicNote>::from(single_row.columns.get(15).to_u32()),
        })
    }
}

