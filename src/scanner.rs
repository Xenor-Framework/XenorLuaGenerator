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

#[derive(Debug)]
struct DocBlock {
    class_name: Option<String>,
    description: String,
    params: Vec<Param>,
    returns: Vec<Return>,
    start_line: usize,
}

impl DocBlock {
    fn new(start_line: usize) -> Self {
        Self {
            class_name: None,
            description: String::new(),
            params: Vec::new(),
            returns: Vec::new(),
            start_line,
        }
    }
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

fn is_doc_comment(line: &str) -> bool {
    let trimmed = line.trim_start();
    trimmed.starts_with("--@") || trimmed.starts_with("-- @") || 
    (trimmed.starts_with("--") && !trimmed.starts_with("---") && !trimmed.starts_with("-- TODO") && !trimmed.starts_with("-- FIXME"))
}

fn extract_doc_content(line: &str) -> String {
    line.trim_start()
        .trim_start_matches("--@")
        .trim_start_matches("-- @")
        .trim_start_matches("--")
        .trim()
        .to_string()
}

fn categorize_function(func_name: &str, class_name: &Option<String>) -> (String, String) {
    if func_name.contains('.') {
        let parts: Vec<&str> = func_name.splitn(2, '.').collect();
        (parts[0].to_string(), parts[1].to_string())
    } else {
        let category = class_name.clone().unwrap_or_else(|| "Global".to_string());
        (category, func_name.to_string())
    }
}

fn parse_function_doc(lines: &[&str], index: &mut usize) -> Option<(String, Function)> {
    let mut doc_block = DocBlock::new(*index);

    while *index < lines.len() && is_doc_comment(lines[*index]) {
        let content = extract_doc_content(lines[*index]);
        
        if let Some(tag_content) = content.strip_prefix("class ") {
            doc_block.class_name = Some(tag_content.trim().to_string());
        } else if let Some(tag_content) = content.strip_prefix("desc ") {
            if doc_block.description.is_empty() {
                doc_block.description = tag_content.trim().to_string();
            } else {
                doc_block.description.push(' ');
                doc_block.description.push_str(tag_content.trim());
            }
        } else if let Some(tag_content) = content.strip_prefix("param ") {
            if let Some(param) = parse_param(tag_content) {
                doc_block.params.push(param);
            }
        } else if let Some(tag_content) = content.strip_prefix("return ") {
            if let Some(ret) = parse_return(tag_content) {
                doc_block.returns.push(ret);
            }
        } else if content.starts_with('@') {
            continue;
        } else if !content.trim().is_empty() && doc_block.description.is_empty() {
            doc_block.description = content.trim().to_string();
        }
        
        *index += 1;
    }
    
    while *index < lines.len() && lines[*index].trim().is_empty() {
        *index += 1;
    }

    for lookahead in 0..3 {
        if *index + lookahead >= lines.len() {
            break;
        }
        
        if let Some(func_name) = extract_function_name(lines[*index + lookahead]) {
            let (category, name) = categorize_function(&func_name, &doc_block.class_name);
            
            return Some((category, Function {
                name,
                description: doc_block.description,
                params: doc_block.params,
                returns: doc_block.returns,
            }));
        }
    }
    
    None
}

fn parse_return(content: &str) -> Option<Return> {
    let content = content.trim();
    
    if let Some(comma_pos) = content.find(',') {
        let return_type = content[..comma_pos].trim().to_string();
        let description = content[comma_pos + 1..].trim().to_string();
        return Some(Return { return_type, description });
    }
    
    if let Some(space_pos) = content.find(' ') {
        let return_type = content[..space_pos].trim().to_string();
        let description = content[space_pos + 1..].trim().to_string();
        return Some(Return { return_type, description });
    }
    
    Some(Return {
        return_type: content.to_string(),
        description: String::new(),
    })
}

fn parse_param(content: &str) -> Option<Param> {
    let content = content.trim();
    
    if let Some(colon_pos) = content.find(':') {
        let name = content[..colon_pos].trim().to_string();
        let rest = &content[colon_pos + 1..];
        
        if let Some(space_pos) = rest.find(' ') {
            let param_type = rest[..space_pos].trim().to_string();
            let description = rest[space_pos + 1..].trim().to_string();
            return Some(Param { name, param_type, description });
        } else {
            let param_type = rest.trim().to_string();
            return Some(Param { name, param_type, description: String::new() });
        }
    }
    
    let parts: Vec<&str> = content.splitn(3, ',').map(|s| s.trim()).collect();
    if parts.len() >= 2 {
        return Some(Param {
            name: parts[0].to_string(),
            param_type: parts[1].to_string(),
            description: parts.get(2).unwrap_or(&"").to_string(),
        });
    }
    
    let words: Vec<&str> = content.split_whitespace().collect();
    if words.len() >= 2 {
        return Some(Param {
            name: words[0].to_string(),
            param_type: words[1].to_string(),
            description: words[2..].join(" "),
        });
    }
    
    None
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