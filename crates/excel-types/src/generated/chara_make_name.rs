/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct CharaMakeName {
    pub row_id: u32,
    pub hyur_midlander_male: String,
    pub hyur_midlander_female: String,
    pub hyur_midlander_last_name: String,
    pub hyur_highlander_male: String,
    pub hyur_highlander_female: String,
    pub hyur_highlander_last_name: String,
    pub elezen_male: String,
    pub elezen_female: String,
    pub elezen_wildwood_last_name: String,
    pub elezen_duskwight_last_name: String,
    pub miqote_sun_male: String,
    pub miqote_sun_female: String,
    pub miqote_sun_male_last_name: String,
    pub miqote_sun_female_last_name: String,
    pub miqote_moon_male: String,
    pub miqote_moon_female: String,
    pub miqote_moon_lastname: String,
    pub lalafell_plainsfolk_first_name_start: String,
    pub lalafell_plainsfolk_last_name_start: String,
    pub lalafell_plainsfolk_end_of_names: String,
    pub lalafell_dunesfolk_male: String,
    pub lalafell_dunesfolk_male_last_name: String,
    pub lalafell_dunesfolk_female: String,
    pub lalafell_dunesfolk_female_last_name: String,
    pub roegadyn_sea_wolf_male: String,
    pub roegadyn_sea_wolf_male_last_name: String,
    pub roegadyn_sea_wolf_female: String,
    pub roegadyn_sea_wolf_female_last_name: String,
    pub roegadyn_hellsguard_first_name: String,
    pub roegadyn_hellsguard_male_last_name: String,
    pub roegadyn_hellsguard_female_last_name: String,
    pub au_ra_raen_male: String,
    pub au_ra_raen_female: String,
    pub au_ra_raen_last_name: String,
    pub au_ra_xaela_male: String,
    pub au_ra_xaela_female: String,
    pub au_ra_xaela_last_name: String,
    pub hrothgar_hellions_first_name: String,
    pub hrothgar_hellions_last_name: String,
    pub hrothgar_lost_first_name: String,
    pub hrothgar_lost_last_name: String,
    pub viera_first_name: String,
    pub viera_rava_last_name: String,
    pub viera_veena_last_name: String,
}

impl Sheet for CharaMakeName {
    const SHEET_NAME: &'static str = "CharaMakeName";
}

impl FromExcelRow for CharaMakeName {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            hyur_midlander_male: single_row.columns.get(0).to_owned_string(),
            hyur_midlander_female: single_row.columns.get(1).to_owned_string(),
            hyur_midlander_last_name: single_row.columns.get(2).to_owned_string(),
            hyur_highlander_male: single_row.columns.get(3).to_owned_string(),
            hyur_highlander_female: single_row.columns.get(4).to_owned_string(),
            hyur_highlander_last_name: single_row.columns.get(5).to_owned_string(),
            elezen_male: single_row.columns.get(6).to_owned_string(),
            elezen_female: single_row.columns.get(7).to_owned_string(),
            elezen_wildwood_last_name: single_row.columns.get(8).to_owned_string(),
            elezen_duskwight_last_name: single_row.columns.get(9).to_owned_string(),
            miqote_sun_male: single_row.columns.get(10).to_owned_string(),
            miqote_sun_female: single_row.columns.get(11).to_owned_string(),
            miqote_sun_male_last_name: single_row.columns.get(12).to_owned_string(),
            miqote_sun_female_last_name: single_row.columns.get(13).to_owned_string(),
            miqote_moon_male: single_row.columns.get(14).to_owned_string(),
            miqote_moon_female: single_row.columns.get(15).to_owned_string(),
            miqote_moon_lastname: single_row.columns.get(16).to_owned_string(),
            lalafell_plainsfolk_first_name_start: single_row.columns.get(17).to_owned_string(),
            lalafell_plainsfolk_last_name_start: single_row.columns.get(18).to_owned_string(),
            lalafell_plainsfolk_end_of_names: single_row.columns.get(19).to_owned_string(),
            lalafell_dunesfolk_male: single_row.columns.get(20).to_owned_string(),
            lalafell_dunesfolk_male_last_name: single_row.columns.get(21).to_owned_string(),
            lalafell_dunesfolk_female: single_row.columns.get(22).to_owned_string(),
            lalafell_dunesfolk_female_last_name: single_row.columns.get(23).to_owned_string(),
            roegadyn_sea_wolf_male: single_row.columns.get(24).to_owned_string(),
            roegadyn_sea_wolf_male_last_name: single_row.columns.get(25).to_owned_string(),
            roegadyn_sea_wolf_female: single_row.columns.get(26).to_owned_string(),
            roegadyn_sea_wolf_female_last_name: single_row.columns.get(27).to_owned_string(),
            roegadyn_hellsguard_first_name: single_row.columns.get(28).to_owned_string(),
            roegadyn_hellsguard_male_last_name: single_row.columns.get(29).to_owned_string(),
            roegadyn_hellsguard_female_last_name: single_row.columns.get(30).to_owned_string(),
            au_ra_raen_male: single_row.columns.get(31).to_owned_string(),
            au_ra_raen_female: single_row.columns.get(32).to_owned_string(),
            au_ra_raen_last_name: single_row.columns.get(33).to_owned_string(),
            au_ra_xaela_male: single_row.columns.get(34).to_owned_string(),
            au_ra_xaela_female: single_row.columns.get(35).to_owned_string(),
            au_ra_xaela_last_name: single_row.columns.get(36).to_owned_string(),
            hrothgar_hellions_first_name: single_row.columns.get(37).to_owned_string(),
            hrothgar_hellions_last_name: single_row.columns.get(38).to_owned_string(),
            hrothgar_lost_first_name: single_row.columns.get(39).to_owned_string(),
            hrothgar_lost_last_name: single_row.columns.get(40).to_owned_string(),
            viera_first_name: single_row.columns.get(47).to_owned_string(),
            viera_rava_last_name: single_row.columns.get(48).to_owned_string(),
            viera_veena_last_name: single_row.columns.get(49).to_owned_string(),
        })
    }
}

