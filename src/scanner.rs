use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use regex::Regex;

#[derive(Debug, Deserialize, Serialize)]
pub struct Param {
    pub name: String,
    #[serde(rename = "type")]
    pub param_type: String,
    pub description: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Return {
    #[serde(rename = "type")]
    pub return_type: String,
    pub description: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Function {
    pub name: String,
    pub description: String,
    pub params: Vec<Param>,
    pub returns: Vec<Return>,
}

pub type Documentation = HashMap<String, Vec<Function>>;

pub fn scan_directory(path: &str) -> Result<Documentation, Box<dyn std::error::Error>> {
    let mut docs: Documentation = HashMap::new();
    scan_recursive(&Path::new(path), &mut docs)?;
    Ok(docs)
}

fn scan_recursive(dir: &Path, docs: &mut Documentation) -> Result<(), Box<dyn std::error::Error>> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                scan_recursive(&path, docs)?;
            } else if path.extension().and_then(|s| s.to_str()) == Some("lua") {
                parse_lua_file(&path, docs)?;
            }
        }
    }
    Ok(())
}

fn parse_lua_file(path: &PathBuf, docs: &mut Documentation) -> Result<(), Box<dyn std::error::Error>> {
    println!("[ INFO ] Scanning file: {:?}", path);
    let content = fs::read_to_string(path)?;
    let lines: Vec<&str> = content.lines().collect();
    
    let mut i = 0;
    while i < lines.len() {
        if lines[i].trim_start().starts_with("--@") || lines[i].trim_start().starts_with("-- @") {
            if let Some(func) = parse_function_doc(&lines, &mut i) {
                let category = func.0;
                let function = func.1;
                println!("[ INFO ] Found function: {} in category {}", function.name, category);
                docs.entry(category).or_insert_with(Vec::new).push(function);
            }
        }
        i += 1;
    }
    
    Ok(())
}

fn parse_function_doc(lines: &[&str], index: &mut usize) -> Option<(String, Function)> {
    let mut class_name = String::new();
    let mut description = String::new();
    let mut params = Vec::new();
    let mut returns = Vec::new();
    
    let start_index = *index;
    
    // Parse comment block
    while *index < lines.len() && (lines[*index].trim_start().starts_with("--@") || lines[*index].trim_start().starts_with("-- @")) {
        let line = lines[*index].trim_start()
            .trim_start_matches("--@")
            .trim_start_matches("-- @")
            .trim();
        
        if line.starts_with("class ") {
            class_name = line[6..].trim().to_string();
        } else if line.starts_with("desc ") {
            description = line[5..].trim().to_string();
        } else if line.starts_with("param ") {
            if let Some(param) = parse_param(line) {
                params.push(param);
            }
        } else if line.starts_with("return ") {
            let return_info = line[7..].trim();
            let parts: Vec<&str> = return_info.splitn(2, ',').map(|s| s.trim()).collect();
            if parts.len() == 2 {
                returns.push(Return {
                    return_type: parts[0].to_string(),
                    description: parts[1].to_string(),
                });
            } else {
                returns.push(Return {
                    return_type: "any".to_string(),
                    description: return_info.to_string(),
                });
            }
        }
        *index += 1;
    }
    
    // Skip empty lines
    while *index < lines.len() && lines[*index].trim().is_empty() {
        *index += 1;
    }
    
    // Parse function declaration
    if *index < lines.len() {
        let func_line = lines[*index];
        if let Some(func_name) = extract_function_name(func_line) {
            let (category, name) = if func_name.contains('.') {
                let parts: Vec<&str> = func_name.splitn(2, '.').collect();
                (parts[0].to_string(), parts[1].to_string())
            } else {
                let cat = if !class_name.is_empty() {
                    class_name
                } else {
                    "Global".to_string()
                };
                (cat, func_name)
            };
            
            return Some((category, Function {
                name: name,
                description,
                params,
                returns,
            }));
        }
    }
    
    None
}

fn parse_param(line: &str) -> Option<Param> {
    let content = line[6..].trim();
    let parts: Vec<&str> = content.splitn(3, ',').map(|s| s.trim()).collect();
    
    if parts.len() >= 3 {
        Some(Param {
            name: parts[0].to_string(),
            param_type: parts[1].to_string(),
            description: parts[2].to_string(),
        })
    } else if parts.len() == 2 {
        Some(Param {
            name: parts[0].to_string(),
            param_type: parts[1].to_string(),
            description: String::new(),
        })
    } else {
        None
    }
}

fn extract_function_name(line: &str) -> Option<String> {
    let function_regex = Regex::new(r"function\s+([a-zA-Z_][a-zA-Z0-9_]*(?:\.[a-zA-Z_][a-zA-Z0-9_]*)*)\s*\(").unwrap();
    let local_function_regex = Regex::new(r"local\s+function\s+([a-zA-Z_][a-zA-Z0-9_]*)\s*\(").unwrap();
    let method_regex = Regex::new(r"([a-zA-Z_][a-zA-Z0-9_]*(?:\.[a-zA-Z_][a-zA-Z0-9_]*)*):([a-zA-Z_][a-zA-Z0-9_]*)\s*\(").unwrap();
    let assignment_regex = Regex::new(r"([a-zA-Z_][a-zA-Z0-9_]*(?:\.[a-zA-Z_][a-zA-Z0-9_]*)*)\s*=\s*function\s*\(").unwrap();
    
    if let Some(captures) = function_regex.captures(line) {
        return Some(captures[1].to_string());
    }
    
    if let Some(captures) = local_function_regex.captures(line) {
        return Some(captures[1].to_string());
    }
    
    if let Some(captures) = method_regex.captures(line) {
        return Some(format!("{}.{}", &captures[1], &captures[2]));
    }
    
    if let Some(captures) = assignment_regex.captures(line) {
        return Some(captures[1].to_string());
    }
    
    None
}