/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct TripleTriadCardResident {
    pub row_id: u32,
    pub top: u8,
    pub bottom: u8,
    pub left: u8,
    pub right: u8,
    pub triple_triad_card_rarity_id: u32,
    pub triple_triad_card_rarity: RowRef<TripleTriadCardRarity>,
    pub triple_triad_card_type_id: u32,
    pub triple_triad_card_type: RowRef<TripleTriadCardType>,
    pub sale_value: u16,
    pub sort_key: u8,
    pub order: u16,
    pub ui_priority: u8,
    pub acquisition_type: u8,
    pub acquisition_id: u32,
    pub location_id: u32,
    pub quest_id: u32,
    pub quest: RowRef<Quest>,
}

impl Sheet for TripleTriadCardResident {
    const SHEET_NAME: &'static str = "TripleTriadCardResident";
}

impl FromExcelRow for TripleTriadCardResident {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            top: single_row.columns.get(1).to_u8(),
            bottom: single_row.columns.get(2).to_u8(),
            left: single_row.columns.get(3).to_u8(),
            right: single_row.columns.get(4).to_u8(),
            triple_triad_card_rarity_id: single_row.columns.get(5).to_u32(),
            triple_triad_card_rarity: RowRef::<TripleTriadCardRarity>::from(single_row.columns.get(5).to_u32()),
            triple_triad_card_type_id: single_row.columns.get(6).to_u32(),
            triple_triad_card_type: RowRef::<TripleTriadCardType>::from(single_row.columns.get(6).to_u32()),
            sale_value: single_row.columns.get(7).to_u16(),
            sort_key: single_row.columns.get(8).to_u8(),
            order: single_row.columns.get(9).to_u16(),
            ui_priority: single_row.columns.get(10).to_u8(),
            acquisition_type: single_row.columns.get(12).to_u8(),
            acquisition_id: single_row.columns.get(13).to_u32(),
            location_id: single_row.columns.get(14).to_u32(),
            quest_id: single_row.columns.get(15).to_u32(),
            quest: RowRef::<Quest>::from(single_row.columns.get(15).to_u32()),
        })
    }
}

