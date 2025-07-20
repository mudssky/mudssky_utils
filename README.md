# mudssky_utils

[![Crates.io](https://img.shields.io/crates/v/mudssky_utils.svg)](https://crates.io/crates/mudssky_utils)
[![Documentation](https://docs.rs/mudssky_utils/badge.svg)](https://docs.rs/mudssky_utils)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.85%2B-blue.svg)](https://www.rust-lang.org)

一个功能丰富的 Rust 实用工具库，提供字符串处理、数组操作、对象处理、数学计算、字节转换、异步工具、函数式编程、环境变量处理、日志记录和正则表达式等常用功能。

## 特性

### 🔤 字符串处理 (String)
- **大小写转换**：`camel_case`, `snake_case`, `pascal_case`, `dash_case`
- **字符串操作**：`capitalize`, `trim`, `remove_prefix`, `get_file_ext`
- **模糊匹配**：`fuzzy_match` - 字符串相似度计算
- **模板解析**：`parse_template` - 支持自定义正则模式的模板替换
- **随机生成**：`generate_uuid`, `generate_base62_code`, `generate_random_string`
- **路径处理**：`generate_merge_paths` - 智能路径合并

### 📊 数组操作 (Array)
- **数组分割**：`chunk` - 将数组分割成指定大小的块
- **数组扁平化**：`flatten` - 多维数组扁平化处理
- **集合操作**：`unique`, `intersection`, `difference`, `union`
- **数组清理**：`compact` - 移除空值和无效元素
- **范围生成**：`range` - 灵活的数字范围生成器
- **数组转换**：`zip`, `unzip` - 数组组合与分离
- **聚合函数**：`sum`, `group_by` - 数组统计和分组

### 🗂️ 对象处理 (Object)
- **属性选择**：`pick`, `omit` - 选择或排除指定属性
- **对象合并**：`merge` - 深度合并多个对象
- **键值操作**：`invert`, `map_keys`, `map_values` - 键值转换和映射
- **条件过滤**：`pick_by`, `omit_by` - 基于条件的属性过滤
- **JSON 处理**：`safe_json_stringify`, `remove_non_serializable_props`
- **对象验证**：类型安全的对象操作

### 🧮 数学计算 (Math)
- **随机数生成**：`random_int`, `random_int_max`, `random_range`
- **数组随机**：`get_random_item_from_array` - 从数组中随机选择元素
- **数值处理**：范围限制、统计计算等数学工具

### 💾 字节转换 (Bytes)
- **单位支持**：B, KB, MB, GB, TB, PB 完整字节单位
- **智能解析**：`parse_bytes` - 解析 "1.5 GB" 等字符串格式
- **格式化输出**：`format_bytes` - 可自定义格式的字节显示
- **配置选项**：支持小数位数、千位分隔符、单位分隔符等
- **单例模式**：全局字节转换器，性能优化

### ⚡ 异步工具 (Async)
- **异步延迟**：`sleep_async` - 非阻塞延迟执行
- **超时控制**：`timeout` - 为异步操作添加超时机制
- **并发处理**：`join_all` - 并发执行多个异步任务

### 🔧 函数式工具 (Function)
- **防抖动**：`Debouncer` - 延迟执行，避免频繁调用
- **节流控制**：`Throttler` - 限制函数执行频率
- **轮询机制**：`Poller` - 可配置的定时任务执行器
- **重试机制**：`with_retry` - 自动重试失败的操作，支持自定义策略
- **状态管理**：支持取消操作和状态查询
- **异步支持**：完全基于 async/await 的现代异步编程

### 🌍 环境变量 (Env)
- **环境检测**：`is_development`, `is_production`, `is_test` - 环境判断
- **调试模式**：`is_debug` - 调试模式检测
- **配置获取**：安全的环境变量读取和类型转换

### 📝 日志记录 (Logger)
- **多级日志**：TRACE, DEBUG, INFO, WARN, ERROR 五个级别
- **灵活格式**：支持 JSON 和文本格式化器
- **多输出目标**：控制台、文件等多种输出方式
- **配置化**：可配置日志级别、格式和输出目标
- **线程安全**：支持多线程环境下的安全日志记录

### 🔍 正则表达式 (Regex)
- **常用模式**：预定义的常用正则表达式模式
- **验证工具**：邮箱、URL、电话号码等格式验证
- **文本提取**：基于正则的文本内容提取

### ❌ 错误处理 (Error)
- **统一错误类型**：各模块的错误类型定义
- **错误链**：支持错误原因追踪
- **类型安全**：基于 `thiserror` 的类型安全错误处理

## 安装

在你的 `Cargo.toml` 中添加：

```toml
[dependencies]
mudssky_utils = "0.1.0"
tokio = { version = "1.46", features = ["full"] }  # 异步功能需要
```

## 使用示例

### 字符串处理

```rust
use mudssky_utils::string::*;

// 大小写转换
let camel = camel_case("hello_world");  // "helloWorld"
let snake = snake_case("HelloWorld");   // "hello_world"
let pascal = pascal_case("hello-world"); // "HelloWorld"

// 模糊匹配
let score = fuzzy_match("hello", "hllo"); // 0.8

// UUID 生成
let uuid = generate_uuid(); // "a1b2c3d4-e5f6-7890-abcd-ef1234567890"
```

### 数组操作

```rust
use mudssky_utils::array::*;

// 数组分块
let chunks = chunk(&[1, 2, 3, 4, 5], 2); // [[1, 2], [3, 4], [5]]

// 数组去重
let unique_items = unique(&[1, 2, 2, 3, 3, 3]); // [1, 2, 3]

// 数组交集
let common = intersection(&[1, 2, 3], &[2, 3, 4]); // [2, 3]
```

### 对象处理

```rust
use mudssky_utils::object::*;
use std::collections::HashMap;

let mut map = HashMap::new();
map.insert("name".to_string(), "Alice".to_string());
map.insert("age".to_string(), "30".to_string());
map.insert("city".to_string(), "NYC".to_string());

// 选择特定键
let picked = pick(&map, &["name", "age"]); // {"name": "Alice", "age": "30"}

// 排除特定键
let omitted = omit(&map, &["city"]); // {"name": "Alice", "age": "30"}
```

### 字节转换

```rust
use mudssky_utils::bytes::*;

// 解析字节字符串
let bytes = parse_bytes("1.5 GB").unwrap(); // 1610612736

// 格式化字节
let formatted = bytes(1024).unwrap(); // "1 KB"

// 自定义格式
let options = BytesOptions {
    unit: Some(ByteUnit::MB),
    decimal_places: 3,
    ..Default::default()
};
let custom = get_bytes_instance().convert_number(1048576, Some(options)).unwrap();
```

### 函数式工具

```rust
use mudssky_utils::function::*;
use tokio::time::Duration;

#[tokio::main]
async fn main() {
    // 防抖动
    let mut debouncer = Debouncer::new(Duration::from_millis(300));
    debouncer.execute(|| println!("执行！")).await;
    
    // 节流
    let mut throttler = Throttler::new(Duration::from_millis(100));
    throttler.execute(|| println!("节流执行！")).await;
    
    // 重试机制
    let result = with_retry(
        || async { Ok::<i32, Box<dyn std::error::Error + Send + Sync>>(42) },
        &RetryOptions::default()
    ).await;
}
```

### 环境变量和日志

```rust
use mudssky_utils::{env::*, logger::*};

// 环境检测
if is_development() {
    println!("开发环境");
}

if is_debug() {
    println!("调试模式");
}

// 日志记录
let logger = Logger::new(LoggerConfig::default());
logger.info("应用启动");
logger.error("发生错误");

// JSON 格式日志
let json_logger = Logger::new(LoggerConfig {
    formatter: Box::new(JsonFormatter::default()),
    ..Default::default()
});
json_logger.info("JSON 格式日志");
```

### 数学计算

```rust
use mudssky_utils::math::*;

// 生成随机数
let random = random_int(1, 100).unwrap(); // 1-99 之间的随机数

// 从数组中随机选择
let item = get_random_item_from_array(&[1, 2, 3, 4, 5]).unwrap();
```

## 测试

运行所有测试：

```bash
cargo test
```

运行特定模块测试：

```bash
cargo test --test string_tests
cargo test --test array_tests
cargo test --test object_tests
# ... 等等
```

## 代码质量

检查代码质量：

```bash
cargo clippy -- -D warnings
```

生成文档：

```bash
cargo doc --no-deps --open
```

## 特点

- ✅ **类型安全**：充分利用 Rust 的类型系统，编译时错误检查
- ✅ **错误处理**：使用 `Result` 类型进行优雅的错误处理
- ✅ **异步支持**：完整的 async/await 支持，适合现代异步编程
- ✅ **零拷贝**：尽可能避免不必要的内存分配和拷贝
- ✅ **文档完整**：每个函数都有详细的文档和使用示例
- ✅ **测试覆盖**：53+ 个测试用例确保代码质量和稳定性
- ✅ **Clippy 兼容**：通过所有 Clippy 检查，符合 Rust 最佳实践
- ✅ **模块化设计**：清晰的模块结构，按需引入
- ✅ **性能优化**：针对常用场景进行性能优化
- ✅ **跨平台**：支持 Windows、macOS、Linux 等主流平台

## 许可证

本项目采用 [MIT 许可证](LICENSE)。

## 贡献

欢迎提交 Issue 和 Pull Request！请确保：

1. 代码通过所有测试：`cargo test`
2. 代码通过 Clippy 检查：`cargo clippy -- -D warnings`
3. 代码格式化：`cargo fmt`
4. 添加适当的测试用例
5. 更新相关文档

## 作者

- **mudssky** - [GitHub](https://github.com/mudssky)

---

如果这个库对你有帮助，请给个 ⭐ Star！

## 许可证

MIT License

## 作者

mudssky