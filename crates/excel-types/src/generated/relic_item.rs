/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct RelicItem {
    pub row_id: u32,
    pub gladiator_item_id: u32,
    pub gladiator_item: RowRef<Item>,
    pub pugilist_item_id: u32,
    pub pugilist_item: RowRef<Item>,
    pub marauder_item_id: u32,
    pub marauder_item: RowRef<Item>,
    pub lancer_item_id: u32,
    pub lancer_item: RowRef<Item>,
    pub archer_item_id: u32,
    pub archer_item: RowRef<Item>,
    pub conjurer_item_id: u32,
    pub conjurer_item: RowRef<Item>,
    pub thaumaturge_item_id: u32,
    pub thaumaturge_item: RowRef<Item>,
    pub arcanist_smn_item_id: u32,
    pub arcanist_smn_item: RowRef<Item>,
    pub arcanist_sch_item_id: u32,
    pub arcanist_sch_item: RowRef<Item>,
    pub shield_item_id: u32,
    pub shield_item: RowRef<Item>,
    pub rogue_item_id: u32,
    pub rogue_item: RowRef<Item>,
}

impl Sheet for RelicItem {
    const SHEET_NAME: &'static str = "RelicItem";
}

impl FromExcelRow for RelicItem {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            gladiator_item_id: single_row.columns.get(1).to_u32(),
            gladiator_item: RowRef::<Item>::from(single_row.columns.get(1).to_u32()),
            pugilist_item_id: single_row.columns.get(2).to_u32(),
            pugilist_item: RowRef::<Item>::from(single_row.columns.get(2).to_u32()),
            marauder_item_id: single_row.columns.get(3).to_u32(),
            marauder_item: RowRef::<Item>::from(single_row.columns.get(3).to_u32()),
            lancer_item_id: single_row.columns.get(4).to_u32(),
            lancer_item: RowRef::<Item>::from(single_row.columns.get(4).to_u32()),
            archer_item_id: single_row.columns.get(5).to_u32(),
            archer_item: RowRef::<Item>::from(single_row.columns.get(5).to_u32()),
            conjurer_item_id: single_row.columns.get(6).to_u32(),
            conjurer_item: RowRef::<Item>::from(single_row.columns.get(6).to_u32()),
            thaumaturge_item_id: single_row.columns.get(7).to_u32(),
            thaumaturge_item: RowRef::<Item>::from(single_row.columns.get(7).to_u32()),
            arcanist_smn_item_id: single_row.columns.get(8).to_u32(),
            arcanist_smn_item: RowRef::<Item>::from(single_row.columns.get(8).to_u32()),
            arcanist_sch_item_id: single_row.columns.get(9).to_u32(),
            arcanist_sch_item: RowRef::<Item>::from(single_row.columns.get(9).to_u32()),
            shield_item_id: single_row.columns.get(10).to_u32(),
            shield_item: RowRef::<Item>::from(single_row.columns.get(10).to_u32()),
            rogue_item_id: single_row.columns.get(11).to_u32(),
            rogue_item: RowRef::<Item>::from(single_row.columns.get(11).to_u32()),
        })
    }
}

