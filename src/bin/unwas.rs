// unwas — 别名删除工具
// 只负责：删除别名 + --help / --version

use std::process;
use was::ProfileManager;

// ============================================================
// 命令枚举
// ============================================================

enum UnwasCmd {
    Delete(String),
    Help,
    Version,
}

// ============================================================
// 参数解析
// ============================================================

fn parse_args() -> Result<UnwasCmd, String> {
    let raw: Vec<String> = std::env::args().collect();
    let mut positional: Vec<&str> = Vec::new();

    for arg in raw.iter().skip(1) {
        match arg.as_str() {
            "--help" | "-?" => return Ok(UnwasCmd::Help),
            "--version" | "-V" => return Ok(UnwasCmd::Version),
            _ if arg.starts_with('-') => return Err(format!("未知选项：{}", arg)),
            _ => positional.push(arg.as_str()),
        }
    }

    match positional.len() {
        0 => Err("用法：unwas <name>".to_string()),
        1 => {
            let arg = positional[0];
            if arg.contains('=') {
                Err("设置别名请使用 `was <name>=<command>`".to_string())
            } else {
                Ok(UnwasCmd::Delete(arg.to_string()))
            }
        }
        _ => Err("参数过多".to_string()),
    }
}

// ============================================================
// 帮助
// ============================================================

fn print_help() {
    eprintln!("用法：unwas [选项] <name>");
    eprintln!("");
    eprintln!("  unwas <name>  删除指定别名");
    eprintln!("  -?, --help    显示此帮助");
    eprintln!("  -V, --version 显示版本号");
}

// ============================================================
// 主入口
// ============================================================

fn main() {
    let cmd = match parse_args() {
        Ok(c) => c,
        Err(msg) => {
            eprintln!("错误：{}", msg);
            process::exit(1);
        }
    };

    match cmd {
        UnwasCmd::Help => {
            print_help();
            process::exit(0);
        }
        UnwasCmd::Version => {
            println!("unwas v{}", env!("CARGO_PKG_VERSION"));
            process::exit(0);
        }
        UnwasCmd::Delete(name) => {
            if !name.chars().all(|c| c.is_alphanumeric() || c == '_') {
                eprintln!("错误：别名名称只能包含字母、数字和下划线");
                process::exit(1);
            }

            let manager = ProfileManager::new(ProfileManager::default_path());
            let lines = match manager.read_raw() {
                Ok(l) => l,
                Err(e) => {
                    eprintln!("错误：无法读取 $PROFILE：{}", e);
                    process::exit(1);
                }
            };
            let mut aliases = manager.parse_aliases(&lines);
            let before = aliases.len();
            aliases.retain(|a| a.name != name);

            if aliases.len() == before {
                eprintln!("错误：别名 '{}' 未定义", name);
                process::exit(1);
            }

            if let Err(e) = manager.save(&aliases) {
                eprintln!("错误：写入 $PROFILE 失败：{}", e);
                process::exit(1);
            }
            println!("已删除别名：{}", name);
            println!("已保存到：{}", manager.path.display());
        }
    }
}
