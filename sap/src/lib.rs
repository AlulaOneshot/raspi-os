use std::{collections::HashMap, fs::{self, File}, path::Path};

use isolang::Language;
use tar::Archive;
use tempfile::tempdir;
use toml::Table;
use xz2::read::XzDecoder;

pub struct PackageManifest {
    package_id: String,
    name: HashMap<Language, String>,
    version: (i32, i32, i32),
    r#type: PackageType,
    category: PackageCategory,
    author: HashMap<Language, String>,
    publisher: HashMap<Language, String>,
    minimum_system_version: (i32, i32, i32),
    genres: Vec<String>,
    languages: Vec<Language>
}

impl PackageManifest {
    pub fn extract_from_package(path: &Path) -> Result<Self, String> {
        if path.exists() {
            let mut temp = tempdir().map_err(|e| e.to_string())?;
            let file = File::open(path).map_err(|e| e.to_string())?;
            let decompressor = XzDecoder::new(file);
            
            let mut archive = Archive::new(decompressor);
            archive.unpack(temp.path()).map_err(|e| e.to_string())?;

            let manifest_path = temp.path().join("manifest.toml");
            if !manifest_path.exists() {
                return Err("Package does not contain manifest".into());
            }

            let manifest_text = fs::read_to_string(manifest_path).map_err(|e| e.to_string())?;

            let manifest_base = &manifest_text.parse::<Table>().map_err(|e| e.to_string())?;

            match manifest_base.get("manifest") {
                Some(v) => {
                    match v {
                        toml::Value::Table(t) => {
                            match t.get("package_id") {
                                Some(v) => {
                                    match v {
                                        toml::Value::String(s) => {
                                            
                                            let split_identifier: Vec<&str> = s.splitn(3, '.').collect();

                                        },
                                        _ => {
                                            return Err("Key package_id is not a string".into());
                                        }
                                    }
                                }
                                None => {
                                    return Err("Key package_id is not present".into())
                                }
                            }
                        },
                        _ => {
                            return Err("Key [manifest] is not a table.".into());
                        }
                    }
                },
                None => {
                    return Err("Manifest does not contain [manifest] table".into());
                }
            }

            return Ok();
        }
        else {
            Err(format!("File {} does not exist", path.to_string_lossy()))
        }
    }
}

pub enum PackageCategory {
    Tool,
    Media,
    Social,
    Web,
    Education,
    System,
    Test
}

pub enum PackageType {
    Theme,
    Application,
    Game,
    Dlc,
    Mod,
    Demo,
    Tool
}