# mudssky_utils

一个功能丰富的 Rust 实用工具库，提供字符串处理、数组操作、对象处理、数学计算、字节转换、异步工具和函数式编程等常用功能。

## 特性

### 🔤 字符串处理 (String)
- 大小写转换：`camel_case`, `snake_case`, `pascal_case`, `dash_case`
- 字符串操作：`capitalize`, `trim`, `remove_prefix`
- 模糊匹配：`fuzzy_match`
- 模板解析：`parse_template`
- UUID 生成：`generate_uuid`
- Base62 编码：`generate_base62_code`
- 路径合并：`generate_merge_paths`

### 📊 数组操作 (Array)
- 数组处理：`chunk`, `flatten`, `unique`, `intersection`
- 数组查询：`difference`, `union`, `compact`
- 范围生成：`range`
- 数组转换：`zip`, `unzip`

### 🗂️ 对象处理 (Object)
- 对象操作：`pick`, `omit`, `merge`, `invert`
- 键值转换：`map_keys`, `map_values`
- 条件过滤：`pick_by`, `omit_by`
- JSON 处理：`safe_json_stringify`, `remove_non_serializable_props`

### 🧮 数学计算 (Math)
- 随机数生成：`random_int`, `random_int_max`, `random_range`
- 数组随机：`get_random_item_from_array`
- 数值处理：`clamp`, `sum`, `average`

### 💾 字节转换 (Bytes)
- 字节单位转换：支持 B, KB, MB, GB, TB, PB
- 智能解析：`parse_bytes`
- 格式化输出：`format_bytes`
- 单例模式：全局字节转换器

### ⚡ 异步工具 (Async)
- 异步延迟：`sleep_async`
- 超时控制：`timeout`
- 并发处理：`join_all`

### 🔧 函数式工具 (Function)
- 防抖动：`Debouncer` - 延迟执行，避免频繁调用
- 节流：`Throttler` - 限制执行频率
- 轮询：`Poller` - 定时执行任务
- 重试机制：`with_retry` - 自动重试失败的操作

## 安装

在你的 `Cargo.toml` 中添加：

```toml
[dependencies]
mudssky_utils = "0.1.0"
tokio = { version = "1.46", features = ["full"] }  # 如果使用异步功能
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
let formatted = format_bytes(1024, &FormatOptions::default()); // "1 KB"
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

- ✅ **类型安全**：充分利用 Rust 的类型系统
- ✅ **错误处理**：使用 `Result` 类型进行优雅的错误处理
- ✅ **异步支持**：支持 async/await 模式
- ✅ **零拷贝**：尽可能避免不必要的内存分配
- ✅ **文档完整**：每个函数都有详细的文档和示例
- ✅ **测试覆盖**：67+ 个测试用例确保代码质量
- ✅ **Clippy 兼容**：通过所有 Clippy 检查

## 许可证

MIT License

## 作者

mudssky