// lib.rs — was / unwas 共享代码
// Alias 和 ProfileManager 两个核心类型，供两个二进制入口共用

use std::io::{self, BufRead, Write};
use std::path::PathBuf;
use std::fs;

// ============================================================
// 常量
// ============================================================

/// $PROFILE 中标记区域起始行
pub const MANAGED_START: &str = "# >>> was managed <<<";
/// $PROFILE 中标记区域结束行
pub const MANAGED_END: &str = "# <<< was managed >>>";

// ============================================================
// Alias — 单个别名对象
// ============================================================

/// 表示一个别名，包含名称和对应的命令
pub struct Alias {
    pub name: String,
    pub command: String,
}

impl Alias {
    /// 从一行函数定义中解析别名
    /// 格式：`function global:<name> { & <command> $args }`
    pub fn parse_line(line: &str) -> Option<Alias> {
        let line = line.trim();
        let prefix = "function global:";
        if !line.starts_with(prefix) {
            return None;
        }
        let body = &line[prefix.len()..];
        if let Some(split) = body.find(" { & ") {
            let name = &body[..split];
            let cmd_start = split + 5;
            let suffix = " $args }";
            if body.ends_with(suffix) {
                let cmd_end = cmd_start + (body.len() - suffix.len() - split - 5);
                let command = &body[cmd_start..cmd_end];
                if name.chars().all(|c| c.is_alphanumeric() || c == '_') {
                    return Some(Alias {
                        name: name.to_string(),
                        command: command.to_string(),
                    });
                }
            }
        }
        None
    }

    /// 格式化为 PowerShell 函数定义行
    pub fn format(&self) -> String {
        format!("function global:{} {{ & {} $args }}", self.name, self.command)
    }

    /// 格式化为 human-readable 显示
    pub fn display(&self) -> String {
        format!("{} = {}", self.name, self.command)
    }
}

// ============================================================
// ProfileManager — 管理 $PROFILE 文件中的别名标记区域
// ============================================================

/// 负责读写 $PROFILE 文件，在标记区域内维护别名列表
pub struct ProfileManager {
    pub path: PathBuf,
}

impl ProfileManager {
    /// 使用指定路径创建管理器
    pub fn new(path: PathBuf) -> ProfileManager {
        ProfileManager { path }
    }

    /// 获取默认的 PowerShell $PROFILE 路径（pwsh 7）
    pub fn default_path() -> PathBuf {
        let home = dirs::home_dir().expect("无法获取用户主目录");
        home.join("Documents").join("PowerShell").join("Microsoft.PowerShell_profile.ps1")
    }

    /// 读取 $PROFILE 文件全部内容
    pub fn read_raw(&self) -> io::Result<Vec<String>> {
        if !self.path.exists() {
            return Ok(Vec::new());
        }
        let file = fs::File::open(&self.path)?;
        let reader = io::BufReader::new(file);
        reader.lines().collect()
    }

    /// 从文件内容中解析标记区域内的别名
    pub fn parse_aliases(&self, lines: &[String]) -> Vec<Alias> {
        let mut in_section = false;
        let mut aliases = Vec::new();
        for line in lines {
            let trimmed = line.trim();
            if trimmed == MANAGED_START {
                in_section = true;
                continue;
            }
            if trimmed == MANAGED_END {
                break;
            }
            if in_section {
                if let Some(alias) = Alias::parse_line(line) {
                    aliases.push(alias);
                }
            }
        }
        aliases
    }

    /// 保存别名列表到 $PROFILE，标记区域外内容保持不变
    pub fn save(&self, aliases: &[Alias]) -> io::Result<()> {
        let original = self.read_raw()?;
        let mut output: Vec<String> = Vec::new();
        let mut in_section = false;
        let mut section_written = false;

        for line in &original {
            let trimmed = line.trim();
            if trimmed == MANAGED_START {
                in_section = true;
                output.push(MANAGED_START.to_string());
                for alias in aliases {
                    output.push(alias.format());
                }
                section_written = true;
                continue;
            }
            if trimmed == MANAGED_END {
                in_section = false;
                output.push(MANAGED_END.to_string());
                continue;
            }
            if !in_section {
                output.push(line.clone());
            }
        }

        if !section_written {
            if let Some(last) = output.last() {
                if !last.is_empty() {
                    output.push(String::new());
                }
            }
            output.push(MANAGED_START.to_string());
            for alias in aliases {
                output.push(alias.format());
            }
            output.push(MANAGED_END.to_string());
        }

        let mut file = fs::File::create(&self.path)?;
        for line in &output {
            writeln!(file, "{}", line)?;
        }
        Ok(())
    }
}
