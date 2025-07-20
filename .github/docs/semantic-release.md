# Semantic Release 配置指南

本项目使用 [semantic-release](https://semantic-release.gitbook.io/) 进行自动化版本管理和发布。semantic-release 会根据提交信息自动确定版本号、生成变更日志并发布到 crates.io。

## 🎯 工作原理

semantic-release 通过分析提交信息来确定版本类型：

- **PATCH** (0.0.X)：修复 bug (`fix:` 类型的提交)
- **MINOR** (0.X.0)：新增功能 (`feat:` 类型的提交)
- **MAJOR** (X.0.0)：破坏性变更 (`BREAKING CHANGE` 或 `!` 标记)

## 📝 提交信息规范

### 基本格式

```
<type>(<scope>): <subject>

<body>

<footer>
```

### 提交类型 (type)

| 类型 | 描述 | 版本影响 |
|------|------|----------|
| `feat` | 新功能 | MINOR |
| `fix` | 修复 bug | PATCH |
| `docs` | 文档更新 | 无 |
| `style` | 代码格式化 | 无 |
| `refactor` | 重构代码 | 无 |
| `test` | 测试相关 | 无 |
| `chore` | 构建过程或辅助工具 | 无 |
| `perf` | 性能优化 | PATCH |
| `ci` | CI 配置变更 | 无 |

### 作用域 (scope)

可选的作用域，表示变更影响的模块：

- `string` - 字符串工具
- `array` - 数组工具
- `number` - 数字工具
- `date` - 日期工具
- `crypto` - 加密工具
- `fs` - 文件系统工具

### 破坏性变更

对于破坏性变更，有两种标记方式：

1. **在类型后添加 `!`**：
   ```
   feat!: remove deprecated string_utils module
   ```

2. **在 footer 中添加 `BREAKING CHANGE`**：
   ```
   feat(string): add new fuzzy matching API
   
   BREAKING CHANGE: The old fuzzy_match function has been removed.
   Use the new fuzzy_search function instead.
   ```

### 提交示例

#### 新功能
```
feat(string): add fuzzy string matching

Implement fuzzy string matching using Levenshtein distance algorithm.
This allows for approximate string matching with configurable threshold.

Closes #123
```

#### 修复 bug
```
fix(array): handle empty arrays in chunk function

The chunk function now properly handles empty input arrays
and returns an empty array instead of panicking.

Fixes #456
```

#### 破坏性变更
```
feat!: redesign crypto module API

Simplify the crypto module by removing deprecated functions
and consolidating similar functionality.

BREAKING CHANGE: The following functions have been removed:
- old_encrypt() - use encrypt() instead
- old_decrypt() - use decrypt() instead
```

#### 文档更新
```
docs: update README with new installation instructions

Add npm and cargo installation methods.
Update usage examples with latest API.
```

## ⚙️ 配置文件

项目的 semantic-release 配置位于 `.releaserc.json`：

```json
{
  "branches": ["main"],
  "plugins": [
    "@semantic-release/commit-analyzer",
    "@semantic-release/release-notes-generator",
    [
      "@semantic-release/changelog",
      {
        "changelogFile": "CHANGELOG.md"
      }
    ],
    [
      "@semantic-release-cargo/semantic-release-cargo",
      {
        "publish": true
      }
    ],
    [
      "@semantic-release/git",
      {
        "assets": ["CHANGELOG.md", "Cargo.toml"],
        "message": "chore(release): ${nextRelease.version} [skip ci]\n\n${nextRelease.notes}"
      }
    ],
    "@semantic-release/github"
  ]
}
```

### 插件说明

1. **@semantic-release/commit-analyzer**：分析提交信息确定版本类型
2. **@semantic-release/release-notes-generator**：生成发布说明
3. **@semantic-release/changelog**：生成和更新 CHANGELOG.md
4. **@semantic-release-cargo/semantic-release-cargo**：更新 Cargo.toml 版本并发布到 crates.io
5. **@semantic-release/git**：提交版本变更到 Git
6. **@semantic-release/github**：创建 GitHub Release

## 🚀 发布流程

### 自动发布

当代码推送到 `main` 分支时，GitHub Actions 会自动：

1. **分析提交**：检查自上次发布以来的所有提交
2. **确定版本**：根据提交类型确定新版本号
3. **更新文件**：
   - 更新 `Cargo.toml` 中的版本号
   - 生成/更新 `CHANGELOG.md`
4. **创建标签**：创建新的 Git 标签
5. **发布包**：发布到 crates.io
6. **创建 Release**：在 GitHub 创建 Release
7. **提交变更**：将版本变更提交回仓库

### 手动触发发布

如果需要手动触发发布（通常不需要）：

```bash
# 确保在 main 分支
git checkout main
git pull origin main

# 运行 semantic-release
npx semantic-release
```

## 📊 版本策略

### 版本号格式

遵循 [语义化版本](https://semver.org/lang/zh-CN/) 规范：`MAJOR.MINOR.PATCH`

- **MAJOR**：不兼容的 API 修改
- **MINOR**：向下兼容的功能性新增
- **PATCH**：向下兼容的问题修正

### 预发布版本

对于 beta 或 alpha 版本，可以使用预发布标识：

- `1.0.0-alpha.1`
- `1.0.0-beta.1`
- `1.0.0-rc.1`

## 🛠️ 开发工具

### Commitizen

推荐使用 [Commitizen](https://commitizen-tools.github.io/commitizen/) 来规范提交信息：

```bash
# 安装
npm install -g commitizen cz-conventional-changelog

# 配置
echo '{"path": "cz-conventional-changelog"}' > ~/.czrc

# 使用
git add .
cz
```

### VS Code 扩展

推荐安装以下 VS Code 扩展：

- **Conventional Commits**：提供提交信息模板
- **GitLens**：增强 Git 功能

## 🔍 故障排除

### 常见问题

#### 1. 没有触发发布

**原因**：提交信息不符合规范或没有触发版本变更的提交类型

**解决**：
- 检查提交信息格式
- 确保有 `feat:` 或 `fix:` 类型的提交
- 查看 GitHub Actions 日志

#### 2. 发布失败

**原因**：可能是 crates.io 令牌问题或网络问题

**解决**：
- 检查 `CARGO_REGISTRY_TOKEN` 是否正确配置
- 查看详细的错误日志
- 验证 Cargo.toml 元数据

#### 3. 版本号不正确

**原因**：提交信息分析错误或配置问题

**解决**：
- 检查提交历史
- 验证 `.releaserc.json` 配置
- 手动运行 semantic-release 进行调试

### 调试命令

```bash
# 预览下一个版本（不实际发布）
npx semantic-release --dry-run

# 查看详细日志
npx semantic-release --debug

# 检查配置
npx semantic-release --verify-conditions
```

## 📚 参考资源

- [Semantic Release 官方文档](https://semantic-release.gitbook.io/)
- [Angular 提交规范](https://github.com/angular/angular/blob/main/CONTRIBUTING.md#commit)
- [语义化版本规范](https://semver.org/lang/zh-CN/)
- [Conventional Commits](https://www.conventionalcommits.org/zh-hans/)
- [semantic-release-cargo 插件](https://github.com/semantic-release-cargo/semantic-release-cargo)

---

通过遵循这些规范和最佳实践，你可以确保项目的版本管理和发布过程完全自动化，提高开发效率并减少人为错误。