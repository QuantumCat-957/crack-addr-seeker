# 地址生成器

## 介绍

这个程序是一个地址生成器工具，主要用于生成末尾匹配长度相同字符的地址，并将生成的地址记录到文件中。支持生成以太坊（Ethereum）和波场（TRON）地址，并且可以通过配置参数来自定义生成行为。

## 功能

- 生成末尾匹配长度相同字符的以太坊和波场地址
- 配置文件大小限制和日志轮换间隔
- 支持多种助记词语言选择和自定义 BIP39 密码
- 记录生成的地址到文件中

## 安装

1. 克隆仓库：
   ```bash
   git clone <仓库地址>

2. 进入项目目录并构建：
   ```bash
   cd <项目目录>
   cargo build --release
   ```

## 使用

你可以通过命令行参数来运行这个工具。下面是常用的命令行参数及其说明：

### 命令行参数

- `--phrase <PHRASE>`: 助记词，使用空格分隔的词语，默认为 `fan swamp loop mesh enact tennis priority artefact canal hour skull joy`。
- `--password <PASSWORD>`: BIP39 密码，默认为空。
- `--max-file-size <SIZE>`: 最大文件大小，单位为字节。默认为 `104857600`（100 MB）。
- `--rotation-interval-secs <SECS>`: 日志轮换间隔，单位为秒。默认为 `3600`（1 小时）。
- `--language <LANGUAGE>`: 助记词语言，支持的语言包括：`english`、`chinese-simplified`、`chinese-traditional`、`czech`、`french`、`italian`、`japanese`、`korean`、`portuguese`、`spanish`。默认为 `english`。
- `--eth-match-length <LENGTH>`: 地址末尾匹配的字符数，默认为 `6`。
- `--tron-match-length <LENGTH>`: 地址末尾匹配的字符数，默认为 `4`。
- `--reverse-index`: 指定是否反向递增派生路径的索引值。默认情况下，索引值是正向递增的。如果设置此选项为 `true`，则索引值将从负数开始反向递增。

### 示例

反向递增派生路径索引值，生成末尾匹配长度为 6 的以太坊和波场地址，使用自定义助记词和密码：
```bash
target/release/crack-addr-seeker --phrase 'fan swamp loop mesh enact tennis priority artefact canal hour skull joy' --password '123' --max-file-size 104857600 --rotation-interval-secs 3600 --language english --eth-match-length 6 --tron-match-length 4 --reverse-index
```

## 文件结构

- `src/`: 源代码目录
  - `address/`: 地址生成相关的模块
  - `config/`: 配置相关的模块
  - `constant/`: 常量定义
  - `handle/`: 地址生成处理模块
  - `language/`: 语言处理模块
  - `timer/`: 计时器模块
  - `write/`: 地址记录写入模块
  - `xpriv/`: 助记词处理模块
