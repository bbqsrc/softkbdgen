use serde::Deserialize;

mod ir;
mod models;
pub use models::*;
mod ser;
pub use ser::ToXml;

#[derive(Debug, Clone, PartialEq, Deserialize, Default)]
pub struct Name {
    pub value: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Default)]
pub struct Map {
    pub iso: String,
    pub to: String,
    pub transform: Option<String>,
    #[serde(rename = "longPress")]
    pub long_press: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Default)]
pub struct KeyMap {
    #[serde(rename = "map")]
    pub keys: Vec<Map>,
    pub modifiers: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Default)]
pub struct Version {
    pub platform: String,
    pub number: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Default)]
pub struct Transform {
    pub from: String,
    pub to: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Default)]
pub struct Transforms {
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(rename = "transform")]
    pub values: Vec<Transform>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Default)]
pub struct Names {
    #[serde(rename = "name")]
    pub values: Vec<Name>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Default)]
pub struct Keyboard {
    pub locale: String,
    pub names: Vec<Names>,
    pub version: Version,
    #[serde(rename = "keyMap")]
    pub key_maps: Vec<KeyMap>,
    pub transforms: Option<Vec<Transforms>>,
    // <settings transformFailure="omit" transformPartial="hide"/>
}

use self::ir::{parse_modifiers, DesktopLayer, MobileLayer};
use crate::bundle::{
    key_map::{DesktopKeyMap, MobileKeyMap},
    keys,
    models::{DesktopModes, IsoKey, MobileModes, Mode},
};
use indexmap::IndexMap;

impl Keyboard {
    pub fn is_mobile(&self) -> bool {
        // TODO make this actually robust
        self.locale.contains("android")
    }

    pub fn mode_name(&self) -> &'static str {
        if self.locale.contains("android") {
            "mobile"
        } else if self.locale.contains("windows") {
            "win"
        } else if self.locale.contains("osx") {
            "mac"
        } else if self.locale.contains("chrome") {
            "chrome"
        } else {
            "unknown"
        }
    }

    pub fn to_mode(&self) -> Mode {
        if self.is_mobile() {
            Mode::Mobile(self.to_mobile_modes())
        } else {
            Mode::Desktop(self.to_desktop_modes())
        }
    }

    pub fn to_mobile_modes(&self) -> MobileModes {
        let mut out = IndexMap::new();

        for key_map in self.key_maps.iter() {
            let layer = MobileLayer::from(key_map);
            out.insert(
                layer.mode,
                MobileKeyMap(
                    layer
                        .keys
                        .iter()
                        .map(|row| {
                            row.iter()
                                .map(|key| keys::KeyValue::from(key.to_owned()))
                                .collect()
                        })
                        .collect(),
                ),
            );
        }

        MobileModes(out)
    }

    pub fn to_desktop_modes(&self) -> DesktopModes {
        use std::str::FromStr;

        let mut out = IndexMap::new();

        for key_map in self.key_maps.iter() {
            let mut keys = IndexMap::new();

            for map in key_map.keys.iter() {
                if let Ok(key) = RawIsoKey::from_str(&map.iso) {
                    keys.insert(key, map.to.clone());
                }
            }

            let mods = parse_modifiers(key_map.modifiers.as_ref());

            let layer = DesktopLayer::new(mods.clone(), keys);
            let mut keys_out: IndexMap<IsoKey, keys::KeyValue> = IndexMap::new();

            for (letter, n, value) in layer.iter() {
                let k = format!("{}{:02}", letter, n);
                if let Ok(v) = IsoKey::from_str(&k) {
                    keys_out.insert(v, keys::KeyValue::from(value.map(|s| s.into())));
                }
            }

            out.insert(mods, DesktopKeyMap(keys_out));
        }

        DesktopModes(out)
    }
}
