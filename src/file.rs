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

pub fn create_msg_file(pkg_name:&str)
{

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