// was — Windows Alias System（主入口）
// 只负责：列表、查询、设置别名 + --help / --version

use std::process;
use was::{Alias, ProfileManager};

// ============================================================
// 命令枚举
// ============================================================

enum WasCmd {
    List,
    Get(String),
    Set(String, String),
    Help,
    Version,
}

// ============================================================
// 参数解析
// ============================================================

/// 解析 was 的命令行参数
/// 一旦遇到 name=，后续所有参数（即使是 -xxx）都视为命令值
fn parse_args() -> Result<WasCmd, String> {
    let raw: Vec<String> = std::env::args().collect();
    let mut i = 1;
    let mut set_name: Option<String> = None;
    let mut command_parts: Vec<String> = Vec::new();

    while i < raw.len() {
        let arg = &raw[i];

        match &set_name {
            None => {
                // 尚未进入设置模式，正常解析
                match arg.as_str() {
                    "--help" | "-?" => return Ok(WasCmd::Help),
                    "--version" | "-V" => return Ok(WasCmd::Version),
                    _ if arg.starts_with('-') => return Err(format!("未知选项：{}", arg)),
                    _ => {
                        if let Some((n, v)) = arg.split_once('=') {
                            if n.is_empty() {
                                return Err("别名名称不能为空".to_string());
                            }
                            set_name = Some(n.to_string());
                            if !v.is_empty() {
                                command_parts.push(v.to_string());
                            }
                        } else {
                            // 没有 =，必须是查询模式，且不能有多余参数
                            if i + 1 < raw.len() {
                                return Err("参数过多".to_string());
                            }
                            return Ok(WasCmd::Get(arg.to_string()));
                        }
                    }
                }
            }
            Some(_) => {
                // 已进入设置模式，所有剩余参数都是命令值的一部分
                command_parts.push(arg.clone());
            }
        }
        i += 1;
    }

    match set_name {
        None => Ok(WasCmd::List),
        Some(name) => {
            if command_parts.is_empty() {
                return Err("删除别名请使用 `unwas <name>`".to_string());
            }
            Ok(WasCmd::Set(name, command_parts.join(" ")))
        }
    }
}

// ============================================================
// 帮助
// ============================================================

fn print_help() {
    eprintln!("用法：was [选项] [<name>] [<name>=<command>]");
    eprintln!("");
    eprintln!("  was                    列出所有别名");
    eprintln!("  was <name>            查看指定别名");
    eprintln!("  was <name>=<command>  创建或更新别名");
    eprintln!("  -?, --help            显示此帮助");
    eprintln!("  -V, --version         显示版本号");
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
        WasCmd::Help => {
            print_help();
            process::exit(0);
        }
        WasCmd::Version => {
            println!("was v{}", env!("CARGO_PKG_VERSION"));
            process::exit(0);
        }
        _ => {}
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

    match cmd {
        WasCmd::List => {
            if aliases.is_empty() {
                println!("（没有已定义的别名）");
            } else {
                for alias in &aliases {
                    println!("{}", alias.display());
                }
            }
        }
        WasCmd::Get(name) => {
            if !name.chars().all(|c| c.is_alphanumeric() || c == '_') {
                eprintln!("错误：别名名称只能包含字母、数字和下划线");
                process::exit(1);
            }
            match aliases.iter().find(|a| a.name == name) {
                Some(alias) => println!("{}", alias.display()),
                None => {
                    eprintln!("错误：别名 '{}' 未定义", name);
                    process::exit(1);
                }
            }
        }
        WasCmd::Set(name, command) => {
            if !name.chars().all(|c| c.is_alphanumeric() || c == '_') {
                eprintln!("错误：别名名称只能包含字母、数字和下划线");
                process::exit(1);
            }
            if let Some(existing) = aliases.iter_mut().find(|a| a.name == name) {
                existing.command = command.clone();
            } else {
                aliases.push(Alias {
                    name: name.clone(),
                    command: command.clone(),
                });
            }
            if let Err(e) = manager.save(&aliases) {
                eprintln!("错误：写入 $PROFILE 失败：{}", e);
                process::exit(1);
            }
            println!("已设置别名：{} = {}", name, command);
            println!("已保存到：{}", manager.path.display());
        }
        _ => unreachable!(),
    }
}
