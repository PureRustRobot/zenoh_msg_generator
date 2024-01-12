use std::io::Error;
use crate::log::*;
use std::fs;

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

pub fn create_msg(pkg_name:&str, msg_name:&str)
{
    let _ = create_lib_rs(pkg_name);
    let _ = create_msg_rs(pkg_name, msg_name);
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
fn create_msg_rs(pkg_name:&str, msg_name:&str)->Result<(), Error>
{
    let pkg_path = format!("../{}", pkg_name);

    let lib_path = format!("{}/src/msg.rs", &pkg_path);
    match fs::write(lib_path, format!("pub mod {};\n", msg_name))
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

fn _content_of_msg_rs(msg_file_content:String)->String
{
    let start = "use serde::{Serialize, Deserialize};\n\n".to_string();
    
    format!("{}{}", start, msg_file_content)
}