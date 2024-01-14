use colored::{self, Colorize};

pub fn zmg_log_info(msg:String)
{
    println!("{}:{}", "[ZenohMsgGenerator][INFO]".blue(), msg.blue());
}

pub fn zmg_log_err(msg:String)
{
    println!("{}:{}", "[ZenohMsgGenerator][ERROR]".red(), msg.red());
}