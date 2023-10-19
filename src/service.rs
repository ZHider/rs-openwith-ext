// macro_rules! op_default {
//     ($optional:expr, $default:expr) => {
//         match $optional {
//             Some(value) => value,
//             None => $default,
//         }
//     };
// }

macro_rules! debug {
    ($($e:expr),+) => {
        {
            #[cfg(debug_assertions)]
            {
                dbg!($($e),+)
            }
            #[cfg(not(debug_assertions))]
            {
                ($($e),+)
            }
        }
    };
}  


use std::{env::{self, Args}, path::{PathBuf, Path}};
use crate::utils::config_manager::{ConfigAdaptor, ConfigElement};

/// 运行一个command
pub fn spawn_command<'a>(
    mut command: impl Iterator<Item = &'a str>
) -> Result<std::process::Child, std::io::Error> {
    std::process::Command::new(command.next().expect("Error when split command"))
        .args(command)
        .spawn()
}

/// 获取运行参数
/// 返回：运行文件路径、其余参数
pub fn collect_args() -> (PathBuf, Args) {
    let mut iter = debug!(env::args());
    let current_file = iter.next().expect("Invalid utf-8 str when reading args");
    (PathBuf::from(current_file), iter)
}

/// 过滤运行参数，只保留存在的、有后缀名的文件
pub fn filter_existing_files(files: impl Iterator<Item = String>) -> Option<Vec<PathBuf>> {

    let files: Vec<PathBuf> = files
        .map(|x| PathBuf::from(x))
        .filter(|p| matches!(p.extension(), Some(_))) // 筛选有后缀名的
        .filter(|p| p.exists()) // 筛选文件存在的
        .collect();
    
    
    match files.len() {
        0 => None,
        _ => Some(debug!(files))
    }
}

/// 对每个Path判断命令并操作
pub fn parse_files<'a, 'b, P>(
    files: P, config: &'b ConfigAdaptor
) -> Vec<(&'a Path, &'b ConfigElement)>
where
    P: IntoIterator<Item = &'a Path>,
    P::Item: AsRef<Path>,
{
    let mut result: Vec<(&Path, &'b ConfigElement)> = Vec::new();

    for file in files {
        let filename = file.to_string_lossy();
        let ext = file.extension().unwrap().to_string_lossy();

        if let Some(el) = config.suffix.get(&ext.to_string()) {
            result.push((file, el));

        } else {
            println!(
                "Havn't found any command matched to ext {} for file {}, skipped.",
                ext, filename
            )
        }
    }

    result
}

