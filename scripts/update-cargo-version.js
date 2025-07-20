// scripts/update-cargo-version.js

const fs = require('fs');
const path = require('path');
const TOML = require('@iarna/toml');

// semantic-release 会将下一个版本号作为环境变量传入
const newVersion = process.env.SEMANTIC_RELEASE_NEXT_RELEASE_VERSION;

if (!newVersion) {
    console.error('错误：未提供版本号 (SEMANTIC_RELEASE_NEXT_RELEASE_VERSION is not set).');
    process.exit(1);
}

// 定义 Cargo.toml 文件的路径
const cargoTomlPath = path.resolve(process.cwd(), 'Cargo.toml');

try {
    // 检查文件是否存在
    if (!fs.existsSync(cargoTomlPath)) {
        console.error(`错误：在路径 ${cargoTomlPath} 未找到 Cargo.toml 文件.`);
        process.exit(1);
    }

    console.log(`正在读取 Cargo.toml...`);
    // 同步读取文件内容
    const cargoTomlString = fs.readFileSync(cargoTomlPath, 'utf-8');

    // 使用 @iarna/toml 解析文件
    const cargoToml = TOML.parse(cargoTomlString);

    // 检查 [package] 和 version 字段是否存在
    if (!cargoToml.package || typeof cargoToml.package.version === 'undefined') {
        console.error('错误：Cargo.toml 文件缺少 [package] 部分或 version 字段.');
        process.exit(1);
    }

    console.log(`找到旧版本: ${cargoToml.package.version}`);
    console.log(`正在更新到新版本: ${newVersion}`);

    // 更新 JavaScript 对象中的版本号
    cargoToml.package.version = newVersion;

    // 将更新后的 JavaScript 对象转换回 TOML 字符串
    const newCargoTomlString = TOML.stringify(cargoToml);

    // 将新内容写回 Cargo.toml 文件
    fs.writeFileSync(cargoTomlPath, newCargoTomlString);

    console.log('Cargo.toml 文件已成功更新！');

} catch (error) {
    console.error('处理 Cargo.toml 时发生错误:', error);
    process.exit(1);
}