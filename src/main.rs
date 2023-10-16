use std::{env, collections::VecDeque, path::PathBuf};
use config_manager::ConfigAdaptor;

macro_rules! op_default {
    ($optional:expr, $default:expr) => {
        match $optional {
            Some(value) => value,
            None => $default,
        }
    };
}


fn main() {
    // 没有配置文件则直接退出
    if !is_config_file_exists() { return; }

    let config = ConfigAdaptor::new_demo()
        .load()
        .expect(&format!("Cannot read {} properly!", ConfigAdaptor::SAVED_FILE_PATH));

    let argfiles = filter_existing_files(collect_args())
        .expect("No existing file found in the arguments passed!");

    for file in argfiles {
        let ext = file.extension().unwrap().to_string_lossy();
        let filename = file.to_string_lossy();
        match config.store.get(&ext.to_string()) {
            Some(el) => {
                let command = el.command.replace("{}", &filename);
                println!("{}> {}", op_default!(&el.name, ""), command);
                let _ = spawn_command(&command).expect("Failed to start command");
            },
            _ => println!(
                "Havn't found any command matched to ext {} for file {}, skipped.",
                ext, filename
            )
        }
    }

  
}

fn spawn_command(command: &str) -> Result<std::process::Child, std::io::Error> {
    let mut iter = command.split_ascii_whitespace();
    std::process::Command::new(iter.next().expect("Error when split command"))
        .args(iter)
        .spawn()
}

fn collect_args() -> VecDeque<String> {
    dbg!(env::args().collect())
}

fn filter_existing_files(mut args: VecDeque<String>) -> Option<Vec<PathBuf>> {
    // 删除自身路径
    args.pop_front();

    let files: Vec<PathBuf> = args.into_iter()
      .map(|x| PathBuf::from(x))
      .filter(|p| matches!(p.extension(), Some(_))) // 筛选有文件名的
      .filter(|p| p.exists()) // 筛选文件存在的
      .collect();
    
    
    match files.len() {
        0 => None,
        _ => Some(dbg!(files))
    }
}

fn is_config_file_exists() -> bool {
    let pass = std::path::Path::new(ConfigAdaptor::SAVED_FILE_PATH).exists();
    if !pass {
        print!("No config file found, created a demo at {}.\nPress Enter to exit.", ConfigAdaptor::SAVED_FILE_PATH);
        ConfigAdaptor::new_demo().save().expect(&format!("Cannot save demo {}", ConfigAdaptor::SAVED_FILE_PATH));
        let _ = std::io::stdin().read_line(&mut String::new());
    }
    
    pass
}


pub mod config_manager {

    use toml;
    use std::{collections::HashMap, fs};
    use serde_derive::{Serialize, Deserialize};

    type AnyError = Box<dyn std::error::Error>;

    /// config_manager 总结构，维持一个key-value配置对。
    /// 尽管store设置为pub不是很安全，但为了方便使用仍允许直接访问
    #[derive(Serialize, Deserialize, Debug)]
    pub struct ConfigAdaptor {
        pub trim_quotes: bool,
        pub store: HashMap<String, ConfigElement>
    }

    /// 真正储存配置信息的元素，储存在 ConfigAdaptor.store.values() 中。
    /// name: 为这个配置项写上一个可有可无的名字。
    /// command: 对应后缀名要执行的命令，以大花括号作为文件路径，直接原样format。
    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct ConfigElement {
        pub name: Option<String>,
        pub command: String
    }

    impl ConfigAdaptor {
        // 在这里定义配置文件的名字
        pub const SAVED_FILE_PATH: &str = "./config.toml";

        /// 新建一个空 ConfigAdaptor 实例。
        /// ConfigAdaptor.store 为 pub，可以直接访问修改，但不保证程序能够正常运行。
        pub fn new() -> ConfigAdaptor {
            ConfigAdaptor { store: HashMap::new(), trim_quotes: false }
        }
        pub fn new_demo() -> ConfigAdaptor {
            let mut demo = Self::new();
            demo.store.insert(
                "bat".to_string(), 
                ConfigElement { name: Some("Edit CMD Script".to_string()), command: "notepad {}".to_string() }
            );
            demo.store.insert(
                "ps1".to_string(), 
                ConfigElement { name: Some("Run Powershell Script".to_string()), command: "powershell {}".to_string() }
            );
            
            demo
        }

        /// 关联函数，读取配置文件后返回一个 ConfigAdaptor。
        pub fn from_file() -> Result<ConfigAdaptor, AnyError> {
            Ok(toml::from_str(
                fs::read_to_string(Self::SAVED_FILE_PATH)?.as_str()
            )?)
        }

        /// 保存当前实例为配置文件，会直接覆盖。
        pub fn save(&self) -> Result<(), AnyError> {
            let toml_str = toml::to_string(&self)?;
            fs::write(Self::SAVED_FILE_PATH, &toml_str)?;
            Ok(())
        }

        /// 销毁当前实例，并返回读取保存的配置实例。
        /// 剩余行为等同于 Self.from_file()。
        pub fn load(self) -> Result<ConfigAdaptor, AnyError> {
            // 传进self但不使用，self会直接被drop掉
            dbg!("Going to drop self...");
            Ok(Self::from_file()?)
        }

    }

    // impl Drop for ConfigAdaptor {
    //     fn drop(&mut self) {
    //         println!("Dropping |ConfigAdaptor| {:#?}!", self);
    //     }
    // }

    // impl Drop for ConfigElement {
    //     fn drop(&mut self) {
    //         println!("Dropping |ConfigElement| {:#?}!", self);
    //     }
    // }


}

