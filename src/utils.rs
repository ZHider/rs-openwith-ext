pub mod config_manager {

    use toml;
    use std::{collections::HashMap, fs, path::{Path, PathBuf}};
    use serde_derive::{Serialize, Deserialize};

    type AnyError = Box<dyn std::error::Error>;

    // 检测config文件是否存在
    pub fn is_config_file_exists(path: &Path) -> bool {
        let pass = path.exists();
        let path_display = path.to_string_lossy();
        if !pass {
            println!("No config file found, created a demo at {}.", path_display);
            ConfigAdaptor::new_demo().save(path).expect(&format!("Cannot save demo {}", path_display));
        }
    
        pass
    }

    /// 真正储存配置信息的元素，储存在 ConfigAdaptor.store.values() 中。
    /// comment: 由于程序不能保存注释，你可以在这里写上你的注释。
    /// command: 对应后缀名要执行的命令，以大花括号作为文件路径，直接原样format。
    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct ConfigElement {
        pub comment: Option<String>,
        pub command: String
    }

    /// config_manager 总结构，维持一个key-value配置对。
    /// 尽管suffix设置为pub不是很安全，但为了方便使用仍允许直接访问
    #[derive(Serialize, Deserialize, Debug)]
    pub struct ConfigAdaptor {
        pub suffix: HashMap<String, ConfigElement>
    }

    impl ConfigAdaptor {

        const CONFIG_FILE_NAME: &str = "config.toml";

        /// 新建一个空 ConfigAdaptor 实例。
        /// ConfigAdaptor.store 为 pub，可以直接访问修改，但不保证程序能够正常运行。
        pub fn new() -> ConfigAdaptor {
            ConfigAdaptor { suffix: HashMap::new() }
        }
        pub fn new_demo() -> ConfigAdaptor {
            let mut demo = Self::new();
            demo.suffix.insert(
                "bat".to_string(), 
                ConfigElement { comment: Some("Edit CMD Script".to_string()), command: "notepad {}".to_string() }
            );
            demo.suffix.insert(
                "ps1".to_string(), 
                ConfigElement { comment: Some("Run Powershell Script".to_string()), command: "powershell -File {}".to_string() }
            );
            
            demo
        }

        pub fn get_config_path(path: impl Into<PathBuf>) -> PathBuf {
            let mut path: PathBuf = path.into();
            path.pop();
            path.push(Self::CONFIG_FILE_NAME);

            path
        }

        /// 关联函数，读取配置文件后返回一个 ConfigAdaptor。
        pub fn from_file(path: &Path) -> Result<ConfigAdaptor, AnyError> {
            Ok(toml::from_str(
                fs::read_to_string(path)?.as_str()
            )?)
        }

        /// 保存当前实例为配置文件，会直接覆盖。
        pub fn save(&self, path: &Path) -> Result<(), AnyError> {
            let toml_str = toml::to_string(&self)?;
            fs::write(path, &toml_str)?;
            Ok(())
        }

        /// 销毁当前实例，并返回读取保存的配置实例。
        /// 剩余行为等同于 Self.from_file()。
        pub fn load(self, path: &Path) -> Result<ConfigAdaptor, AnyError> {
            // 传进self但不使用，self会直接被drop掉
            Ok(Self::from_file(path)?)
        }

    }

}
