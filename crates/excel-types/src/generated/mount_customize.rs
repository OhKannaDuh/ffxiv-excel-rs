/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct MountCustomize {
    pub row_id: u32,
    pub hyur_midlander_male_scale: u16,
    pub hyur_midlander_female_scale: u16,
    pub hyur_highlander_male_scale: u16,
    pub hyur_highlander_female_scale: u16,
    pub elezen_male_scale: u16,
    pub elezen_female_scale: u16,
    pub lala_male_scale: u16,
    pub lala_female_scale: u16,
    pub miqo_male_scale: u16,
    pub miqo_female_scale: u16,
    pub roe_male_scale: u16,
    pub roe_female_scale: u16,
    pub au_ra_male_scale: u16,
    pub au_ra_female_scale: u16,
    pub hrothgar_male_scale: u16,
    pub hrothgar_female_scale: u16,
    pub viera_male_scale: u16,
    pub viera_female_scale: u16,
    pub hyur_midlander_male_camera_height: u16,
    pub hyur_midlander_female_camera_height: u16,
    pub hyur_highlander_male_camera_height: u16,
    pub hyur_highlander_female_camera_height: u8,
    pub elezen_male_camera_height: u8,
    pub elezen_female_camera_height: u8,
    pub lala_male_camera_height: u8,
    pub lala_female_camera_height: u8,
    pub miqo_male_camera_height: u8,
    pub miqo_female_camera_height: u8,
    pub roe_male_camera_height: u8,
    pub roe_female_camera_height: u8,
    pub au_ra_male_camera_height: u8,
    pub au_ra_female_camera_height: u8,
    pub hrothgar_male_camera_height: u8,
    pub viera_male_camera_height: u8,
    pub viera_female_camera_height: u8,
}

impl Sheet for MountCustomize {
    const SHEET_NAME: &'static str = "MountCustomize";
}

impl FromExcelRow for MountCustomize {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            hyur_midlander_male_scale: single_row.columns.get(1).to_u16(),
            hyur_midlander_female_scale: single_row.columns.get(2).to_u16(),
            hyur_highlander_male_scale: single_row.columns.get(3).to_u16(),
            hyur_highlander_female_scale: single_row.columns.get(4).to_u16(),
            elezen_male_scale: single_row.columns.get(5).to_u16(),
            elezen_female_scale: single_row.columns.get(6).to_u16(),
            lala_male_scale: single_row.columns.get(7).to_u16(),
            lala_female_scale: single_row.columns.get(8).to_u16(),
            miqo_male_scale: single_row.columns.get(9).to_u16(),
            miqo_female_scale: single_row.columns.get(10).to_u16(),
            roe_male_scale: single_row.columns.get(11).to_u16(),
            roe_female_scale: single_row.columns.get(12).to_u16(),
            au_ra_male_scale: single_row.columns.get(13).to_u16(),
            au_ra_female_scale: single_row.columns.get(14).to_u16(),
            hrothgar_male_scale: single_row.columns.get(15).to_u16(),
            hrothgar_female_scale: single_row.columns.get(16).to_u16(),
            viera_male_scale: single_row.columns.get(17).to_u16(),
            viera_female_scale: single_row.columns.get(18).to_u16(),
            hyur_midlander_male_camera_height: single_row.columns.get(19).to_u16(),
            hyur_midlander_female_camera_height: single_row.columns.get(20).to_u16(),
            hyur_highlander_male_camera_height: single_row.columns.get(21).to_u16(),
            hyur_highlander_female_camera_height: single_row.columns.get(22).to_u8(),
            elezen_male_camera_height: single_row.columns.get(23).to_u8(),
            elezen_female_camera_height: single_row.columns.get(24).to_u8(),
            lala_male_camera_height: single_row.columns.get(25).to_u8(),
            lala_female_camera_height: single_row.columns.get(26).to_u8(),
            miqo_male_camera_height: single_row.columns.get(27).to_u8(),
            miqo_female_camera_height: single_row.columns.get(28).to_u8(),
            roe_male_camera_height: single_row.columns.get(29).to_u8(),
            roe_female_camera_height: single_row.columns.get(30).to_u8(),
            au_ra_male_camera_height: single_row.columns.get(31).to_u8(),
            au_ra_female_camera_height: single_row.columns.get(32).to_u8(),
            hrothgar_male_camera_height: single_row.columns.get(33).to_u8(),
            viera_male_camera_height: single_row.columns.get(34).to_u8(),
            viera_female_camera_height: single_row.columns.get(35).to_u8(),
        })
    }
}

