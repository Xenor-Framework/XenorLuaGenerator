use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::path::Path;

mod scanner;

#[derive(Debug, Deserialize, Serialize)]
struct Param {
    name: String,
    #[serde(rename = "type")]
    param_type: String,
    description: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Return {
    #[serde(rename = "type")]
    return_type: String,
    description: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Function {
    name: String,
    description: String,
    params: Vec<Param>,
    returns: Vec<Return>,
}

type Documentation = HashMap<String, Vec<Function>>;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let json_content = fs::read_to_string("docs.json")
        .expect("[ ERROR ] Failed to read docs.json from current directory");
    let docs: Documentation = serde_json::from_str(&json_content)?;
    
    let dist_path = Path::new("dist");
    if dist_path.exists() {
        fs::remove_dir_all(dist_path)?;
    }
    fs::create_dir(dist_path)?;
    
    generate_css()?;
    
    generate_search_script()?;
    
    for (category, functions) in &docs {
        generate_category_page(category, functions, &docs)?;
    }
    
    if let Some(first_category) = docs.keys().next() {
        generate_index_redirect(first_category)?;
    }
    
    println!("[ OK ] Documentation generated in ./dist/");
    Ok(())
}

fn generate_css() -> Result<(), Box<dyn std::error::Error>> {
    let template_content = fs::read_to_string("template/style.css")
        .expect("Failed to read template/style.css");
    
    let mut file = fs::File::create("dist/style.css")?;
    file.write_all(template_content.as_bytes())?;
    Ok(())
}

fn generate_search_script() -> Result<(), Box<dyn std::error::Error>> {
    let template_content = fs::read_to_string("template/search.js")
        .expect("Failed to read template/search.js");
    
    let mut file = fs::File::create("dist/search.js")?;
    file.write_all(template_content.as_bytes())?;
    Ok(())
}

fn generate_category_page(
    category: &str,
    functions: &[Function],
    all_docs: &Documentation
) -> Result<(), Box<dyn std::error::Error>> {
    let filename = format!("dist/{}.html", category.to_lowercase());
    let mut file = fs::File::create(filename)?;
    
    let template = fs::read_to_string("template/category.html")
        .expect("Failed to read template/category.html");
    
    let mut navigation = String::new();
    for (cat_name, cat_functions) in all_docs {
        navigation.push_str(&format!(r#"
                <div class="nav-section">
                    <div class="nav-title">{}</div>
                    <ul class="nav-list">"#, cat_name));
        
        for func in cat_functions {
            let href = if cat_name == category {
                format!("#{}", func.name.to_lowercase())
            } else {
                format!("{}.html#{}", cat_name.to_lowercase(), func.name.to_lowercase())
            };
            
            navigation.push_str(&format!(r#"
                        <li class="nav-item">
                            <a href="{}" class="nav-link">{}</a>
                        </li>"#, href, func.name));
        }
        
        navigation.push_str(r#"
                    </ul>
                </div>"#);
    }
    
    // Functions
    let mut functions_html = String::new();
    for func in functions {
        functions_html.push_str(&format!(r#"
            <div class="function" id="{}" data-name="{}" data-description="{}">
                <div class="function-header">
                    <h2 class="function-name">{}</h2>
                    <span class="function-id">{}:{}</span>
                </div>
                <p class="function-description">{}</p>"#,
            func.name.to_lowercase(),
            func.name,
            func.description,
            func.name,
            category,
            func.name,
            func.description
        ));
        
        // Parameters
        if !func.params.is_empty() {
            functions_html.push_str(r#"
                <div class="params-section">
                    <h3 class="section-title">Parameters</h3>
                    <div class="param-list">"#);
            
            for param in &func.params {
                functions_html.push_str(&format!(r#"
                        <div class="param-item">
                            <span class="param-name">{}</span>
                            <span class="param-type">{}</span>
                            <div class="param-desc">{}</div>
                        </div>"#,
                    param.name, param.param_type, param.description
                ));
            }
            
            functions_html.push_str(r#"
                    </div>
                </div>"#);
        } else {
            functions_html.push_str(r#"
                <div class="params-section">
                    <h3 class="section-title">Parameters</h3>
                    <div class="empty-state">No parameters</div>
                </div>"#);
        }
        
        // Returns
        if !func.returns.is_empty() {
            functions_html.push_str(r#"
                <div class="returns-section">
                    <h3 class="section-title">Returns</h3>
                    <div class="return-list">"#);
            
            for ret in &func.returns {
                functions_html.push_str(&format!(r#"
                        <div class="return-item">
                            <span class="return-type">{}</span>
                            <div class="return-desc">{}</div>
                        </div>"#,
                    ret.return_type, ret.description
                ));
            }
            
            functions_html.push_str(r#"
                    </div>
                </div>"#);
        } else {
            functions_html.push_str(r#"
                <div class="returns-section">
                    <h3 class="section-title">Returns</h3>
                    <div class="empty-state">No return value</div>
                </div>"#);
        }
        
        functions_html.push_str(r#"
            </div>"#);
    }
    
    let html = template
        .replace("{{category}}", category)
        .replace("{{navigation}}", &navigation)
        .replace("{{functions}}", &functions_html);
    
    file.write_all(html.as_bytes())?;
    Ok(())
}

fn generate_index_redirect(first_category: &str) -> Result<(), Box<dyn std::error::Error>> {
    let template = fs::read_to_string("template/index.html")
        .expect("Failed to read template/index.html");
    
    let html = template.replace("{{first_category}}", &first_category.to_lowercase());
    
    let mut file = fs::File::create("dist/index.html")?;
    file.write_all(html.as_bytes())?;
    Ok(())
}