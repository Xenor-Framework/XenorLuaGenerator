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
        .expect("Failed to read docs.json from current directory");
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
    
    println!("✓ Documentation generated in ./dist/");
    Ok(())
}

fn generate_css() -> Result<(), Box<dyn std::error::Error>> {
    let css = r#"* {
    margin: 0;
    padding: 0;
    box-sizing: border-box;
}

body {
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
    line-height: 1.6;
    color: #ffffff;
    background: #1c1c1e;
    background-image: 
        radial-gradient(circle at 25% 25%, rgba(44,44,46,0.3) 0%, transparent 50%),
        radial-gradient(circle at 75% 75%, rgba(44,44,46,0.2) 0%, transparent 50%),
        radial-gradient(circle at 50% 50%, rgba(28,28,30,0.8) 0%, rgba(44,44,46,0.1) 100%);
    position: relative;
}

body::before {
    content: '';
    position: fixed;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    background-image: 
        url("data:image/svg+xml,%3Csvg width='60' height='60' viewBox='0 0 60 60' xmlns='http://www.w3.org/2000/svg'%3E%3Cg fill='none' fill-rule='evenodd'%3E%3Cg fill='%23ffffff' fill-opacity='0.02'%3E%3Ccircle cx='7' cy='7' r='1'/%3E%3Ccircle cx='27' cy='17' r='1'/%3E%3Ccircle cx='47' cy='7' r='1'/%3E%3Ccircle cx='17' cy='37' r='1'/%3E%3Ccircle cx='37' cy='47' r='1'/%3E%3Ccircle cx='57' cy='27' r='1'/%3E%3C/g%3E%3C/g%3E%3C/svg%3E");
    pointer-events: none;
    z-index: -1;
}

.container {
    display: flex;
    min-height: 100vh;
}

.sidebar {
    width: 280px;
    background: rgba(44,44,46,0.8);
    backdrop-filter: blur(20px);
    -webkit-backdrop-filter: blur(20px);
    color: #ffffff;
    padding: 20px;
    overflow-y: auto;
    position: fixed;
    height: 100vh;
    border-right: 1px solid rgba(255,255,255,0.1);
}

.search-container {
    margin-bottom: 20px;
}

.search-box {
    width: 100%;
    padding: 12px 16px;
    border: 1px solid rgba(255,255,255,0.1);
    border-radius: 12px;
    background: rgba(58,58,60,0.6);
    color: #ffffff;
    font-size: 14px;
    transition: all 0.2s ease;
}

.search-box:focus {
    outline: none;
    border-color: rgba(0,122,255,0.6);
    background: rgba(58,58,60,0.8);
}

.search-box::placeholder {
    color: rgba(255,255,255,0.6);
}

.nav-section {
    margin-bottom: 20px;
}

.nav-title {
    font-size: 16px;
    font-weight: 600;
    margin-bottom: 8px;
    color: rgba(255,255,255,0.9);
    cursor: pointer;
    padding: 8px 12px;
    border-radius: 8px;
    transition: background 0.2s ease;
    display: flex;
    align-items: center;
    justify-content: space-between;
}

.nav-title:hover {
    background: rgba(255,255,255,0.05);
}

.nav-title::after {
    content: '▼';
    font-size: 10px;
    color: rgba(255,255,255,0.6);
    transition: transform 0.2s ease;
}

.nav-section.collapsed .nav-title::after {
    transform: rotate(-90deg);
}

.nav-list {
    list-style: none;
    overflow: hidden;
    transition: max-height 0.3s ease;
    max-height: 1000px;
}

.nav-section.collapsed .nav-list {
    max-height: 0;
}

.nav-item {
    margin-bottom: 2px;
}

.nav-link {
    color: rgba(255,255,255,0.8);
    text-decoration: none;
    display: block;
    padding: 8px 16px;
    margin-left: 12px;
    border-radius: 8px;
    transition: all 0.2s ease;
    font-size: 14px;
    position: relative;
}

.nav-link:hover {
    background: rgba(255,255,255,0.08);
    color: #ffffff;
}

.nav-link.active {
    background: rgba(0,122,255,0.2);
    color: #007AFF;
    border-left: 3px solid #007AFF;
    padding-left: 13px;
}

.content {
    margin-left: 280px;
    flex: 1;
    padding: 30px;
    max-width: 1200px;
}

.page-title {
    font-size: 34px;
    font-weight: 700;
    margin-bottom: 30px;
    color: #ffffff;
    border-bottom: 2px solid rgba(0,122,255,0.3);
    padding-bottom: 16px;
}

.function {
    background: rgba(44,44,46,0.6);
    backdrop-filter: blur(20px);
    -webkit-backdrop-filter: blur(20px);
    border: 1px solid rgba(255,255,255,0.1);
    border-radius: 16px;
    padding: 24px;
    margin-bottom: 24px;
    transition: all 0.3s ease;
}

.function:hover {
    background: rgba(44,44,46,0.8);
    border-color: rgba(255,255,255,0.2);
    transform: translateY(-2px);
    box-shadow: 0 8px 32px rgba(0,0,0,0.3);
}

.function-header {
    display: flex;
    align-items: baseline;
    margin-bottom: 16px;
}

.function-name {
    font-size: 24px;
    font-weight: 700;
    color: #ffffff;
    margin-right: 12px;
}

.function-id {
    color: rgba(255,255,255,0.6);
    font-size: 13px;
    font-family: 'SF Mono', 'Monaco', 'Inconsolata', 'Roboto Mono', monospace;
}

.function-description {
    color: rgba(255,255,255,0.8);
    margin-bottom: 20px;
    font-size: 16px;
    line-height: 1.5;
}

.params-section, .returns-section {
    margin-top: 20px;
}

.section-title {
    font-size: 18px;
    font-weight: 600;
    color: #ffffff;
    margin-bottom: 12px;
}

.param-list, .return-list {
    background: rgba(28,28,30,0.6);
    border-left: 3px solid #007AFF;
    padding: 16px;
    border-radius: 12px;
    border: 1px solid rgba(255,255,255,0.05);
}

.param-item, .return-item {
    margin-bottom: 12px;
}

.param-item:last-child, .return-item:last-child {
    margin-bottom: 0;
}

.param-name, .return-type {
    font-weight: 600;
    color: #FF3B30;
    font-family: 'SF Mono', 'Monaco', 'Inconsolata', 'Roboto Mono', monospace;
    font-size: 14px;
}

.param-type {
    color: #007AFF;
    font-family: 'SF Mono', 'Monaco', 'Inconsolata', 'Roboto Mono', monospace;
    margin-left: 8px;
    font-size: 14px;
}

.param-desc, .return-desc {
    color: rgba(255,255,255,0.8);
    margin-left: 20px;
    margin-top: 4px;
    line-height: 1.4;
}

.empty-state {
    color: rgba(255,255,255,0.5);
    font-style: italic;
}
    
.footer {
    margin-top: 60px;
    padding: 24px 0;
    border-top: 1px solid rgba(255,255,255,0.1);
    text-align: center;
}

.copyright {
    color: rgba(255,255,255,0.5);
    font-size: 14px;
    font-weight: 400;
}

.copyright a {
    color: #007AFF;
    text-decoration: none;
    transition: color 0.2s ease;
}

.copyright a:hover {
    color: #0051D0;
}
"#;
    
    let mut file = fs::File::create("dist/style.css")?;
    file.write_all(css.as_bytes())?;
    Ok(())
}

fn generate_search_script() -> Result<(), Box<dyn std::error::Error>> {
    let script = r#"
    
    function initSearch() {
    const searchBox = document.getElementById('search');
    const functions = document.querySelectorAll('.function');
    const navItems = document.querySelectorAll('.nav-item');
    
    document.querySelectorAll('.nav-title').forEach(title => {
        title.addEventListener('click', function() {
            const section = this.parentElement;
            section.classList.toggle('collapsed');
        });
    });
    
    searchBox.addEventListener('input', function() {
        const query = this.value.toLowerCase();
        
        functions.forEach(func => {
            const name = func.dataset.name.toLowerCase();
            const desc = func.dataset.description.toLowerCase();
            
            if (name.includes(query) || desc.includes(query)) {
                func.style.display = 'block';
            } else {
                func.style.display = 'none';
            }
        });
        
        navItems.forEach(item => {
            const link = item.querySelector('.nav-link');
            const name = link.textContent.toLowerCase();
            
            if (name.includes(query)) {
                item.style.display = 'block';
                const section = item.closest('.nav-section');
                section.classList.remove('collapsed');
            } else {
                item.style.display = 'none';
            }
        });
    });
    
    const observer = new IntersectionObserver(entries => {
        entries.forEach(entry => {
            if (entry.isIntersecting) {
                const id = entry.target.id;
                document.querySelectorAll('.nav-link').forEach(link => {
                    link.classList.remove('active');
                    if (link.getAttribute('href') === '#' + id) {
                        link.classList.add('active');
                        const section = link.closest('.nav-section');
                        section.classList.remove('collapsed');
                    }
                });
            }
        });
    }, { threshold: 0.5 });
    
    functions.forEach(func => observer.observe(func));
}

document.addEventListener('DOMContentLoaded', initSearch);"#;
    
    let mut file = fs::File::create("dist/search.js")?;
    file.write_all(script.as_bytes())?;
    Ok(())
}

fn generate_category_page(
    category: &str,
    functions: &[Function],
    all_docs: &Documentation
) -> Result<(), Box<dyn std::error::Error>> {
    let filename = format!("dist/{}.html", category.to_lowercase());
    let mut file = fs::File::create(filename)?;
    
    let mut html = String::new();
    html.push_str(&format!(r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{} - Documentation</title>
    <link rel="stylesheet" href="style.css">
</head>
<body>
    <div class="container">
        <aside class="sidebar">
            <div class="search-container">
                <input type="text" id="search" class="search-box" placeholder="Search...">
            </div>
            <nav class="navigation">"#, category));

    for (cat_name, cat_functions) in all_docs {
        html.push_str(&format!(r#"
                <div class="nav-section">
                    <div class="nav-title">{}</div>
                    <ul class="nav-list">"#, cat_name));
        
        for func in cat_functions {
            let href = if cat_name == category {
                format!("#{}", func.name.to_lowercase())
            } else {
                format!("{}.html#{}", cat_name.to_lowercase(), func.name.to_lowercase())
            };
            
            html.push_str(&format!(r#"
                        <li class="nav-item">
                            <a href="{}" class="nav-link">{}</a>
                        </li>"#, href, func.name));
        }
        
        html.push_str(r#"
                    </ul>
                </div>"#);
    }
    
    html.push_str(r#"
            </nav>
        </aside>
        
        <main class="content">
            <h1 class="page-title">"#);
    html.push_str(category);
    html.push_str(r#"</h1>"#);
    
    // Functions
    for func in functions {
        html.push_str(&format!(r#"
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
            html.push_str(r#"
                <div class="params-section">
                    <h3 class="section-title">Parameters</h3>
                    <div class="param-list">"#);
            
            for param in &func.params {
                html.push_str(&format!(r#"
                        <div class="param-item">
                            <span class="param-name">{}</span>
                            <span class="param-type">{}</span>
                            <div class="param-desc">{}</div>
                        </div>"#,
                    param.name, param.param_type, param.description
                ));
            }
            
            html.push_str(r#"
                    </div>
                </div>"#);
        } else {
            html.push_str(r#"
                <div class="params-section">
                    <h3 class="section-title">Parameters</h3>
                    <div class="empty-state">No parameters</div>
                </div>"#);
        }
        
        // Returns
        if !func.returns.is_empty() {
            html.push_str(r#"
                <div class="returns-section">
                    <h3 class="section-title">Returns</h3>
                    <div class="return-list">"#);
            
            for ret in &func.returns {
                html.push_str(&format!(r#"
                        <div class="return-item">
                            <span class="return-type">{}</span>
                            <div class="return-desc">{}</div>
                        </div>"#,
                    ret.return_type, ret.description
                ));
            }
            
            html.push_str(r#"
                    </div>
                </div>"#);
        } else {
            html.push_str(r#"
                <div class="returns-section">
                    <h3 class="section-title">Returns</h3>
                    <div class="empty-state">No return value</div>
                </div>"#);
        }
        
        html.push_str(r#"
            </div>"#);
    }
    
    html.push_str(r#"
            <footer class="footer">
                <div class="copyright">
                    © 2025 XenorSDK BSD 3-Clause https://github.com/Xenor-Framework
                </div>
            </footer>
        </main>
    </div>
    <script src="search.js"></script>
</body>
</html>"#);
    
    file.write_all(html.as_bytes())?;
    Ok(())
}

fn generate_index_redirect(first_category: &str) -> Result<(), Box<dyn std::error::Error>> {
    let html = format!(r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta http-equiv="refresh" content="0; url={}.html">
    <title>Documentation</title>
</head>
<body>
    <p>Redirecting to documentation...</p>
</body>
</html>"#, first_category.to_lowercase());
    
    let mut file = fs::File::create("dist/index.html")?;
    file.write_all(html.as_bytes())?;
    Ok(())
}