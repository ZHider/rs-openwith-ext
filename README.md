# rs-openwith-ext

一个使用 rust 开发的小工具，用于对于不同后缀名的文件自定义打开时运行的命令行。

程序接受文件路径作为参数，从配置文件中读取对应后缀名的打开命令，然后运行。

默认配置文件：

```toml

[store.bat]
name = "Edit CMD Script"
command = "notepad {}"

[store.ps1]
name = "Run Powershell Script"
command = "powershell -File {}"

```

需要更改和增加配置，需要自行修改配置文件，增加或修改表项如下：

```toml
[store.<ext>]
name = [Name]
command = <command line>
```

程序在打开时会自动将命令行中的 `{}` 替换成文件路径。


## TODO:

- [ ] 实现功能：在文件路径两边增加双引号以提升兼容性
- [ ] 增加配置：配置文件的存储路径和文件名
- [ ] 增加配置：程序开始时隐藏控制台黑框

也许有可能的想法：

- [ ] 增加更多匹配规则，如正则表达式
- [ ] 更丰富的配置项
- [ ] 更复杂的传入参数和相对的行为

## English

A small utility developed in rust to customize the command line to run when opening files with different extensions.

The program accepts a file path as an argument, reads an open command from a configuration file with the corresponding extension, and then runs it.

The default configuration file:

```toml

[store.bat]
name = "Edit CMD Script"
command = "notepad {}"

[store.ps1]
name = "Run Powershell Script"
command = "powershell -File {}"

```

Configuration changes and additions need to be made, you need to modify the configuration file yourself by adding or modifying table entries as follows:

```toml
[store.<ext>]
name = [Name]
command = <command line>
```

The program will automatically replace `{}` in the command line with the path to the file when it opens.


TODO:

- [ ] Function: add double quotes to file paths to improve compatibility.
- [ ] Add configuration: storage path and filename of configuration file.
- [ ] Add config: hide console black box when program starts

Possible ideas:
- [ ] Add more matching rules, such as regular expressions
- [ ] Richer configuration items
- [ ] more complex incoming parameters and relative behavior


Translated with www.DeepL.com/Translator (free version)