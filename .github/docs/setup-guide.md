# GitHub Actions 配置指南

本指南将帮助你为 Rust 项目配置和使用 GitHub Actions 工作流程。

## 前置要求

### 1. 仓库设置

确保你的 GitHub 仓库已经：
- 启用了 Actions 功能
- 配置了适当的分支保护规则
- 设置了必要的密钥和环境变量

### 2. 项目结构

你的 Rust 项目应该具有以下基本结构：

```
your-project/
├── .github/
│   └── workflows/
│       ├── ci.yml
│       ├── docs.yml
│       └── release.yml
├── src/
├── tests/
├── Cargo.toml
├── Cargo.lock
└── README.md
```

## 配置步骤

### 步骤 1: 复制工作流程文件

将以下三个工作流程文件复制到你的项目的 `.github/workflows/` 目录：

1. `ci.yml` - 持续集成
2. `docs.yml` - 文档生成和部署
3. `release.yml` - 自动发布

### 步骤 2: 配置项目特定设置

#### 修改 `ci.yml`

1. **更新 MSRV (最低支持 Rust 版本)**：
   ```yaml
   - name: Install Rust
     uses: dtolnay/rust-toolchain@1.70.0  # 修改为你的 MSRV
   ```

2. **调整测试矩阵**（可选）：
   ```yaml
   strategy:
     matrix:
       rust:
         - stable
         - beta
         # - nightly  # 如果不需要 nightly 测试可以注释掉
   ```

#### 修改 `docs.yml`

1. **更新自定义域名**（如果有）：
   ```yaml
   - name: Deploy to GitHub Pages
     if: github.ref == 'refs/heads/main'
     uses: peaceiris/actions-gh-pages@v3
     with:
       github_token: ${{ secrets.GITHUB_TOKEN }}
       publish_dir: ./target/doc
       cname: your-project.docs.rs  # 修改为你的域名或删除此行
   ```

2. **更新重定向页面**：
   ```yaml
   - name: Generate documentation
     run: |
       cargo doc --no-deps --all-features
       echo '<meta http-equiv="refresh" content="0; url=your_crate_name">' > target/doc/index.html
   ```

#### 修改 `release.yml`

1. **配置 semantic-release**（如果使用）：
   确保项目根目录有 `.releaserc.json` 文件：
   ```json
   {
     "branches": ["main"],
     "plugins": [
       "@semantic-release/commit-analyzer",
       "@semantic-release/release-notes-generator",
       "@semantic-release/changelog",
       "@semantic-release/github"
     ]
   }
   ```

### 步骤 3: 配置 GitHub 仓库设置

#### 启用 GitHub Pages

1. 进入仓库设置页面
2. 滚动到 "Pages" 部分
3. 选择 "Source" 为 "GitHub Actions"
4. 保存设置

#### 配置分支保护规则

1. 进入 "Settings" > "Branches"
2. 为 `main` 分支添加保护规则：
   - ✅ Require status checks to pass before merging
   - ✅ Require branches to be up to date before merging
   - 选择必需的状态检查：
     - `Test (stable)`
     - `Coverage`
     - `Security audit`
     - `Check Documentation`

### 步骤 4: 配置密钥和环境变量

#### 必需的密钥

在仓库设置的 "Secrets and variables" > "Actions" 中添加：

1. **CARGO_REGISTRY_TOKEN**（用于发布到 crates.io）：
   - 登录 [crates.io](https://crates.io/)
   - 进入 "Account Settings" > "API Tokens"
   - 创建新的 API token
   - 将 token 添加到 GitHub Secrets

#### 可选的密钥

1. **CODECOV_TOKEN**（如果使用私有仓库）：
   - 在 [Codecov](https://codecov.io/) 注册并添加仓库
   - 获取 upload token
   - 添加到 GitHub Secrets

### 步骤 5: 配置项目文件

#### 更新 `Cargo.toml`

确保包含必要的元数据：

```toml
[package]
name = "your-crate-name"
version = "0.1.0"
edition = "2021"
rust-version = "1.70.0"  # 与 CI 中的 MSRV 保持一致
authors = ["Your Name <your.email@example.com>"]
license = "MIT OR Apache-2.0"
description = "A brief description of your crate"
homepage = "https://github.com/yourusername/your-repo"
repository = "https://github.com/yourusername/your-repo"
documentation = "https://docs.rs/your-crate-name"
readme = "README.md"
keywords = ["rust", "utility", "library"]
categories = ["development-tools"]

[dependencies]
# your dependencies

[dev-dependencies]
# your dev dependencies
```

#### 配置 `rustfmt.toml`

创建 `rustfmt.toml` 文件来统一代码格式：

```toml
# 基本配置
max_width = 100
hard_tabs = false
tab_spaces = 4

# 导入配置
reorder_imports = true
reorder_modules = true
reorder_impl_items = true

# 格式化配置
format_code_in_doc_comments = true
format_strings = false
format_macro_matchers = true

# 换行配置
newline_style = "Unix"
use_small_heuristics = "Default"
```

#### 配置 `clippy.toml`

创建 `clippy.toml` 文件来配置 Clippy 规则：

```toml
# 认知复杂度阈值
cognitive-complexity-threshold = 30

# 类型复杂度阈值
type-complexity-threshold = 250

# 函数行数阈值
too-many-lines-threshold = 100

# 避免的名称
avoid-breaking-exported-api = false
```

### 步骤 6: 配置提交规范

#### 安装 commitizen（可选）

```bash
npm install -g commitizen cz-conventional-changelog
echo '{"path": "cz-conventional-changelog"}' > ~/.czrc
```

#### 提交信息格式

使用 Angular 提交规范：

```
<type>(<scope>): <subject>

<body>

<footer>
```

**类型 (type)：**
- `feat`: 新功能
- `fix`: 修复 bug
- `docs`: 文档更新
- `style`: 代码格式化
- `refactor`: 重构
- `test`: 测试相关
- `chore`: 构建过程或辅助工具的变动

**示例：**
```
feat(string): add fuzzy matching function

Implement fuzzy string matching using Levenshtein distance algorithm.
This allows for approximate string matching with configurable threshold.

Closes #123
```

## 验证配置

### 本地验证

在推送到 GitHub 之前，本地运行以下命令验证：

```bash
# 格式检查
cargo fmt --all -- --check

# Clippy 检查
cargo clippy --all-targets --all-features -- -D warnings

# 运行测试
cargo test --all-features

# 生成文档
cargo doc --no-deps --all-features

# 文档测试
cargo test --doc
```

### GitHub Actions 验证

1. 推送代码到 GitHub
2. 检查 Actions 页面的工作流程状态
3. 确保所有检查都通过
4. 验证文档是否正确部署到 GitHub Pages

## 故障排除

### 常见问题及解决方案

#### 1. Actions 权限问题

**问题**：工作流程因权限不足失败

**解决方案**：
- 检查仓库设置中的 Actions 权限
- 确保 `GITHUB_TOKEN` 有足够的权限
- 对于组织仓库，检查组织级别的 Actions 设置

#### 2. 缓存问题

**问题**：构建时间过长或缓存相关错误

**解决方案**：
- 手动清除 Actions 缓存
- 检查缓存键是否正确
- 验证 `Cargo.lock` 文件是否提交

#### 3. 文档部署失败

**问题**：GitHub Pages 部署失败

**解决方案**：
- 检查 Pages 设置是否正确
- 验证分支和目录配置
- 检查自定义域名配置

#### 4. 发布失败

**问题**：crates.io 发布失败

**解决方案**：
- 验证 `CARGO_REGISTRY_TOKEN` 是否正确
- 检查 `Cargo.toml` 中的元数据
- 确保版本号符合语义化版本规范

### 调试技巧

1. **启用调试日志**：
   ```yaml
   - name: Debug step
     run: |
       echo "Debug information"
       env
     env:
       RUST_LOG: debug
   ```

2. **使用 act 本地测试**：
   ```bash
   # 安装 act
   curl https://raw.githubusercontent.com/nektos/act/master/install.sh | sudo bash
   
   # 本地运行工作流程
   act -j test
   ```

3. **分步骤测试**：
   - 注释掉部分步骤
   - 逐步启用以定位问题

## 高级配置

### 矩阵构建

为不同平台和 Rust 版本配置矩阵构建：

```yaml
strategy:
  matrix:
    os: [ubuntu-latest, windows-latest, macos-latest]
    rust: [stable, beta]
    include:
      - os: ubuntu-latest
        rust: nightly
        experimental: true
  fail-fast: false
runs-on: ${{ matrix.os }}
```

### 条件执行

根据条件执行特定步骤：

```yaml
- name: Deploy to crates.io
  if: github.event_name == 'push' && startsWith(github.ref, 'refs/tags/v')
  run: cargo publish --token ${{ secrets.CARGO_REGISTRY_TOKEN }}
```

### 自定义 Actions

创建可重用的自定义 Actions：

```yaml
# .github/actions/setup-rust/action.yml
name: 'Setup Rust'
description: 'Setup Rust toolchain with caching'
inputs:
  toolchain:
    description: 'Rust toolchain to install'
    required: false
    default: 'stable'
runs:
  using: 'composite'
  steps:
    - name: Install Rust
      uses: dtolnay/rust-toolchain@master
      with:
        toolchain: ${{ inputs.toolchain }}
        components: rustfmt, clippy
    # ... 缓存步骤
```

## 最佳实践总结

1. **保持工作流程简洁**：避免过于复杂的配置
2. **使用缓存**：合理使用缓存提高构建速度
3. **并行执行**：尽可能并行运行独立的作业
4. **失败快速**：配置快速失败策略
5. **安全第一**：妥善管理密钥和敏感信息
6. **文档同步**：保持工作流程文档的更新
7. **监控性能**：定期检查工作流程的执行时间
8. **版本固定**：使用特定版本的 Actions 而非 latest

通过遵循这个配置指南，你可以为 Rust 项目建立一个强大、可靠的 CI/CD 流水线。