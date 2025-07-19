# 项目规范

## 项目简介

`mudssky_utils` 是一个服务于个人项目、高质量、轻量级且经过充分测试的 Rust 通用函数集合。
核心原则:
实用性 (Practicality): 只包含在实际项目中被证明有用的函数。
可靠性 (Reliability): 每个函数都必须有文档和测试。
最小依赖 (Minimalism): 尽可能只依赖 Rust 标准库（std），谨慎引入第三方依赖。

## 代码风格规范

### Rust Edition

最新稳定版 (Stable) 的 Rust

## 编码规范与风格

1. **代码格式化**:
   - 所有代码在提交前**必须**使用 cargo fmt 进行格式化。CI 环境会进行检查。
2. **代码质量检查 (Linting)**:
   - 所有代码在提交前**必须**通过 cargo clippy 的检查。
   - 应当以解决所有 clippy 警告（warnings）为目标，推荐使用 `cargo clippy -- -D warnings` 来将警告视为错误。
3. **命名约定**:
   - 遵循 [Rust API Guidelines](https://www.google.com/url?sa=E&q=https%3A%2F%2Frust-lang.github.io%2Fapi-guidelines%2Fintroduction.html) 的命名约定。
   - **类型 (Structs, Enums, Traits)**: PascalCase，例如 PathCleaner。
   - **函数、变量、模块名**: snake_case，例如 fn trim_lines()。
4. **API 设计**:
   - **公开性**: 只有明确希望被外部使用的函数、结构体等才应标记为 pub。
   - **错误处理**: 可失败的函数**必须**返回 `Result<T, E>`，而不是 `panic!`。为特定模块定义清晰的错误类型（Error Enum）,使用thiserror库定义错误类型
   - **安全性**: 除非绝对必要且有充分理由，否则**严禁**使用 unsafe 代码。任何 unsafe 代码块都必须有详细的注释，解释其为何是安全的。

## 提交规范

### Commit 规范

项目严格遵循 **Angular 提交规范**，这有助于生成清晰的提交历史和自动化的版本日志。

提交信息格式:

```
{type}({scope}): {emoji}{subject}
```

### 版本管理与发布

项目使用自动化工具进行版本管理和发布。

- **发布分支**: `main`
- **推荐工具**: `cargo-release`
  - 该工具可以自动完成版本号更新、创建 Git Tag、推送和发布到 `crates.io` 的流程。
  - 运行 `cargo release <level>` (其中 `level` 是 `patch`, `minor`, `major`) 即可触发。
- **CI/CD 发布**:
  - 在 GitHub Actions 中，我们使用 **Trusted Publishing (OIDC)** 来安全地发布到 `crates.io`，避免在 Secrets 中存储长期的 API 令牌。

## 开发指南

### 项目结构

Rust 的模块系统有其独特的组织方式，我们遵循官方推荐的结构：

```
.
├── .github/          # CI/CD 工作流 (例如：GitHub Actions)
├── src/              # 源代码目录
│   ├── lib.rs        # 库的根文件和主入口，负责声明所有公共模块
│   ├── array.rs      # 数组/Vec 相关工具模块
│   ├── string.rs     # 字符串相关工具模块
│   ├── fs.rs         # 文件系统相关工具模块
│   ├── types.rs      # 公用的类型定义 (如果需要跨模块共享)
│   └── ...           # 其他功能模块
├── tests/            # 集成测试目录
│   ├── array_tests.rs
│   └── string_tests.rs
├── examples/         # 使用示例目录
│   └── simple_usage.rs
├── Cargo.toml        # 项目清单文件 (元数据、依赖等)
├── README.md         # 项目说明
└── .rustfmt.toml     # 格式化配置
```

### 开发流程

1. 开发方法，编写方法文档，包含文档测试
2. 在tests/，目录下编写集成测试，运行集成测试通过
3. 执行`cargo clippy -- -D warnings`，修复遇到的问题

### 测试

1. **集成测试 (Integration Tests)**
    - 位于根目录下的 `tests/` 目录中。每个 `.rs` 文件都是一个独立的测试 Crate。
    - 用于测试库的公共 API，模拟真实用户的使用方式。

2. **文档测试 (Doc Tests)**
    - 直接写在文档注释（`///`）中的代码示例会自动作为测试运行。这是保证文档与代码同步的最佳方式。

**运行所有测试:**

```bash
cargo test
```

## 文档

- **API 文档**: 项目的公共 API 文档通过代码中的**文档注释 (`///`)** 自动生成。
- **生成文档**: 运行 `cargo doc --open` 会在本地构建 HTML 文档并在浏览器中打开。
- **项目指南**: 对于更详细的指南和说明（类似 VitePress 的效果），我们可以使用 `mdbook` 来创建一本单独的书籍。
