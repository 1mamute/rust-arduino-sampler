use serde::{Deserialize, Serialize};
use serde_yaml::from_str;

#[derive(Debug, PartialEq, Deserialize)]
pub struct Controller {
    metadata: Metadata,
    spec: Spec,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
struct Metadata {
    name: String,
    version: String,
    vendor: String,
}

#[test]
fn test_metadata_struct() {
    let j: &str = r#"---
name: Minilab
version: MK II
vendor: Arturia
"#;

    let expected: Metadata = Metadata {
        name: "Minilab".to_string(),
        version: "MK II".to_string(),
        vendor: "Arturia".to_string(),
    };

    let teste: Metadata = serde_yaml::from_str(&j).unwrap();
    assert_eq!(expected, teste);
}

#[derive(Debug, PartialEq, Deserialize)]
struct Spec {
    keymap: Keymap,
}

#[test]
fn test_spec_struct() {
    let j: &str = r#"---
keymap:
    keys:
        - name: FirstWhite
          index: 1
          note: 144
          velocity: 1
"#;

    let expected: Spec = Spec {
        keymap: Keymap {
            keys: vec![Key {
                name: "FirstWhite".to_string(),
                index: 1,
                note: 144,
                velocity: 1,
            }],
        },
    };

    let teste: Spec = serde_yaml::from_str(&j).unwrap();
    assert_eq!(expected, teste);
}

#[derive(Debug, PartialEq, Deserialize)]
struct Keymap {
    keys: Vec<Key>,
}

#[test]
fn test_keymap_struct() {
    let j: &str = r#"---
keys:
    - name: FirstWhite
      index: 1
      note: 144
      velocity: 1
"#;

    let expected: Keymap = Keymap {
        keys: vec![Key {
            name: "FirstWhite".to_string(),
            index: 1,
            note: 144,
            velocity: 1,
        }],
    };

    let teste: Keymap = serde_yaml::from_str(&j).unwrap();
    assert_eq!(expected, teste);
}

#[derive(Debug, PartialEq, Deserialize)]
struct Key {
    name: String,
    index: u8,
    note: u8,
    velocity: u8, // https://serde.rs/string-or-struct.html
}

#[test]
fn test_key_struct() {
    let j: &str = r#"---
name: FirstWhite
index: 1
note: 144
velocity: 1
"#;

    let expected: Key = Key {
        name: "FirstWhite".to_string(),
        index: 1,
        note: 144,
        velocity: 1,
    };

    let teste: Key = serde_yaml::from_str(&j).unwrap();
    assert_eq!(expected, teste);
}

pub fn _parse_configuration() -> Result<Controller, serde_yaml::Error> {
    let boilerplate = Controller {
        metadata: Metadata {
            name: "Minilab".to_string(),
            version: "MK II".to_string(),
            vendor: "Arturia".to_string(),
        },
        spec: Spec {
            keymap: Keymap {
                keys: vec![Key {
                    name: "FirstWhite".to_string(),
                    index: 1,
                    note: 144,
                    velocity: 1,
                }],
            },
        },
    };

    Ok(boilerplate)
}
