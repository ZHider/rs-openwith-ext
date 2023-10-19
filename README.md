# rs-openwith-ext

## 描述

一个使用 rust 开发的小工具，用于对于不同后缀名的文件自定义打开时运行的命令行。

程序接受文件路径作为参数，从配置文件中读取对应后缀名的打开命令，然后运行。

默认配置文件：

```toml

[suffix.ps1]
comment = "Run Powershell Script"
command = "powershell -File {}"

[suffix.bat]
comment = "Edit CMD Script"
command = "notepad {}"

```

需要更改和增加配置，需要自行修改配置文件，增加或修改表项如下：

```toml
[suffix.<自定义后缀名>]
comment = "自定义注释"
command = "自定义命令"
```

程序在打开时会自动将命令行中的 `{}` 替换成文件路径。
命令将以空格分隔直接传递给应用。


## TODO:

- ~~[+] 实现功能：在文件路径两边增加双引号以提升兼容性~~
- [ ] 增加配置：配置文件的存储路径和文件名
- [ ] 增加配置：程序开始时隐藏控制台黑框

也许有可能的想法：

- [ ] 增加更多匹配规则，如正则表达式
- [ ] 更丰富的配置项
- [ ] 更复杂的传入参数和相对的行为

## 更新日志

### 2023-10-20

- 拆分代码到多个文件
- 完善程序逻辑，将命令拆分后直接传递
- 改为将配置文件存储在可执行文件同目录下

## English

A small utility developed in rust to customize the command line to run when opening files with different extensions.

The program accepts a file path as an argument, reads an open command from a configuration file with the corresponding extension, and then runs it.

The default configuration file:

```toml

[suffix.ps1]
comment = "Run Powershell Script"
command = "powershell -File {}"

[suffix.bat]
comment = "Edit CMD Script"
command = "notepad {}"

```

Configuration changes and additions need to be made, and you need to modify the configuration file yourself by adding or modifying table entries as follows:

```toml
[suffix.<custom suffix>]
comment = "Custom comment"
command = "Custom command"
```

The program will automatically replace `{}` in the command line with the file path when opened.
Commands will be passed directly to the application separated by spaces.


## TODO.

- ~~[+] Implementation: add double quotes to both sides of the file path to improve compatibility~~
- [ ] Add configuration: storage path and filename of the configuration file
- [ ] Add config: hide console black box when program starts

Possible ideas:

- [ ] Add more matching rules, such as regular expressions
- [ ] Richer configuration items
- [ ] more complex incoming parameters and relative behavior

## Update log

### 2023-10-20

- Split code to multiple files
- Improve program logic by splitting commands and passing them directly
- Changed to store configuration files in the same directory as the executable file

Translated with www.DeepL.com/Translator (free version)