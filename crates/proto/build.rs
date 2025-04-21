use std::collections::{HashMap, HashSet};
use std::{env, fs};
use std::error::Error;
use std::ffi::OsStr;
use std::fs::read_dir;
use std::fs::{create_dir_all, read_to_string, OpenOptions};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use quote::{format_ident, quote};
use syn::__private::ToTokens;
use syn::visit::{visit_type, Visit};
use syn::{Attribute, Expr, ExprLit, Ident, Item, ItemConst, Lit, Type};

#[derive(Clone, Debug)]
struct TokenInfo {
    pub name: String,
    pub file_path: PathBuf,
    pub usages: HashMap<String, PathBuf>,
}

#[derive(Debug)]
struct Version {
    pub version: i32,
    pub enums: HashMap<String, TokenInfo>,
    pub packets: HashMap<String, TokenInfo>,
    pub types: HashMap<String, TokenInfo>,
}

impl Version {
    pub fn get_all_tokens(&self) -> HashMap<String, TokenInfo> {
        vec![
            self.enums.clone(), 
            self.packets.clone(), 
            self.types.clone()
        ]
            .into_iter()
            .flatten()
            .collect::<HashMap<_, _>>()
    }
    
    pub fn find_usages(&mut self) {
        let enums = self.enums.clone();
        for (name, token_info) in enums {
            self.find_usages_from_token(name.as_str(), &token_info)
        }

        let packets = self.packets.clone();
        for (name, token_info) in packets {
            self.find_usages_from_token(name.as_str(), &token_info)
        }

        let types = self.types.clone();
        for (name, token_info) in types {
            self.find_usages_from_token(name.as_str(), &token_info)
        }
    }

    fn find_usages_from_token(&mut self, ignore: &str, other: &TokenInfo) {
        let content = read_to_string(&other.file_path).unwrap();
        let syn_tree = syn::parse_file(&content).unwrap();

        let mut finder = UsageFinder {
            ignore,
            other,
            enums: &mut self.enums,
            packets: &mut self.packets,
            types: &mut self.types,
        };

        finder.visit_file(&syn_tree);
    }
}

struct UsageFinder<'a> {
    pub ignore: &'a str,
    pub other: &'a TokenInfo,
    pub enums: &'a mut HashMap<String, TokenInfo>,
    pub packets: &'a mut HashMap<String, TokenInfo>,
    pub types: &'a mut HashMap<String, TokenInfo>,
}

impl<'ast, 'a> Visit<'ast> for UsageFinder<'a> {
    fn visit_type(&mut self, i: &'ast Type) {
        if let Type::Path(path) = i {
            if let Some(segment) = path.path.segments.last() {
                let name = segment.ident.to_string();

                if name != self.ignore {
                    if let Some(token_info) = self.enums.get_mut(&name) {
                        token_info
                            .usages
                            .insert(self.other.name.clone(), self.other.file_path.clone());
                    }

                    if let Some(token_info) = self.packets.get_mut(&name) {
                        token_info
                            .usages
                            .insert(self.other.name.clone(), self.other.file_path.clone());
                    }

                    if let Some(token_info) = self.types.get_mut(&name) {
                        token_info
                            .usages
                            .insert(self.other.name.clone(), self.other.file_path.clone());
                    }
                }
            }
        }

        visit_type(self, i);
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    let src_dir = Path::new(&manifest_dir).join("src");
    let version_dir = src_dir.join("version");
    let gen_dir = src_dir.join("gen");

    let log_dir = gen_dir.join("log");
    create_dir_all(&log_dir)?;

    let log_verbose_dir = log_dir.join("verbose");
    create_dir_all(&log_verbose_dir)?;

    let mut version_dirs: Vec<_> = read_dir(&version_dir)?
        .filter_map(|entry| {
            let entry = entry.ok()?;
            if entry.path().is_dir() {
                Some(entry)
            } else {
                None
            }
        })
        .collect();

    version_dirs.sort_by_key(|dir| {
        dir.file_name()
            .to_string_lossy()
            .trim_start_matches("v")
            .parse::<i32>()
            .unwrap_or(0)
    });

    let mut versions: Vec<Version> = Vec::new();

    for dir in &version_dirs {
        if !dir.path().is_dir() {
            continue;
        }

        let info_file_path = dir.path().join("info.rs");
        if !info_file_path.exists() {
            continue;
        }

        let content = read_to_string(&info_file_path)?;
        if let Some(protocol_version) = parse_protocol_version(&content) {
            let enums: HashMap<String, TokenInfo> = find_version_enums(&dir.path())
                .unwrap_or(Vec::new())
                .iter()
                .filter_map(|path| {
                    let token_names = find_proto_gen_types_in_file(path);

                    if let Some(token_names) = token_names {
                        let tokens = token_names
                            .iter()
                            .map(|n| {
                                (
                                    n.clone(),
                                    TokenInfo {
                                        name: n.clone(),
                                        file_path: path.clone(),
                                        usages: HashMap::new(),
                                    },
                                )
                            })
                            .collect::<HashMap<String, TokenInfo>>();

                        Some(tokens)
                    } else {
                        println!(
                            "cargo:warning=Didn't find any proto_gen types in file: {}!",
                            path.file_name()?.to_str()?
                        );
                        None
                    }
                })
                .flatten()
                .collect();

            let packets: HashMap<String, TokenInfo> = find_version_packets(&dir.path())
                .unwrap_or(Vec::new())
                .iter()
                .filter_map(|path| {
                    let token_names = find_proto_gen_types_in_file(path);

                    if let Some(token_names) = token_names {
                        let tokens = token_names
                            .iter()
                            .map(|n| {
                                (
                                    n.clone(),
                                    TokenInfo {
                                        name: n.clone(),
                                        file_path: path.clone(),
                                        usages: HashMap::new(),
                                    },
                                )
                            })
                            .collect::<HashMap<String, TokenInfo>>();

                        Some(tokens)
                    } else {
                        println!(
                            "cargo:warning=Didn't find any proto_gen types in file: {}!",
                            path.file_name()?.to_str()?
                        );
                        None
                    }
                })
                .flatten()
                .collect();

            let types: HashMap<String, TokenInfo> = find_version_types(&dir.path())
                .unwrap_or(Vec::new())
                .iter()
                .filter_map(|path| {
                    let token_names = find_proto_gen_types_in_file(path);

                    if let Some(token_names) = token_names {
                        let tokens = token_names
                            .iter()
                            .map(|n| {
                                (
                                    n.clone(),
                                    TokenInfo {
                                        name: n.clone(),
                                        file_path: path.clone(),
                                        usages: HashMap::new(),
                                    },
                                )
                            })
                            .collect::<HashMap<String, TokenInfo>>();

                        Some(tokens)
                    } else {
                        println!(
                            "cargo:warning=Didn't find any proto_gen types in file: {}!",
                            path.file_name()?.to_str()?
                        );
                        None
                    }
                })
                .flatten()
                .collect();

            let mut version = Version {
                version: protocol_version,
                enums,
                packets,
                types,
            };

            version.find_usages();

            // region Logging
            let mut enum_usages = version
                .enums
                .iter()
                .filter_map(|(name, e)| {
                    if e.usages.len() > 0 {
                        Some((
                            name,
                            e.usages
                                .iter()
                                .map(|(n, p)| (n, p.file_name().unwrap()))
                                .collect::<HashMap<_, _>>(),
                        ))
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();
            enum_usages.sort_by(|(a, _), (b, _)| a.cmp(b));

            let mut packet_usages = version
                .packets
                .iter()
                .filter_map(|(name, e)| {
                    if e.usages.len() > 0 {
                        Some((
                            name,
                            e.usages
                                .iter()
                                .map(|(n, p)| (n, p.file_name().unwrap()))
                                .collect::<HashMap<_, _>>(),
                        ))
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();
            packet_usages.sort_by(|(a, _), (b, _)| a.cmp(b));

            let mut type_usages = version
                .types
                .iter()
                .filter_map(|(name, e)| {
                    if e.usages.len() > 0 {
                        Some((
                            name,
                            e.usages
                                .iter()
                                .map(|(n, p)| (n, p.file_name().unwrap()))
                                .collect::<HashMap<_, _>>(),
                        ))
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();
            type_usages.sort_by(|(a, _), (b, _)| a.cmp(b));

            let log_str = format!(
                "VERSION: {:#?},\n\nENUM USAGES: {:#?},\n\nPACKET USAGES: {:#?},\n\nTYPE USAGES: {:#?}",
                protocol_version,
                enum_usages,
                packet_usages,
                type_usages,
            );

            let log_file = log_dir.join(format!("log_{}.txt", protocol_version));

            let mut file = OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(&log_file)?;

            file.write_all(log_str.as_bytes())?;

            let log_verbose_file =
                log_verbose_dir.join(format!("log_verbose_{}.txt", protocol_version));

            let file_full = OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(&log_verbose_file)?;

            write!(&file_full, "{:#?}", version)?;
            // endregion

            versions.push(version);
        }
    }

    // region Logging
    let log_verbose_path = log_verbose_dir.join("log_verbose.txt");
    let log_verbose_file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&log_verbose_path)?;

    write!(&log_verbose_file, "{:#?}", versions)?;
    // endregion
    
    let token_usages = versions
        .iter()
        .map(|version| {
            let enums = version.enums
                .iter()
                .map(|item| {
                    (
                        item.0.clone(),
                        item.1.usages
                            .iter()
                            .map(|(n, _)| n.clone())
                            .collect::<HashSet<_>>(),
                    )
                })
                .collect::<Vec<_>>();
            
            let packets = version.packets
                .iter()
                .map(|item| {
                    (
                        item.0.clone(),
                        item.1.usages
                            .iter()
                            .map(|(n, _)| n.clone())
                            .collect::<HashSet<_>>(),
                    )
                })
                .collect::<Vec<_>>();

            let types = version.types
                .iter()
                .map(|item| {
                    (
                        item.0.clone(),
                        item.1.usages
                            .iter()
                            .map(|(n, _)| n.clone())
                            .collect::<HashSet<_>>(),
                    )
                })
                .collect::<Vec<_>>();
            
            vec![enums, packets, types]
                .iter()
                .flatten()
                .map(|v| { (v.0.clone(), v.1.clone()) })
                .collect::<HashMap<_, HashSet<_>>>()
        })
        .flatten()
        .fold(HashMap::<_, HashSet<_>>::new(), |mut acc, (key, set)| {
            acc.entry(key)
                .and_modify(|e| e.extend(set.clone()))
                .or_insert(set);
            acc
        });

    // region Logging
    let log_verbose_path = log_verbose_dir.join("log_verbose_usages.txt");
    let log_verbose_file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&log_verbose_path)?;

    write!(&log_verbose_file, "{:#?}", token_usages)?;
    // endregion

    let gen_version_folder_path = gen_dir.join("version");
    create_dir_all(&gen_version_folder_path)?;
    
    let mut mod_file_tokens = quote! {};
    
    for i in 0..versions.len() {
        let current = &versions[i];
        
        println!("cargo:warning=PROCESSING {:?}", current.version);
        
        let mut full_flat_usage_set: HashMap<String, HashSet<PathBuf>> = HashMap::new();

        let mut full_flat_enum_usage_set: HashMap<String, HashSet<PathBuf>> = HashMap::new(); //  if the usage is an enum
        let mut full_flat_packet_usage_set: HashMap<String, HashSet<PathBuf>> = HashMap::new(); //  if the usage is a packet
        let mut full_flat_type_usage_set: HashMap<String, HashSet<PathBuf>> = HashMap::new(); //  if the usage is a type
        
        for j in 0..i {
            if i == j { continue; }
            
            let prev = &versions[j];
            
            println!("cargo:warning=COMPARING {:?} -> {:?}", current.version, prev.version);

            let mut flat_enum_usage_set: HashMap<String, HashSet<PathBuf>> = HashMap::new(); //  if the usage is an enum
            let mut flat_packet_usage_set: HashMap<String, HashSet<PathBuf>> = HashMap::new(); //  if the usage is a packet
            let mut flat_type_usage_set: HashMap<String, HashSet<PathBuf>> = HashMap::new(); //  if the usage is a type
            
            let mut token_name_queue: Vec<String> = current.get_all_tokens().iter().map(|(n, _)| n.clone()).collect();
            while !token_name_queue.is_empty() {
                let token_name = token_name_queue.pop().unwrap();
                
                if let Some(usage_names) = token_usages.get(&token_name) {
                    for usage_name in usage_names {
                        if let Some(usage_token) = prev.enums.get(usage_name) {
                            flat_enum_usage_set.entry(token_name.clone())
                                .and_modify(|e| {
                                    e.insert(usage_token.file_path.clone());
                                })
                                .or_insert({
                                    let mut set = HashSet::new();
                                    set.insert(usage_token.file_path.clone());
                                    set
                                });
                        }

                        if let Some(usage_token) = prev.packets.get(usage_name) {
                            flat_packet_usage_set.entry(token_name.clone())
                                .and_modify(|e| {
                                    e.insert(usage_token.file_path.clone());
                                })
                                .or_insert({
                                    let mut set = HashSet::new();
                                    set.insert(usage_token.file_path.clone());
                                    set
                                });
                        }

                        if let Some(usage_token) = prev.types.get(usage_name) {
                            flat_type_usage_set.entry(token_name.clone())
                                .and_modify(|e| {
                                    e.insert(usage_token.file_path.clone());
                                })
                                .or_insert({
                                    let mut set = HashSet::new();
                                    set.insert(usage_token.file_path.clone());
                                    set
                                });
                        }
                        
                        if let Some(sub_usages) = token_usages.get(usage_name) {
                            token_name_queue.extend(sub_usages.iter().cloned())
                        }
                    }
                }
            }

            // region Logging
            let log_verbose_usages_path = log_verbose_dir.join(format!("log_verbose_usages_{}-{}.txt", current.version, prev.version));
            let log_verbose_usages_file = OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(&log_verbose_usages_path)?;
            
            let mut usage_map = HashMap::new();
            usage_map.insert("enums", flat_enum_usage_set.clone());
            usage_map.insert("packet", flat_packet_usage_set.clone());
            usage_map.insert("types", flat_type_usage_set.clone());

            write!(&log_verbose_usages_file, "{:#?}", usage_map)?;
            // endregion
            
            full_flat_enum_usage_set.extend(flat_enum_usage_set);
            full_flat_packet_usage_set.extend(flat_packet_usage_set);
            full_flat_type_usage_set.extend(flat_type_usage_set);
        }

        // region Logging
        let log_verbose_usages_path = log_verbose_dir.join(format!("log_verbose_usages_{}.txt", current.version));
        let log_verbose_usages_file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&log_verbose_usages_path)?;

        let mut usage_map = HashMap::new();
        usage_map.insert("enums", full_flat_enum_usage_set.clone());
        usage_map.insert("packet", full_flat_packet_usage_set.clone());
        usage_map.insert("types", full_flat_type_usage_set.clone());

        write!(&log_verbose_usages_file, "{:#?}", usage_map)?;
        // endregion
        
        let gen_folder = gen_version_folder_path.join(format!("v{}", current.version));
        create_dir_all(&gen_folder)?;
        
        let enums_folder = gen_folder.join("enums");
        create_dir_all(&enums_folder)?;
        
        let packets_folder = gen_folder.join("packets");
        create_dir_all(&packets_folder)?;
        
        let types_folder = gen_folder.join("types");
        create_dir_all(&types_folder)?;
        
        let enum_usage_paths = full_flat_enum_usage_set.values().cloned().flatten().collect::<HashSet<_>>();
        let packet_usage_paths = full_flat_packet_usage_set.values().cloned().flatten().collect::<HashSet<_>>();
        let type_usage_paths = full_flat_type_usage_set.values().cloned().flatten().collect::<HashSet<_>>();

        let mut enum_paths = current.enums.values().map(|t| t.file_path.clone()).collect::<HashSet<_>>();
        let mut packet_paths = current.packets.values().map(|t| t.file_path.clone()).collect::<HashSet<_>>();
        let mut type_paths = current.types.values().map(|t| t.file_path.clone()).collect::<HashSet<_>>();
        
        enum_paths.extend(enum_usage_paths);
        packet_paths.extend(packet_usage_paths);
        type_paths.extend(type_usage_paths);
        
        for path in &enum_paths {
            fs::copy(path.clone(), enums_folder.join(path.file_name().unwrap()))?;
        }

        for path in &packet_paths {
            fs::copy(path.clone(), packets_folder.join(path.file_name().unwrap()))?;
        }

        for path in &type_paths {
            fs::copy(path.clone(), types_folder.join(path.file_name().unwrap()))?;
        }
        
        let base_mod = quote! {
            macro_rules! export {
                ($name:ident) => {
                    mod $name;
                    pub use $name::*;
                };
            }
        };

        {
            let mut enum_mod = base_mod.clone();
            for path in &enum_paths {
                let file_name = format_ident!("{}", path.file_stem().unwrap().to_str().unwrap());
                enum_mod.extend(quote! {
                    export!(#file_name);
                })
            }

            let packet_file = syn::parse2::<syn::File>(enum_mod)?;

            let enum_mod_file_path = enums_folder.join("mod.rs");
            let mut enum_mod_file = OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(&enum_mod_file_path)?;
            enum_mod_file.write_all(prettyplease::unparse(&packet_file).as_bytes())?
        }

        {
            let mut packet_mod = base_mod.clone();
            for path in &packet_paths {
                let file_name = format_ident!("{}", path.file_stem().unwrap().to_str().unwrap());
                packet_mod.extend(quote! {
                    export!(#file_name);
                })
            }

            let packet_file = syn::parse2::<syn::File>(packet_mod)?;

            let packet_mod_file_path = packets_folder.join("mod.rs");
            let mut packet_mod_file = OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(&packet_mod_file_path)?;
            packet_mod_file.write_all(prettyplease::unparse(&packet_file).as_bytes())?
        }

        {
            let mut type_mod = base_mod.clone();
            for path in &type_paths {
                let file_name = format_ident!("{}", path.file_stem().unwrap().to_str().unwrap());
                type_mod.extend(quote! {
                    export!(#file_name);
                })
            }

            let type_file = syn::parse2::<syn::File>(type_mod)?;

            let type_mod_file_path = types_folder.join("mod.rs");
            let mut type_mod_file = OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(&type_mod_file_path)?;
            type_mod_file.write_all(prettyplease::unparse(&type_file).as_bytes())?
        }

        {
            let version_mod = quote! {
                pub mod enums;
                pub mod packets;
                pub mod types;
            };

            let version_file = syn::parse2::<syn::File>(version_mod)?;

            let version_mod_path = gen_folder.join("mod.rs");
            let mut version_mod_file = OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(&version_mod_path)?;
            version_mod_file.write_all(prettyplease::unparse(&version_file).as_bytes())?
        }
        
        let version_mod_ident = format_ident!("v{}", current.version.to_string());
        mod_file_tokens.extend(quote! {
            pub mod #version_mod_ident;
        })
    }
    
    let mod_syn_file = syn::parse2::<syn::File>(mod_file_tokens)?;

    let mod_path = gen_version_folder_path.join("mod.rs");
    let mut mod_file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&mod_path)?;
    mod_file.write_all(prettyplease::unparse(&mod_syn_file).as_bytes())?;

    Ok(())
}

fn parse_protocol_version(content: &str) -> Option<i32> {
    let syn_tree = syn::parse_file(&content).ok()?;

    for item in syn_tree.items {
        if let Item::Const(ItemConst { ident, expr, .. }) = item {
            if ident == "PROTOCOL_VERSION" {
                if let Expr::Lit(ExprLit {
                    lit: Lit::Int(lit_int),
                    ..
                }) = *expr
                {
                    return lit_int.to_string().parse::<i32>().ok();
                }
            }
        }
    }

    None
}

fn find_version_enums(path: &PathBuf) -> Option<Vec<PathBuf>> {
    let enums_folder = path.join("enums");
    if !enums_folder.exists() {
        return None;
    }

    let rs_files: Vec<_> = read_dir(enums_folder)
        .ok()?
        .filter_map(|entry| {
            let entry = entry.ok()?;
            if entry.path().is_file()
                && entry.path().extension() == Some(OsStr::new("rs"))
                && entry.path().file_name() != Some(OsStr::new("mod.rs"))
            {
                Some(entry.path())
            } else {
                None
            }
        })
        .collect();

    Some(rs_files)
}

fn find_version_packets(path: &PathBuf) -> Option<Vec<PathBuf>> {
    let packets_folder = path.join("packets");
    if !packets_folder.exists() {
        return None;
    }

    let rs_files: Vec<_> = read_dir(packets_folder)
        .ok()?
        .filter_map(|entry| {
            let entry = entry.ok()?;
            if entry.path().is_file()
                && entry.path().extension() == Some(OsStr::new("rs"))
                && entry.path().file_name() != Some(OsStr::new("mod.rs"))
            {
                Some(entry.path())
            } else {
                None
            }
        })
        .collect();

    Some(rs_files)
}

fn find_version_types(path: &PathBuf) -> Option<Vec<PathBuf>> {
    let types_folder = path.join("types");
    if !types_folder.exists() {
        return None;
    }

    let rs_files: Vec<_> = read_dir(types_folder)
        .ok()?
        .filter_map(|entry| {
            let entry = entry.ok()?;
            if entry.path().is_file()
                && entry.path().extension() == Some(OsStr::new("rs"))
                && entry.path().file_name() != Some(OsStr::new("mod.rs"))
            {
                Some(entry.path())
            } else {
                None
            }
        })
        .collect();

    Some(rs_files)
}

fn find_proto_gen_types_in_file(file: &PathBuf) -> Option<Vec<String>> {
    let content = read_to_string(file).ok()?;
    let syn_tree = syn::parse_file(&content).ok()?;

    let mut proto_gen_types: Vec<String> = Vec::new();

    for item in syn_tree.items {
        if let Item::Struct(item_struct) = &item {
            if has_proto_gen_attr(&item_struct.attrs) {
                proto_gen_types.push(item_struct.ident.to_string());
            }
        }

        if let Item::Enum(item_enum) = &item {
            if has_proto_gen_attr(&item_enum.attrs) {
                proto_gen_types.push(item_enum.ident.to_string());
            }
        }
    }

    if proto_gen_types.is_empty() {
        None
    } else {
        Some(proto_gen_types)
    }
}

fn has_proto_gen_attr(attrs: &[Attribute]) -> bool {
    for attr in attrs {
        if attr.path().is_ident("allow") {
            if attr.to_token_stream().to_string().contains("proto_gen") {
                return true;
            }
        }
    }
    false
}
