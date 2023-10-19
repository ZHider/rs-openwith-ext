mod utils;
use utils::config_manager::{ConfigAdaptor, self};
#[macro_use]
mod service;
use service::*;

fn main() {
    // 获取系统参数
    let (exe_path, argfiles) = collect_args();
    let config_path = debug!(ConfigAdaptor::get_config_path(exe_path));

    // 没有配置文件则直接退出
    if !config_manager::is_config_file_exists(&config_path) { return; }

    // 获取配置信息
    let config = ConfigAdaptor::new_demo()
        .load(&config_path)
        .expect(&format!("Cannot read {} properly!", config_path.to_string_lossy()));

    // 检查路径合法性
    let argfiles = filter_existing_files(argfiles).expect("No existing file found in the arguments passed!");

    // 从配置文件中获取后缀名关联的配置信息
    let parsed_files = parse_files(argfiles.iter().map(|x| x.as_path()), &config);
    
    for (p, ce) in parsed_files {
        let _ = spawn_command(ce.command.split_whitespace().map(
            // 替换 {} 为文件路径
            |part| if part == "{}" {p.to_str().expect("Invalid UTF-8 path")} else {part}
        )).expect("Error when executing command");
    }
  
}
