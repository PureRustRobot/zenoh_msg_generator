use std::fs;

pub fn create_package(pkg_name:&str)->std::io::Result<()>
{
    let dir_path = format!("../{}", pkg_name);
    fs::create_dir(&dir_path)?;
    let cargo_toml_path = format!("{}/Cargo.toml", dir_path);
    let _  = fs::write(cargo_toml_path, create_toml_pkg_info(pkg_name));
    Ok(())
}

fn create_toml_pkg_info(pkg_name:&str)->String
{
    let start = "[package]\n".to_string();
    let name_info = format!("name = \"{}\"\n", pkg_name);
    let version_info = "version = \"0.1.0\"\n".to_string();
    let edition_info = "edition = \"2021\"\n\n".to_string();
    let add_info = "# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html\n\n".to_string();

    let result = format!("{}{}{}{}{}", start, name_info, version_info, edition_info, add_info);

    result
}

fn create_toml_depen_info(dependencies:Vec<String>)
{
    let start = "[dependencies]\n".to_string();
    let serde_info = "serde = {version = \"\", features = [\"derive\"]}\n".to_string();
    
}