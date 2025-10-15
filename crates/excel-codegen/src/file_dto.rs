use serde::Deserialize;

use crate::RESERVED_KEYWORDS;

#[derive(Deserialize, Debug)]
pub struct DefinitionFile {
    pub sheet: String,

    // #[serde(rename = "defaultColumn", default)]
    // pub default_column: String,
    pub definitions: Vec<Definition>,
}

#[derive(Deserialize, Debug)]
pub struct Definition {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub index: u32,
    pub converter: Option<Converter>,

    #[serde(rename = "type", default)]
    pub data_type: DefinitionType,
}

impl Definition {
    pub fn to_field_name(&self) -> String {
        to_field_name(self.name.clone())
    }
}

#[derive(Deserialize, Debug, Default, Eq, PartialEq)]
pub enum DefinitionType {
    #[default]
    None,
    #[serde(rename = "repeat")]
    Repeat,
}

pub fn to_field_name(subject: String) -> String {
    let mut s = subject;
    s = s.replace('{', "_").replace('}', "");
    s = s.replace('<', "_").replace('>', "");
    s = s.replace('[', "_").replace(']', "");
    s = s.replace('(', "_").replace(')', "");
    s = s.replace('-', "_");
    s = s.replace(':', "_");
    s = s.replace('%', "percent");
    s = s.replace(' ', "");
    s = s.replace('/', "");
    s = s.replace('\'', "");

    let mut out = String::new();
    let chars: Vec<char> = s.chars().collect();

    for (i, &ch) in chars.iter().enumerate() {
        if i > 0 && ch.is_uppercase() {
            let prev = chars[i - 1];
            let next = chars.get(i + 1).copied().unwrap_or('_');

            if (prev.is_lowercase() || prev.is_ascii_digit()) || next.is_lowercase() {
                out.push('_');
            }
        }
        out.push(ch);
    }

    let fieldname = out.replace("__", "_").to_lowercase();

    if RESERVED_KEYWORDS.contains(&fieldname.as_str()) {
        return format!("_{fieldname}");
    }

    if let Some(first_char) = fieldname.chars().next() {
        if first_char.is_ascii_digit() {
            return format!("_{}", fieldname);
        }
    }

    fieldname
}

#[derive(Deserialize, Debug)]
pub enum ConverterType {
    #[serde(rename = "link")]
    Link,
    #[serde(rename = "multiref")]
    MultiRef,
    #[serde(rename = "icon")]
    Icon,
    // Not handled right now
    #[serde(rename = "complexlink")]
    ComplexLink,
    #[serde(rename = "generic")]
    Generic,
    #[serde(rename = "color")]
    Color,
    #[serde(rename = "tomestone")]
    Tomestone,
}

#[derive(Deserialize, Debug)]
pub struct Converter {
    #[serde(rename = "type")]
    pub converter_type: ConverterType,
    pub target: Option<String>,
    pub targets: Option<Vec<String>>,
}
