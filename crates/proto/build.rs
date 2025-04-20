use std::collections::{HashMap, HashSet};
use std::{env, fs};
use std::error::Error;
use std::ffi::OsStr;
use std::fs::{create_dir_all, read_to_string, File, OpenOptions};
use std::fs::read_dir;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use proc_macro2::TokenTree;
use syn::{Expr, ExprLit, Item, ItemConst, ItemEnum, ItemStruct, Lit, Type};
use syn::__private::ToTokens;

#[derive(Clone, Debug)]
struct TokenInfo {
    pub file_path: PathBuf,
    pub usages: HashSet<PathBuf>
}

struct Version {
    pub enums: HashMap<String, TokenInfo>,
    pub packets: HashMap<String, TokenInfo>,
    pub types: HashMap<String, TokenInfo>,
}

impl Version {
    pub fn find_usages(&mut self) {
        let enums = self.enums.clone();
        for (name, token_info) in enums {
            self.find_usages_in_file(name.as_str(), &token_info.file_path)
        }

        let types = self.types.clone();
        for (name, token_info) in types {
            self.find_usages_in_file(name.as_str(), &token_info.file_path)
        }

        let packets = self.packets.clone();
        for (name, token_info) in packets {
            self.find_usages_in_file(name.as_str(), &token_info.file_path)
        }
    }
    
    fn find_usages_in_file(&mut self, ignore: &str, file: &PathBuf) {
        let content = read_to_string(file).unwrap();
        let syn_tree = syn::parse_file(&content).unwrap();

        let tokens = syn_tree.to_token_stream();

        for token in tokens {
            if let TokenTree::Ident(ident) = token {
                let name = ident.to_string();
                if name == ignore { continue; }

                if let Some(token_info) = self.enums.get_mut(&name) {
                    token_info.usages.insert(file.clone());
                }

                if let Some(token_info) = self.types.get_mut(&name) {
                    token_info.usages.insert(file.clone());
                }

                if let Some(token_info) = self.packets.get_mut(&name) {
                    token_info.usages.insert(file.clone());
                }
            }
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    
    let src_dir = Path::new(&manifest_dir).join("src");
    let version_dir = src_dir.join("version");
    let gen_dir = src_dir.join("gen");
    
    let mut version_dirs: Vec<_> = read_dir(version_dir)?
        .filter_map(|entry| {
            let entry = entry.ok()?;
            if entry.path().is_dir() {
                Some(entry)
            }
            else { None }
        })
        .collect();
    
    version_dirs.sort_by_key(|dir| {
        dir.file_name().to_string_lossy().trim_start_matches("v").parse::<u16>().unwrap_or(0)
    });
    
    
    let mut versions: HashMap<u16, Version> = HashMap::new();
    
    for dir in &version_dirs {
        if !dir.path().is_dir() { continue; }
        
        let info_file_path = dir.path().join("info.rs");
        if !info_file_path.exists() { continue; }
        
        let content = read_to_string(&info_file_path)?;
        if let Some(protocol_version) = parse_protocol_version(&content) {
            let enums: HashMap<String, TokenInfo> = find_version_enums(&dir.path())
                .unwrap_or(Vec::new())
                .iter()
                .filter_map(|path| {
                    let name = get_first_enum_name_in_file(path)?;
                    
                    let token_info = TokenInfo {
                        file_path: path.clone(),
                        usages: HashSet::new(),
                    };
                    
                    Some((name, token_info))
                })
                .collect();

            let packets: HashMap<String, TokenInfo> = find_version_packets(&dir.path())
                .unwrap_or(Vec::new())
                .iter()
                .filter_map(|path| {
                    let name = get_first_struct_name_in_file(path)?;

                    let token_info = TokenInfo {
                        file_path: path.clone(),
                        usages: HashSet::new(),
                    };

                    Some((name, token_info))
                })
                .collect();

            let types: HashMap<String, TokenInfo> = find_version_types(&dir.path())
                .unwrap_or(Vec::new())
                .iter()
                .filter_map(|path| {
                    let name = get_first_struct_name_in_file(path)?;

                    let token_info = TokenInfo {
                        file_path: path.clone(),
                        usages: HashSet::new(),
                    };

                    Some((name, token_info))
                })
                .collect();
            
            let mut version = Version {
                enums,
                packets,
                types,
            };
            
            version.find_usages();

            let log_str = format!(
                "VERSION: {:#?},\n\nENUM USAGES: {:#?},\n\nPACKET USAGES: {:#?},\n\nTYPE USAGES: {:#?}",
                protocol_version,
                version.enums.iter()
                    .filter_map(|(name, e)| 
                        if (e.usages.len() > 0) {
                            Some(
                                (
                                    name,
                                    e.usages
                                        .iter()
                                        .map(|p|
                                            p.file_name().unwrap()
                                        )
                                        .collect::<Vec<_>>()
                                )
                            )
                        }
                        else { None }
                    )
                    .collect::<Vec<_>>(),
                version.packets.iter()
                    .filter_map(|(name, e)| 
                        if (e.usages.len() > 0) {
                            Some(
                                (
                                    name, 
                                    e.usages
                                        .iter()
                                        .map(|p|
                                            p.file_name().unwrap()
                                        )
                                        .collect::<Vec<_>>()
                                )
                            )
                        }
                        else { None }
                    )
                    .collect::<Vec<_>>(),
                version.types.iter()
                    .filter_map(|(name, e)|
                        if (e.usages.len() > 0) {
                            Some(
                                (
                                    name,
                                    e.usages
                                        .iter()
                                        .map(|p|
                                            p.file_name().unwrap()
                                        )
                                        .collect::<Vec<_>>()
                                )
                            )
                        }
                        else { None }
                    )
                    .collect::<Vec<_>>(),
            );
            
            let log_dir = gen_dir.join("log");
            create_dir_all(&log_dir)?;
            
            let log_file = log_dir.join(format!("log_{}.txt", protocol_version));
            
            let mut file = OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(&log_file)?;
            
            file.write_all(log_str.as_bytes())?;
            
            versions.insert(
                protocol_version, 
                version
            );
        }
    }
    
    Ok(())
}

fn parse_protocol_version(content: &str) -> Option<u16> {
    let syn_tree = syn::parse_file(&content).ok()?;
    
    for item in syn_tree.items {
        if let Item::Const(ItemConst { ident, expr, .. }) = item {
            if ident == "PROTOCOL_VERSION" {
                if let Expr::Lit(ExprLit { lit: Lit::Int(lit_int), .. }) = *expr {
                    return lit_int.to_string().parse::<u16>().ok();
                }
            }
        }
    }
    
    None
}

fn find_version_enums(path: &PathBuf) -> Option<Vec<PathBuf>> {
    let enums_folder = path.join("enums");
    if !enums_folder.exists() { return None; }
    
    let rs_files: Vec<_> = read_dir(enums_folder).ok()?
        .filter_map(|entry| {
            let entry = entry.ok()?;
            if entry.path().is_file() 
                && entry.path().extension() == Some(OsStr::new("rs")) 
                && entry.path().file_name() != Some(OsStr::new("mod.rs"))
            {
                Some(entry.path())
            } else { None }
        })
        .collect();
    
    Some(rs_files)
}

fn find_version_packets(path: &PathBuf) -> Option<Vec<PathBuf>> {
    let packets_folder = path.join("packets");
    if !packets_folder.exists() { return None; }

    let rs_files: Vec<_> = read_dir(packets_folder).ok()?
        .filter_map(|entry| {
            let entry = entry.ok()?;
            if entry.path().is_file()
                && entry.path().extension() == Some(OsStr::new("rs"))
                && entry.path().file_name() != Some(OsStr::new("mod.rs"))
            {
                Some(entry.path())
            } else { None }
        })
        .collect();

    Some(rs_files)
}

fn find_version_types(path: &PathBuf) -> Option<Vec<PathBuf>> {
    let types_folder = path.join("types");
    if !types_folder.exists() { return None; }

    let rs_files: Vec<_> = read_dir(types_folder).ok()?
        .filter_map(|entry| {
            let entry = entry.ok()?;
            if entry.path().is_file()
                && entry.path().extension() == Some(OsStr::new("rs"))
                && entry.path().file_name() != Some(OsStr::new("mod.rs"))
            {
                Some(entry.path())
            } else { None }
        })
        .collect();

    Some(rs_files)
}

fn get_first_enum_name_in_file(file: &PathBuf) -> Option<String> {
    let content = read_to_string(file).ok()?;
    let syn_tree = syn::parse_file(&content).ok()?;
    
    for item in syn_tree.items {
        if let Item::Enum(item_enum) = item {
            return Some(item_enum.ident.to_string());
        }
    }
    
    None
}

fn get_first_struct_name_in_file(file: &PathBuf) -> Option<String> {
    let content = read_to_string(file).ok()?;
    let syn_tree = syn::parse_file(&content).ok()?;

    for item in syn_tree.items {
        if let Item::Struct(item_struct) = item {
            return Some(item_struct.ident.to_string());
        }
    }

    None
}