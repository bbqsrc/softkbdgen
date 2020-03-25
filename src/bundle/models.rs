use crate::{DesktopKeyMap, MobileKeyMap};
use derive_collect_docs::CollectDocs;
use serde::{Deserialize, Serialize};
use serde_yaml as yaml;
use shrinkwraprs::Shrinkwrap;
use std::collections::BTreeMap;
use strum_macros::{Display, EnumIter, EnumString};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, CollectDocs)]
#[example(
    yaml,
    r#"
    name: "Tastatur"
    description: "Et testtastatur"
"#
)]
pub struct ProjectDesc {
    /// The name string for the project. For mobile keyboards, this is the title of the app.
    pub name: String,
    /// The description of the project.
    pub description: String,
}

/// Meta data for the project, stored in the `project.yaml` file.
#[example(
    yaml,
    r#"
locales:
  en:
    name: "Keyboard Project"
    description: "A test keyboard"
  nb:
    name: "Tastatur"
    description: "Et testtastatur"
author: Example Person
email: person@place.example
organisation: Example Corp
copyright: Copyright © 2017 Example Corpa
"#
)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default, CollectDocs)]
pub struct Project {
    /// Strings for describing the project.
    ///
    /// This is a map from a language code (ISO 639-1 or ISO 639-3) to a project
    /// description in that language. If a language has both an ISO 639-1 or ISO
    /// 639-3 code, prefer the 639-1 variant for better support.
    ///
    /// The project description must be defined for at least the `en` locale,
    /// and preferably also for each well-supported locale that you expect to
    /// support.
    #[example(
        yaml,
        "
        locales:
          en:
            name: My Keyboard Project
            description: A keyboard supporting zero languages.
    "
    )]
    pub locales: BTreeMap<String, ProjectDesc>,
    /// The primary author(s)
    pub author: String,
    /// One email address to contact the author(s) of the project
    pub email: String,
    /// The copyright string to be used where and if necessary.
    pub copyright: String,
    /// The associated organisation. Put author here too if no organisation.
    pub organisation: String,
}

/// Strings to be shown on some OSes
#[example(
    yaml,
    r#"
    space: space
    return: return
    "#
)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, CollectDocs)]
pub struct LayoutStrings {
    #[example(yaml, r#"space: gasska"#)]
    pub space: String,

    #[example(yaml, r#"return: linnjamålssom"#)]
    #[serde(rename = "return")]
    pub return_: String,
}

/// Derive options
// TODO: Add documentation
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, CollectDocs)]
pub struct DeriveOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transforms: Option<bool>,
}

/// ISO key codes
///
/// cf. <https://commons.wikimedia.org/wiki/File:Keyboard-sections-zones-grid-ISOIEC-9995-1.jpg>
/// and <https://commons.wikimedia.org/wiki/File:Keyboard-alphanumeric-section-ISOIEC-9995-2-2009-with-amd1-2012.png>
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[derive(EnumString, Display, EnumIter)]
#[derive(Serialize, Deserialize, CollectDocs)]
#[repr(u8)]
pub enum IsoKey {
    E00 = 0,
    E01,
    E02,
    E03,
    E04,
    E05,
    E06,
    E07,
    E08,
    E09,
    E10,
    E11,
    E12,
    D01,
    D02,
    D03,
    D04,
    D05,
    D06,
    D07,
    D08,
    D09,
    D10,
    D11,
    D12,
    C01,
    C02,
    C03,
    C04,
    C05,
    C06,
    C07,
    C08,
    C09,
    C10,
    C11,
    C12,
    B00,
    B01,
    B02,
    B03,
    B04,
    B05,
    B06,
    B07,
    B08,
    B09,
    B10,
}

/// US keyboard layout
static INDEX_TO_KEYCODE: &[u8] = br"`1234567890-=qwertyuiop[]asdfghjkl;'\`zxcvbnm,./";

impl IsoKey {
    /// Returns the X11 character code
    pub fn to_character_code(self) -> u8 {
        let index = self as u8;

        let c = if let Some(i) = INDEX_TO_KEYCODE.get(index as usize) {
            i
        } else {
            panic!(
                "character code map covers all ISO keys, but got {} for {}",
                index, self
            );
        };

        *c
    }

    /// Returns the X11 character code
    pub fn to_character(self) -> char {
        std::char::from_u32(u32::from(self.to_character_code())).expect("keycode is ascii")
    }
}

/// Target specific modes
///
/// This is a nested map with known keys (the fields below) for each supported
/// target.
///
/// NOTE: Each target is either described by <<DesktopModes>>, or by
/// <<MobileModes>>.
#[example(
    yaml,
    r#"
modes:
  mac:
    default: |
      ' 1 2 3 4 5 6 7 8 9 0 + ´
        á š e r t y u i o p å ŋ
        a s d f g h j k l ö ä đ
      ž z č c v b n m , . -
    shift: |
      § ! " # $ % & / ( ) = ? `
        Á Š E R T Y U I O P Å Ŋ
        A S D F G H J K L Ö Ä Đ
      Ž Z Č C V B N M ; : _
  win:
    default: |
      § 1 2 3 4 5 6 7 8 9 0 + ´
        á š e r t y u i o p å ŋ
        a s d f g h j k l ö ä đ
      ž z č c v b n m , . -
    shift: |
      ½ ! " # ¤ % & / ( ) = ? `
        Á Š E R T Y U I O P Å Ŋ
        A S D F G H J K L Ö Ä Đ
      Ž Z Č C V B N M ; : _
"#
)]
#[derive(Debug, Clone, PartialEq)]
#[derive(Serialize, Deserialize, Default, CollectDocs)]
pub struct Modes {
    /// Windows
    #[serde(skip_serializing_if = "Option::is_none")]
    pub win: Option<DesktopModes>,
    // macOS
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mac: Option<DesktopModes>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// iOS, used for both iPhone and iPad keyboards
    pub ios: Option<MobileModes>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub android: Option<MobileModes>,
    /// ChromeOS (used on Chrome Books)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chrome: Option<DesktopModes>,
    /// Linux (X11)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x11: Option<DesktopModes>,
    /// Desktop default mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desktop: Option<DesktopModes>,
    /// Mobile default mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobile: Option<MobileModes>,
}

impl Modes {
    pub fn available_modes(&self) -> Vec<String> {
        let mut res = Vec::new();
        if self.win.is_some() {
            res.push("win".into());
        }
        if self.mac.is_some() {
            res.push("mac".into());
        }
        if self.ios.is_some() {
            res.push("ios".into());
        }
        if self.android.is_some() {
            res.push("android".into());
        }
        if self.chrome.is_some() {
            res.push("chrome".into());
        }
        if self.x11.is_some() {
            res.push("x11".into());
        }
        if self.desktop.is_some() {
            res.push("desktop".into());
        }
        if self.mobile.is_some() {
            res.push("mobile".into());
        }
        res
    }
}

/// Maps modifier combination to map of keys
///
/// Both mobile-default and mobile-shift modes are required.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[derive(Shrinkwrap, CollectDocs)]
pub struct MobileModes(pub BTreeMap<String, MobileKeyMap>);

/// Maps modifier combination to map of keys
///
/// In general only the `default` and `shift` modes are strictly required.
/// Some targets require other modes, and the tool will inform you if they are
/// missing.
#[example(
    yaml,
    r#"
default: |
  ' 1 2 3 4 5 6 7 8 9 0 + ´
    á š e r t y u i o p å ŋ
    a s d f g h j k l ö ä đ
  ž z č c v b n m , . -
shift: |
  § ! " # $ % & / ( ) = ? `
    Á Š E R T Y U I O P Å Ŋ
    A S D F G H J K L Ö Ä Đ
  Ž Z Č C V B N M ; : _
cmd+shift: |
  ° ! " # € % & / ( ) = ? `
    Q W E R T Y U I O P Å ^
    A S D F G H J K L Ö Ä *
  > Z X C V B N M ; : _
"#
)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[derive(Shrinkwrap, CollectDocs)]
pub struct DesktopModes(pub BTreeMap<String, DesktopKeyMap>);

pub enum Mode {
    Mobile(MobileModes),
    Desktop(DesktopModes),
}

/// A layout is defined as a file by the name `<locale>.yaml` and lives in the
/// `locales/` directory in the kbdgen project bundle.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default, CollectDocs)]
pub struct Layout {
    /// The display names for the layout, keyed by locale.
    ///
    /// A valid locale is any ISO 639-1 or 639-3 code. If a language has both,
    /// prefer the 639-1 variant for better support.
    ///
    /// It must be defined for at least the `en` locale, and preferably also for
    /// each well-supported locale that you expect to support.
    #[example(
        yaml,
        r#"
        displayNames:
          en: Julev Sami (Sweden)
          smj: Julevsámegiella (Svierik)
          smj-SE: Julevsámegiella (Svierik)
    "#
    )]
    #[serde(rename = "displayNames")]
    pub display_names: BTreeMap<String, String>,

    /// The different modes.
    ///
    /// NOTE: Do not forget the `\|` symbol after the `<mode>` key or you will
    /// receive unexpected parsing errors.
    #[example(
        yaml,
        r#"
        modes:
          mobile:
            default: |
              á w e r t y u i o p å
              a s d f g h j k l ö ä
                 z x c v b n m ŋ
            shift: |
              Á W E R T Y U I O P Å
              A S D F G H J K L Ö Ä
                 Z X C V B N M Ŋ
          mac:
            default: |
              § 1 2 3 4 5 6 7 8 9 0 + ´
                á w e r t y u i o p å ŋ
                a s d f g h j k l ö ä '
              < z x c v b n m , . -
            # ...
    "#
    )]
    pub modes: Modes,

    /// Specify the decimal separator for the given locale. Required for the
    /// numpad keys on some targets. Normally a '.' or ','.
    #[example(yaml, r#"decimal: ",""#)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decimal: Option<String>,

    /// An override for space keys on some OSes. Keyed by target.
    #[example(
        yaml,
        r#"
        space:
          mac:
            caps: '\u{A0}'
            alt: '\u{A0}'
    "#
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub space: Option<BTreeMap<String, BTreeMap<String, String>>>,

    /// Defines the dead keys for a target for a given `<mode>`.
    ///
    /// This is a nested map:
    /// The first key is the target;
    /// the second key is the mode (from the modes property);
    /// finally, the value is an array of dead keys.
    ///
    /// It is recommended that the keys of this array are wrapped in quotes to
    /// make diaeresis and other hard to see glyphs maintainable for future
    /// developers, including yourself.
    #[example(
        yaml,
        r#"
        deadKeys:
          mac:
            default: ["`"]
            shift: ['`']
    "#
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "deadKeys")]
    pub dead_keys: Option<BTreeMap<String, BTreeMap<String, Vec<String>>>>,

    /// The items to be shown when a key is long-pressed. Values are space
    /// separated in one string.
    #[example(
        yaml,
        r#"
        deadKeys:
          mac:
            default: ["`"]
            shift: ['`']
    "#
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub longpress: Option<BTreeMap<String, String>>,

    /// The chain of inputs necessary to provide an output after a deadkey is
    /// pressed. Keyed by each individual input.
    ///
    /// Always includes deadkeys but some targets support key sequencing
    /// (replacing glyphs based on input pattern) — this behaviour is target
    /// dependent.
    ///
    /// This map may be repeatedly nested until a terminal is reached. If a
    /// sequence is short-circuited, the `" "` is used as the fallback output in
    /// all cases.
    #[example(
        yaml,
        r#"
        longpress:
          A: Æ Ä Å Â
          Á: Q
          C: Č
          D: Đ
    "#
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transforms: Option<BTreeMap<String, BTreeMap<String, String>>>,

    /// Strings to be shown on some OSes
    ///
    /// Currently, they are used for specifying strings to be shown on the space
    /// and return keys on mobile targets.
    #[example(
        yaml,
        r#"
        strings:
          space: space
          return: return
    "#
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strings: Option<LayoutStrings>,

    /// Derives
    #[serde(skip_serializing_if = "Option::is_none")]
    pub derive: Option<DeriveOptions>,

    /// A map of target-specific customisation properties.
    ///
    /// Key is the code for the target. Only necessary if you need to set a
    /// target-specific property.
    #[example(
        yaml,
        r#"
        targets:
          win:
            locale: sma-Latn-NO
    "#
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targets: Option<LayoutTarget>,
}

impl Layout {
    pub fn name(&self) -> Option<String> {
        self.display_names
            .get("en")
            .or_else(|| self.display_names.values().next())
            .cloned()
    }
}

/// Targets for settings per layout
#[derive(Debug, Clone, PartialEq)]
#[derive(Serialize, Deserialize, Default, CollectDocs)]
pub struct LayoutTarget {
    #[serde(skip_serializing_if = "Option::is_none")]
    win: Option<LayoutTargetWindows>,

    #[serde(skip_serializing_if = "Option::is_none")]
    mac: Option<YamlValue>,

    #[serde(skip_serializing_if = "Option::is_none")]
    ios: Option<LayoutTargetIOS>,

    #[example(
        yaml,
        r#"
        android:
          legacyName: kpv"#
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    android: Option<LayoutTargetAndroid>,

    #[example(
        yaml,
        r#"
        chrome:
          locale: sv
          xkbLayout: se
    "#
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    chrome: Option<YamlValue>,

    #[serde(skip_serializing_if = "Option::is_none")]
    x11: Option<YamlValue>,

    #[serde(skip_serializing_if = "Option::is_none")]
    desktop: Option<YamlValue>,

    #[serde(skip_serializing_if = "Option::is_none")]
    mobile: Option<YamlValue>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, CollectDocs)]
pub struct LayoutTargetWindows {
    /// The actual locale within Windows, as per their broken ISO 639-3 scheme
    /// or secret hardcoded lists.
    pub locale: String,

    /// The language name to be cached, in order to try to mask the ugly ISO
    /// code name that often shows.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "languageName")]
    pub language_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, CollectDocs)]
pub struct LayoutTargetIOS {
    /// Minimum SDK can be specified for a specific layout
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "legacyName")]
    pub legacy_name: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, CollectDocs)]
pub struct LayoutTargetAndroid {
    /// The API level that is the minimum supported for a keyboard. Useful for
    /// limiting access to a keyboard where it is known several glyphs are
    /// missing on older devices.
    ///
    /// https://source.android.com/source/build-numbers.html[See the Android documentation for API versions compared to OS version].
    ///
    /// NOTE: The lowest API supported by this keyboard is API 16, but it may
    /// work on older variants.
    #[example(yaml, "minimumSdk: 16")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "minimumSdk")]
    pub minimum_sdk: Option<u32>,

    /// Styles
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<BTreeMap<String, YamlValue>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "legacyName")]
    pub legacy_name: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, CollectDocs)]
pub struct TargetAndroid {
    pub version: String,

    pub build: u32,

    /// The reverse-domain notation ID for the package
    #[example(yaml, "packageId: com.example.mypackageid")]
    #[serde(rename = "packageId")]
    pub package_id: String,

    /// Path to the icon file to be converted into the various sizes required by
    /// Android, relative to project root.
    #[example(yaml, "icon: icons/icon.png")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "sentryDsn")]
    pub sentry_dsn: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "showNumberHints")]
    pub show_number_hints: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "minimumSdk")]
    pub minimum_sdk: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub chfst: Option<bool>,

    /// Path to the Android keystore (see <<Generating keystores>> section for
    /// more information)
    #[example(yaml, "keyStore: my.keystore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "keyStore")]
    pub key_store: Option<String>,

    /// The key to use within the provided keystore
    #[example(yaml, "keyAlias: myprojectkey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "keyAlias")]
    pub key_alias: Option<String>,
}

// TODO: Keyboards have a provisioningProfileId -- add this here?
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, CollectDocs)]
pub struct TargetIOS {
    #[example(yaml, r#"version: 0.1.0"#)]
    pub version: String,

    #[example(yaml, r#"build: 1"#)]
    pub build: u32,

    #[example(yaml, r#"packageId: com.example.mypackageid"#)]
    #[serde(rename = "packageId")]
    pub package_id: String,

    #[example(yaml, r#"icon: icons/icon.png"#)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,

    #[example(yaml, r#"bundleName: Fancy Example Keyboards"#)]
    #[serde(rename = "bundleName")]
    pub bundle_name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "teamId")]
    pub team_id: Option<String>,

    #[example(
        yaml,
        r#"codeSignId: "iPhone Distribution: The University of Tromso (000ABC000)""#
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "codeSignId")]
    pub code_sign_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "sentryDsn")]
    pub sentry_dsn: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "aboutDir")]
    pub about_dir: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub chfst: Option<bool>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, CollectDocs)]
pub struct TargetWindows {
    #[example(yaml, r#"version: 0.2.0"#)]
    pub version: String,

    #[example(yaml, r#"appName: Fancy Example Keyboards"#)]
    #[serde(rename = "appName")]
    pub app_name: String,

    #[example(yaml, r#"url: 'http://divvun.no'"#)]
    pub url: String,

    #[example(yaml, r#"uuid: 0D18406F-1209-43EF-B18F-58961BC8E2E3"#)]
    pub uuid: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "codeSignPfx")]
    pub code_sign_pfx: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "customLocales")]
    pub custom_locales: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "licensePath")]
    pub license_path: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "readmePath")]
    pub readme_path: Option<String>,
}

// TODO: Keyboards have a provisioningProfileId -- add this here?
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, CollectDocs)]
pub struct TargetMacOS {
    #[example(yaml, r#"version: 0.1.6"#)]
    pub version: String,

    #[example(yaml, r#"build: 12"#)]
    pub build: u32,

    #[example(yaml, r#"packageId: com.example.mypackageid"#)]
    #[serde(rename = "packageId")]
    pub package_id: String,

    #[example(yaml, r#"icon: icons/icon.png"#)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,

    #[example(yaml, r#"bundleName: Fancy Example Keyboards"#)]
    #[serde(rename = "bundleName")]
    pub bundle_name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "teamId")]
    pub team_id: Option<String>,

    #[example(
        yaml,
        r#"codeSignId: "iPhone Distribution: The University of Tromso (000ABC000)""#
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "codeSignId")]
    pub code_sign_id: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, CollectDocs)]
pub struct TargetChrome {
    pub version: String,
    pub build: u32,

    #[serde(rename = "appId")]
    pub app_id: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, CollectDocs)]
pub struct TargetX11 {
    pub version: String,
    pub build: u32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, CollectDocs)]
pub struct TargetMim {
    pub language_code: String,
    pub description: Option<String>,
}

/// Opaque YAML value
///
/// Sorry, that means there is no further documentation on its structure here.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, CollectDocs)]
pub struct YamlValue(yaml::Value);
