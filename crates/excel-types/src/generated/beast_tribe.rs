/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct BeastTribe {
    pub row_id: u32,
    pub min_level: u8,
    pub beast_rank_bonus_id: u32,
    pub beast_rank_bonus: RowRef<BeastRankBonus>,
    pub icon_reputation_id: u32,
    pub icon_id: u32,
    pub max_rank: u8,
    pub expansion_id: u32,
    pub expansion: RowRef<ExVersion>,
    pub currency_item_id: u32,
    pub currency_item: RowRef<Item>,
    pub display_order: u32,
    pub name: u8,
    pub adjective: String,
    pub plural: i8,
    pub possessive_pronoun: String,
    pub starts_with_vowel: i8,
    pub pronoun: i8,
    pub article: i8,
    pub def: i8,
    pub name_relation: i8,
}

impl Sheet for BeastTribe {
    const SHEET_NAME: &'static str = "BeastTribe";
}

impl FromExcelRow for BeastTribe {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            min_level: single_row.columns.get(1).to_u8(),
            beast_rank_bonus_id: single_row.columns.get(2).to_u32(),
            beast_rank_bonus: RowRef::<BeastRankBonus>::from(single_row.columns.get(2).to_u32()),
            icon_reputation_id: single_row.columns.get(3).to_u32(),
            icon_id: single_row.columns.get(4).to_u32(),
            max_rank: single_row.columns.get(5).to_u8(),
            expansion_id: single_row.columns.get(6).to_u32(),
            expansion: RowRef::<ExVersion>::from(single_row.columns.get(6).to_u32()),
            currency_item_id: single_row.columns.get(8).to_u32(),
            currency_item: RowRef::<Item>::from(single_row.columns.get(8).to_u32()),
            display_order: single_row.columns.get(9).to_u32(),
            name: single_row.columns.get(10).to_u8(),
            adjective: single_row.columns.get(11).to_owned_string(),
            plural: single_row.columns.get(12).to_i8(),
            possessive_pronoun: single_row.columns.get(13).to_owned_string(),
            starts_with_vowel: single_row.columns.get(14).to_i8(),
            pronoun: single_row.columns.get(15).to_i8(),
            article: single_row.columns.get(16).to_i8(),
            def: single_row.columns.get(17).to_i8(),
            name_relation: single_row.columns.get(18).to_i8(),
        })
    }
}

