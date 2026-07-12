# Changelog

## [2026.07.11] — v0.1.0
- 项目初始化，创建 Rust CLI 工具 `was`
- 实现 `was` / `unwas` 命令，管理 PowerShell $PROFILE 别名
- 创建 AGENTS.md 开发规范文档
- **变更详情**：[Taolun → 2026-07-11 实现](#2026-07-11--实现) | [项目进度](#已完成)

## [2026.07.11] — v0.1.1
- **重构**：was 和 unwas 彻底分离为独立二进制，不再共享 argv[0] 分发
- 提取 `src/lib.rs`，Alias 和 ProfileManager 移至共享库
- `was` 精简为只支持：列表、查询、设置 + `--help`/`-?`/`--version`/`-V`
- `unwas` 独立入口 `src/bin/unwas.rs`，只支持：删除 + `--help`/`-?`/`--version`/`-V`
- 移除 `clap` 依赖，改为手动解析参数
- 修复 `was name=cmd -flag` 时 `-flag` 被误判为未知选项的 bug
- **变更详情**：[Taolun → 2026-07-11 重构](#2026-07-11--重构独立-unwas-二进制) | [项目进度](#已完成-1)

## [2026.07.12] — v0.1.2
- **CI/CD**：添加 GitHub Actions（build + release），仿照 `woman` 项目重构
- 发布时自动构建 `was.exe` + `unwas.exe` 并上传到 GitHub Release
- **变更详情**：[Taolun → 2026-07-12 发布](#2026-07-12--发布) | [项目进度](#已完成-1)


# Taolun

## 2026-07-11 — 项目规划
### 讨论摘要
- 决定用 Rust 开发一个类似 Linux `alias` 的命令行工具
- 工具命名为 `was`（Windows Alias System），删除命令叫 `unwas`
- 只支持 PowerShell $PROFILE，不支持 CMD doskey
- 别名存储在 `$PROFILE` 标记区 `# >>> was managed <<<`
- 发布到 `C:\Program Files\coreutils\bin\`

### 涉及文件
- `Cargo.toml` — 项目配置
- `src/main.rs` — 主逻辑
- `AGENTS.md` — 开发规范


## 2026-07-11 — 实现
### 讨论摘要
- 工具最终命名为 `was`（Windows Alias System），删除命令为 `unwas`
- Rust 实现，依赖 `clap`（虽然后期手动解析了）和 `dirs`
- 双 bin 配置（Cargo.toml 中 `[[bin]]` 指定同一源文件），通过 `argv[0]` 分发 `unwas` 模式
- $PROFILE 标记区格式：`# >>> was managed <<<` / `# <<< was managed >>>`
- 编译后安装到 `C:\Program Files\coreutils\bin\`
- 测试验证：`was` 列出空列表 → `was ll=ls.exe -la` 设置 → `was ll` 查看 → `unwas ll` 删除 → 全部通过

### 涉及文件
- `Cargo.toml` — 双 bin 配置
- `src/main.rs` — 完整实现（~260 行）
- `AGENTS.md` — 已更新进度和本记录

### 相关变更
- [Changelog → 2026.07.11](#20260711--v010) | [项目进度 → 已完成](#已完成)


## 2026-07-11 — 重构：独立 unwas 二进制
### 讨论摘要
- 用户指出 was 和 unwas 职责应当完全分离，不能共用 argv[0] 分发
- `was` 只支持：无参（列表）、`<name>`（查询）、`<name>=<cmd>`（设置）、`--help`/`-?`、`--version`/`-V`
- `unwas` 只支持：`<name>`（删除）、`--help`/`-?`、`--version`/`-V`
- 提取 `lib.rs`，Alias 和 ProfileManager 放共享库
- `was name=` 报错提示使用 `unwas`，`unwas name=cmd` 报错提示使用 `was`
- 帮助短格式用 `-?` 而非 `-h`

### 涉及文件
- `Cargo.toml` — 改为 `unwas` bin 指向 `src/bin/unwas.rs`
- `src/lib.rs` — 新建，抽取共享代码
- `src/main.rs` — 重写，仅 was 逻辑
- `src/bin/unwas.rs` — 新建，独立 unwas 入口

### 相关变更
- [Changelog → 2026.07.11](#20260711--v010) | [项目进度 → 已完成](#已完成)


## 2026-07-12 — 发布
### 讨论摘要
- 仿照 `woman` 项目添加 GitHub Actions CI/CD（build + release）
- build.yml：push/PR 到 main 时构建 release 并上传 artifacts
- release.yml：打 tag 时自动构建、重命名二进制、创建 GitHub Release
- 发布 `v0.1.2`，tag 格式：`YYYY.MM.DD.xxxx`

### 涉及文件
- `.github/workflows/build.yml` — 新建，CI 构建
- `.github/workflows/release.yml` — 新建，自动发布
- `AGENTS.md` — 更新 Changelog 和项目进度

### 相关变更
- [Changelog → 2026.07.12](#20260712--v012) | [项目进度 → 已完成](#已完成-1)


# Agents

## 规范
1. **三次重试原则**：同一个问题重复 3 次无法解决，强制停止，向用户详细汇报遇到的问题，等待用户解答。
2. **全中文**：整个对话流程全部使用中文，包括 AI 思考过程输出在终端中的内容。
3. **详细注释**：代码必须有详细的中文注释。
4. **版本格式**：`YYYY.MM.DD.xxxx`，其中 `xxxx` 为作为 git tag 前的 commit ID 前 4 位，便于溯源。
5. **测试拆分**：测试文件按功能模块拆分成多个文件，禁止在一个文件里写全部测试。
6. **面向对象**：采用 OOP 方式开发，保持功能模块单一，高内聚低耦合。

## 项目进度

### 计划中
- 支持 `--profile` 参数指定自定义 $PROFILE 路径
- 支持导入/导出别名列表（JSON 格式）

### 代办
- （无）

### 已完成
- [x] 创建 Rust 项目结构（Cargo.toml, src, lib）— [Taolun → 2026-07-11 项目规划](#2026-07-11--项目规划)
- [x] 实现 $PROFILE 文件读写与标记区管理
- [x] 实现 `was` 列表 / 查询 / 设置逻辑
- [x] 实现 `unwas` 独立二进制（删除逻辑）
- [x] `was name= -flag` 正确将 `-flag` 视为命令值
- [x] 编译发布到 `C:\Program Files\coreutils\bin\` — [Taolun → 2026-07-11 实现](#2026-07-11--实现) | [Changelog → 2026.07.11](#20260711--v010)
- [x] **v0.1.1 重构**：was/unwas 分离为独立二进制，提取 lib.rs — [Taolun → 2026-07-11 重构](#2026-07-11--重构独立-unwas-二进制) | [Changelog → 2026.07.11](#20260711--v011)
- [x] **v0.1.2 发布**：添加 GitHub Actions CI/CD，自动构建发布 — [Taolun → 2026-07-12 发布](#2026-07-12--发布) | [Changelog → 2026.07.12](#20260712--v012)

## 开发流程
1. **先记录后编码**：每次改动前，先在 `Taolun` 章节保存讨论记录，再开始修改文件。
2. **使用 bash 命令**：Windows 已内置 coreutils，优先使用 `grep` `ls` `sed` `find` 等命令，避免使用 PowerShell cmdlet。
3. **完成后更新**：开发完成后，同步更新「项目进度」和「Changelog」。Changelog 条目与 Taolun 记录、项目进度通过 **外链** 关联，方便溯源。


# 认知修正

## 2026-07-11 — was 与 unwas 必须完全独立
- **踩坑**：最初用单个二进制 + argv[0] 分发实现 was/unwas，被用户纠正
- **纠正**：用户要求每个工具只做一件事，was 不包含任何删除逻辑，unwas 不包含任何设置逻辑
- **教训**：不要为了"代码复用"把职责不同的命令塞进同一个二进制。独立的 `[[bin]]` + 共享 `lib.rs` 是正确的结构
- 帮助短格式用户指定用 `-?`，不是常见的 `-h`

---

> **CoreUtils 使用规范**：`grep` `ls` `sed` `find` 等命令用法详见 `~/.config/opencode/docs/coreutils.md`
