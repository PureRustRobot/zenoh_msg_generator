use std::io::Error;
use colored::{Colorize, self};

pub fn get_msg_file()->Result<String, Error>
{
    let arg = std::env::args().nth(1).unwrap();

    let index_point = arg.find('.').unwrap();
    let file_type = arg.get(index_point..).unwrap();
    if file_type != ".msg"
    {
        println!("{}", "[ZenohMsgGenerator]Failed to generate message".bright_red());
        return Err(std::io::Error::new(std::io::ErrorKind::NotFound, "Different file types"))
    }
    else {
        Ok(arg)
    }
}

