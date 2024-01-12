use colored::{self, Colorize};

pub fn zmg_log_info(msg:String)
{
    println!("{}:{}", "[ZenohMsgGenerator]".bright_blue(), msg.bright_blue());
}

pub fn zmg_log_err(msg:String)
{
    println!("{}:{}", "[ZenohMsgGenerator]".bright_red(), msg.bright_red());
}