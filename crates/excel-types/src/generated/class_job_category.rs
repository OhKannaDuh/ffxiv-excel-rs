/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct ClassJobCategory {
    pub row_id: u32,
    pub name: String,
    pub adv: bool,
    pub gla: bool,
    pub pgl: bool,
    pub mrd: bool,
    pub lnc: bool,
    pub arc: bool,
    pub cnj: bool,
    pub thm: bool,
    pub crp: bool,
    pub bsm: bool,
    pub arm: bool,
    pub gsm: bool,
    pub ltw: bool,
    pub wvr: bool,
    pub alc: bool,
    pub cul: bool,
    pub min: bool,
    pub btn: bool,
    pub fsh: bool,
    pub pld: bool,
    pub mnk: bool,
    pub war: bool,
    pub drg: bool,
    pub brd: bool,
    pub whm: bool,
    pub blm: bool,
    pub acn: bool,
    pub smn: bool,
    pub sch: bool,
    pub rog: bool,
    pub nin: bool,
    pub mch: bool,
    pub drk: bool,
    pub ast: bool,
    pub sam: bool,
    pub rdm: bool,
    pub blu: bool,
    pub gnb: bool,
    pub dnc: bool,
    pub rpr: bool,
    pub sge: bool,
    pub vpr: bool,
    pub pct: bool,
}

impl Sheet for ClassJobCategory {
    const SHEET_NAME: &'static str = "ClassJobCategory";
}

impl FromExcelRow for ClassJobCategory {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            name: single_row.columns.get(0).to_owned_string(),
            adv: single_row.columns.get(1).to_bool(),
            gla: single_row.columns.get(2).to_bool(),
            pgl: single_row.columns.get(3).to_bool(),
            mrd: single_row.columns.get(4).to_bool(),
            lnc: single_row.columns.get(5).to_bool(),
            arc: single_row.columns.get(6).to_bool(),
            cnj: single_row.columns.get(7).to_bool(),
            thm: single_row.columns.get(8).to_bool(),
            crp: single_row.columns.get(9).to_bool(),
            bsm: single_row.columns.get(10).to_bool(),
            arm: single_row.columns.get(11).to_bool(),
            gsm: single_row.columns.get(12).to_bool(),
            ltw: single_row.columns.get(13).to_bool(),
            wvr: single_row.columns.get(14).to_bool(),
            alc: single_row.columns.get(15).to_bool(),
            cul: single_row.columns.get(16).to_bool(),
            min: single_row.columns.get(17).to_bool(),
            btn: single_row.columns.get(18).to_bool(),
            fsh: single_row.columns.get(19).to_bool(),
            pld: single_row.columns.get(20).to_bool(),
            mnk: single_row.columns.get(21).to_bool(),
            war: single_row.columns.get(22).to_bool(),
            drg: single_row.columns.get(23).to_bool(),
            brd: single_row.columns.get(24).to_bool(),
            whm: single_row.columns.get(25).to_bool(),
            blm: single_row.columns.get(26).to_bool(),
            acn: single_row.columns.get(27).to_bool(),
            smn: single_row.columns.get(28).to_bool(),
            sch: single_row.columns.get(29).to_bool(),
            rog: single_row.columns.get(30).to_bool(),
            nin: single_row.columns.get(31).to_bool(),
            mch: single_row.columns.get(32).to_bool(),
            drk: single_row.columns.get(33).to_bool(),
            ast: single_row.columns.get(34).to_bool(),
            sam: single_row.columns.get(35).to_bool(),
            rdm: single_row.columns.get(36).to_bool(),
            blu: single_row.columns.get(37).to_bool(),
            gnb: single_row.columns.get(38).to_bool(),
            dnc: single_row.columns.get(39).to_bool(),
            rpr: single_row.columns.get(40).to_bool(),
            sge: single_row.columns.get(41).to_bool(),
            vpr: single_row.columns.get(42).to_bool(),
            pct: single_row.columns.get(43).to_bool(),
        })
    }
}

