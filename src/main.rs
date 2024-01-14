use zenoh_msg_generator::log::*;
use zenoh_msg_generator::pkg::create_package;
use zenoh_msg_generator::file::*;

fn main()
{
    zmg_log_info("Start Generator".to_string());

    let option = std::env::args().nth(1).unwrap();
    let pkg_name = std::env::args().nth(2).unwrap();
    let msg = std::env::args().nth(3).unwrap();

    let msg_file_name = msg;

    if option == "new"
    {
        let _ = create_package(pkg_name.as_str());
        let _ = create_msg(pkg_name.as_str(), msg_file_name);

        zmg_log_info("Successfull generate".to_string());
    }
    else if option == "add"
    {
        let _ = add_msg(pkg_name.as_str(), msg_file_name);
        zmg_log_info("Successfull add message".to_string());
    }
    else
    {
        zmg_log_err("Please Add Option".to_string());
        return 
    }
}