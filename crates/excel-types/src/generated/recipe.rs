/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct Recipe {
    pub row_id: u32,
    pub number: i32,
    pub craft_type_id: u32,
    pub craft_type: RowRef<CraftType>,
    pub recipe_level_table_id: u32,
    pub recipe_level_table: RowRef<RecipeLevelTable>,
    pub item_result_id: u32,
    pub item_result: RowRef<Item>,
    pub amount_result: u8,
    pub recipe_notebook_list_id: u32,
    pub recipe_notebook_list: RowRef<RecipeNotebookList>,
    pub display_priority: u16,
    pub is_secondary: bool,
    pub material_quality_factor: u8,
    pub difficulty_factor: u16,
    pub quality_factor: u16,
    pub durability_factor: u16,
    pub required_quality: u32,
    pub required_craftsmanship: u16,
    pub required_control: u16,
    pub quick_synth_craftsmanship: u16,
    pub quick_synth_control: u16,
    pub secret_recipe_book_id: u32,
    pub secret_recipe_book: RowRef<SecretRecipeBook>,
    pub quest_id: u32,
    pub quest: RowRef<Quest>,
    pub can_quick_synth: bool,
    pub can_hq: bool,
    pub exp_rewarded: bool,
    pub status_required_id: u32,
    pub status_required: RowRef<Status>,
    pub item_required_id: u32,
    pub item_required: RowRef<Item>,
    pub is_specialization_required: i32,
    pub is_expert: bool,
    pub patch_number: u16,
}

impl Sheet for Recipe {
    const SHEET_NAME: &'static str = "Recipe";
}

impl FromExcelRow for Recipe {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            number: single_row.columns.get(0).to_i32(),
            craft_type_id: single_row.columns.get(1).to_u32(),
            craft_type: RowRef::<CraftType>::from(single_row.columns.get(1).to_u32()),
            recipe_level_table_id: single_row.columns.get(2).to_u32(),
            recipe_level_table: RowRef::<RecipeLevelTable>::from(single_row.columns.get(2).to_u32()),
            item_result_id: single_row.columns.get(4).to_u32(),
            item_result: RowRef::<Item>::from(single_row.columns.get(4).to_u32()),
            amount_result: single_row.columns.get(5).to_u8(),
            recipe_notebook_list_id: single_row.columns.get(22).to_u32(),
            recipe_notebook_list: RowRef::<RecipeNotebookList>::from(single_row.columns.get(22).to_u32()),
            display_priority: single_row.columns.get(23).to_u16(),
            is_secondary: single_row.columns.get(24).to_bit(0),
            material_quality_factor: single_row.columns.get(25).to_u8(),
            difficulty_factor: single_row.columns.get(26).to_u16(),
            quality_factor: single_row.columns.get(27).to_u16(),
            durability_factor: single_row.columns.get(28).to_u16(),
            required_quality: single_row.columns.get(29).to_u32(),
            required_craftsmanship: single_row.columns.get(30).to_u16(),
            required_control: single_row.columns.get(31).to_u16(),
            quick_synth_craftsmanship: single_row.columns.get(32).to_u16(),
            quick_synth_control: single_row.columns.get(33).to_u16(),
            secret_recipe_book_id: single_row.columns.get(34).to_u32(),
            secret_recipe_book: RowRef::<SecretRecipeBook>::from(single_row.columns.get(34).to_u32()),
            quest_id: single_row.columns.get(35).to_u32(),
            quest: RowRef::<Quest>::from(single_row.columns.get(35).to_u32()),
            can_quick_synth: single_row.columns.get(36).to_bit(1),
            can_hq: single_row.columns.get(37).to_bit(2),
            exp_rewarded: single_row.columns.get(38).to_bit(3),
            status_required_id: single_row.columns.get(39).to_u32(),
            status_required: RowRef::<Status>::from(single_row.columns.get(39).to_u32()),
            item_required_id: single_row.columns.get(40).to_u32(),
            item_required: RowRef::<Item>::from(single_row.columns.get(40).to_u32()),
            is_specialization_required: single_row.columns.get(41).to_i32(),
            is_expert: single_row.columns.get(42).to_bit(5),
            patch_number: single_row.columns.get(45).to_u16(),
        })
    }
}

