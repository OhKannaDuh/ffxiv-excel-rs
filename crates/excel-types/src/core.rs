use physis::exd::{ColumnData, ExcelRow};
use std::{
    any::{Any, TypeId},
    collections::HashMap,
    marker::PhantomData,
};

pub trait FromExcelRow: Sized {
    fn from_row(row: &ExcelRow) -> Option<Self>;
}
pub trait Sheet: FromExcelRow + 'static {
    const SHEET_NAME: &'static str;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RowRef<T: FromExcelRow> {
    pub id: u32,
    _m: PhantomData<T>,
}
impl<T: FromExcelRow> From<u32> for RowRef<T> {
    fn from(id: u32) -> Self {
        Self {
            id,
            _m: PhantomData,
        }
    }
}
impl<T: FromExcelRow> RowRef<T> {
    pub fn is_null(&self) -> bool {
        self.id == 0
    }
}

#[derive(Debug, Clone, Copy)]
pub enum MultiRef2<A: FromExcelRow, B: FromExcelRow> {
    A(RowRef<A>),
    B(RowRef<B>),
    Unknown(u32),
}

pub struct DataManager {
    game_data: physis::gamedata::GameData,
    lang: physis::common::Language,
    stores: HashMap<TypeId, Box<dyn Any>>,
}

impl DataManager {
    pub fn new(game_data: physis::gamedata::GameData, lang: physis::common::Language) -> Self {
        Self {
            game_data,
            lang,
            stores: HashMap::new(),
        }
    }

    fn ensure_loaded<T: Sheet>(&mut self) -> Option<&SheetStore<T>> {
        let tid = TypeId::of::<T>();
        if !self.stores.contains_key(&tid) {
            let header = self.game_data.read_excel_sheet_header(T::SHEET_NAME)?;
            let lang = if header.languages.contains(&self.lang) {
                self.lang
            } else {
                physis::common::Language::None
            };
            let mut store = SheetStore::<T>::default();

            for (page_idx, _) in header.pages.iter().enumerate() {
                let exd =
                    self.game_data
                        .read_excel_sheet(T::SHEET_NAME, &header, lang, page_idx)?;
                for row in &exd.rows {
                    if let Some(v) = T::from_row(row) {
                        store.insert(row.row_id, v);
                    }
                }
            }
            self.stores.insert(tid, Box::new(store));
        }
        self.stores.get(&tid)?.downcast_ref::<SheetStore<T>>()
    }

    pub fn get_ref<T: Sheet>(&mut self, id: u32) -> Option<&T> {
        self.ensure_loaded::<T>()?.get(id)
    }
    pub fn get_all_ref<T: Sheet>(&mut self) -> Option<impl Iterator<Item = &T>> {
        Some(self.ensure_loaded::<T>()?.values())
    }
}

struct SheetStore<T> {
    by_id: HashMap<u32, T>,
}

impl<T> Default for SheetStore<T> {
    fn default() -> Self {
        Self {
            by_id: HashMap::new(),
        }
    }
}

impl<T> SheetStore<T> {
    fn insert(&mut self, id: u32, row: T) {
        self.by_id.insert(id, row);
    }
    fn get(&self, id: u32) -> Option<&T> {
        self.by_id.get(&id)
    }
    fn values(&self) -> impl Iterator<Item = &T> {
        self.by_id.values()
    }
}

pub trait ExcelColumnExt {
    fn to_owned_string(self) -> String;
    fn to_bool(self) -> bool;
    fn to_i8(self) -> i8;
    fn to_u8(self) -> u8;
    fn to_i16(self) -> i16;
    fn to_u16(self) -> u16;
    fn to_i32(self) -> i32;
    fn to_u32(self) -> u32;
    fn to_f32(self) -> f32;
    fn to_i64(self) -> i64;
    fn to_u64(self) -> u64;
    fn to_bit(self, bits: u8) -> bool;
}

impl ExcelColumnExt for Option<&ColumnData> {
    fn to_owned_string(self) -> String {
        match self {
            Some(ColumnData::String(s)) => s.clone(),
            _ => String::new(),
        }
    }

    fn to_bool(self) -> bool {
        match self {
            Some(ColumnData::Bool(b)) => *b,
            _ => false,
        }
    }

    fn to_u8(self) -> u8 {
        match self {
            Some(ColumnData::UInt8(v)) => *v,
            _ => 0,
        }
    }

    fn to_u16(self) -> u16 {
        match self {
            Some(ColumnData::UInt16(v)) => *v,
            _ => 0,
        }
    }

    fn to_u32(self) -> u32 {
        match self {
            Some(ColumnData::UInt32(v)) => *v,
            _ => 0,
        }
    }

    fn to_i32(self) -> i32 {
        match self {
            Some(ColumnData::Int32(v)) => *v,
            _ => 0,
        }
    }

    fn to_f32(self) -> f32 {
        match self {
            Some(ColumnData::Float32(v)) => *v,
            _ => 0.0,
        }
    }

    fn to_i64(self) -> i64 {
        match self {
            Some(ColumnData::Int64(v)) => *v,
            _ => 0,
        }
    }

    fn to_u64(self) -> u64 {
        match self {
            Some(ColumnData::UInt64(v)) => *v,
            _ => 0,
        }
    }

    fn to_i8(self) -> i8 {
        match self {
            Some(ColumnData::Int8(v)) => *v,
            _ => 0,
        }
    }

    fn to_i16(self) -> i16 {
        match self {
            Some(ColumnData::Int16(v)) => *v,
            _ => 0,
        }
    }

    fn to_bit(self, bits: u8) -> bool {
        match self {
            Some(ColumnData::UInt8(v)) => (v & (1 << bits)) != 0,
            _ => false,
        }
    }
}
