use std::collections::{HashMap, HashSet};
use std::env;
use std::error::Error;
use std::fs::read_dir;
use std::fs::read_to_string;
use std::path::{Path, PathBuf};
use syn::{Expr, ExprLit, Item, ItemConst, Lit, LitInt};

struct Version {
    enum_files: Vec<syn::File>,
    packet_files: Vec<syn::File>,
    type_files: Vec<syn::File>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let out_dir = env::var("OUT_DIR").unwrap();
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    
    let src_dir = Path::new(&manifest_dir).join("src");
    let version_dir = src_dir.join("version");
    
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
    
    
    let versions: HashMap<u16, Version> = HashMap::new();
    
    for dir in &version_dirs {
        if !dir.path().is_dir() { continue; }
        
        let info_file_path = dir.path().join("info.rs");
        if !info_file_path.exists() { continue; }
        
        let content = read_to_string(&info_file_path)?;
        if let Some(protocol_version) = parse_protocol_version(&content) {
            println!(
                "cargo:warning=Folder: {}, Protocol version: {}",
                dir.file_name().to_string_lossy(),
                protocol_version
            );
        }
    }
    
    println!("cargo:warning=OUT_DIR is {}", out_dir);
    println!("cargo:warning=SRC_DIR is {:?}", src_dir);
    
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