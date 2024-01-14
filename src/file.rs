use std::io::{
    Error,
    BufRead,
    BufReader
};
use crate::log::*;
use std::fs::{self, File};


pub fn get_msg_file(file_name:String)->Result<String, Error>
{

    let index_point = file_name.find('.').unwrap();
    let file_type = file_name.get(index_point..).unwrap();
    if file_type != ".msg"
    {
        zmg_log_err("Failed to find .msg file".to_string());
        return Err(std::io::Error::new(std::io::ErrorKind::NotFound, "Different file types"))
    }
    else {
        zmg_log_info(format!("Find {}", file_name));
        Ok(file_name)
    }
}

pub fn create_msg(pkg_name:&str, msg_file_path:String)
{
    let _ = create_lib_rs(pkg_name);
    let _ = create_msg_rs(pkg_name, msg_file_path);
}

fn create_lib_rs(pkg_name:&str)->Result<(), Error>
{
    let pkg_path = format!("../{}", pkg_name);

    let lib_path = format!("{}/src/lib.rs", &pkg_path);
    match fs::write(lib_path, "pub mod msg;\n")
    {
        Ok(_)=>{
            zmg_log_info("create lib.rs".to_string());
            Ok(())
        },
        Err(_)=>{
            zmg_log_err("Failed to create lib.rs".to_string());
            return Err(std::io::Error::new(std::io::ErrorKind::NotFound, "Different file types"));
        }
    }
}
fn create_msg_rs(pkg_name:&str, msg_file_path:String)->Result<(), Error>
{
    let pkg_path = format!("../{}", pkg_name);

    let lib_path = format!("{}/src/msg.rs", &pkg_path);
    match fs::write(lib_path, content_of_msg_rs(msg_file_path))
    {
        Ok(_)=>{
            zmg_log_info("create msg.rs".to_string());
            Ok(())
        },
        Err(_)=>{
            zmg_log_err("Failed to create msg.rs".to_string());
            return Err(std::io::Error::new(std::io::ErrorKind::NotFound, "Different file types"));
        }
    }
}

fn content_of_msg_rs(msg_file_path:String)->String
{
    let start = "use serde::{Serialize, Deserialize};\n\n".to_string();

    let m_f_p = format!("./{}", msg_file_path);

    let name_index = msg_file_path.find(".").unwrap();
    let struct_name = msg_file_path.get(0..name_index).unwrap();

    let name_str = format!("#[derive(Serialize, Deserialize)]\npub struct {}{{\n", struct_name);

    let mut contents = Vec::<String>::new();

    for result in BufReader::new(File::open(m_f_p).unwrap()).lines()
    {
        let get_str = result.unwrap();
        let space_index = get_str.find(" ").unwrap();
        let comp_name = get_str.get(space_index..).unwrap();
        match get_str.get(0..space_index).unwrap()
        {
            "string"=>{
                let comp = format!("    {}:String,\n", comp_name);
                contents.push(comp);
            },
            "float32"=>{
                let comp = format!("    {}:f32,\n", comp_name);
                contents.push(comp);
            },
            "float64"=>{
                let comp = format!("    {}:f64,\n", comp_name);
                contents.push(comp);
            },
            "int32"=>{
                let comp = format!("    {}:i32,\n", comp_name);
                contents.push(comp);
            },
            "int64"=>{
                let comp = format!("    {}:i64,\n", comp_name);
                contents.push(comp);
            },
            "bool"=>{
                let comp = format!("    {}:bool,\n", comp_name);
                contents.push(comp);
            },
            _=>{}
        }
    }

    let mut result = format!("{}{}", start, name_str);

    for i in 0..contents.len()
    {
        result = format!("{}{}", result, contents[i]);
    }

    zmg_log_info("Get .msg file contents".to_string());

    result = format!("{}}}", result);

    result = format!("{}\npub fn serialize_{}(value:&{})->String\n{{\n    serde_json::to_string(value).unwrap()\n{}", result, struct_name.to_lowercase(), struct_name, "}");
    result = format!("{}\npub fn deserialize_{}(str_value:String)->{}\n{{\n     let result:{} = serde_json::from_str(&str_value).unwrap();\n    result\n}}", result, struct_name.to_lowercase(), struct_name, struct_name);

    result
}

pub fn add_msg(pkg_name:&str, msg_file:String)->Result<(), Error>
{
    let pkg_path = format!("../{}/src/msg.rs", pkg_name);
    let msg_file_path = format!("./{}", msg_file);

    let mut content = Vec::<String>::new();

    let mut result ="".to_string();

    for result in BufReader::new(File::open(&pkg_path).unwrap()).lines()
    {
        content.push(format!("{}\n", result.unwrap()))
    }

    let name_index = msg_file.find(".").unwrap();
    let struct_name = msg_file.get(0..name_index).unwrap();

    let name_str = format!("#[derive(Serialize, Deserialize)]\npub struct {}{{\n", struct_name);

    content.push(name_str);

    for result in BufReader::new(File::open(&msg_file_path).unwrap()).lines()
    {
        let get_str = result.unwrap();
        let space_index = get_str.find(" ").unwrap();
        let comp_name = get_str.get(space_index..).unwrap();
        match get_str.get(0..space_index).unwrap()
        {
            "string"=>{
                let comp = format!("    {}:String,\n", comp_name);
                content.push(comp);
            },
            "float32"=>{
                let comp = format!("    {}:f32,\n", comp_name);
                content.push(comp);
            },
            "float64"=>{
                let comp = format!("    {}:f64,\n", comp_name);
                content.push(comp);
            },
            "int32"=>{
                let comp = format!("    {}:i32,\n", comp_name);
                content.push(comp);
            },
            "int64"=>{
                let comp = format!("    {}:i64,\n", comp_name);
                content.push(comp);
            },
            "bool"=>{
                let comp = format!("    {}:bool,\n", comp_name);
                content.push(comp);
            },
            _=>{}
        }
    }

    for u in 0..content.len()
    {
        result = format!("{}{}", result, content[u]);
    }

    result = format!("{}}}", result);

    result = format!("{}\npub fn serialize_{}(value:&{})->String\n{{\n    serde_json::to_string(value).unwrap()\n{}", result, struct_name.to_lowercase(), struct_name, "}");
    result = format!("{}\npub fn deserialize_{}(str_value:String)->{}\n{{\n     let result:{} = serde_json::from_str(&str_value).unwrap();\n    result\n}}", result, struct_name.to_lowercase(), struct_name, struct_name);

    match fs::write(pkg_path, result)
    {
        Ok(_)=>{
            zmg_log_info("create lib.rs".to_string());
            Ok(())
        },
        Err(_)=>{
            zmg_log_err("Failed to create lib.rs".to_string());
            return Err(std::io::Error::new(std::io::ErrorKind::NotFound, "Different file types"));
        }
    }
}