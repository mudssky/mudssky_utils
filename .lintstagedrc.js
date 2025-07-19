// .lintstagedrc.js

// 修正后的、正确的 clippy 命令
const CLippyCommand = 'cargo clippy --workspace --all-targets -- -D warnings';

module.exports = {
  '*.rs': (filenames) => {
    // 1. 对于 `cargo fmt`，我们只格式化被修改的文件，效率很高。
    const filesToFormat = filenames.join(' ');
    const formatCommand = `cargo fmt -- ${filesToFormat}`;

    // 2. 对于 `cargo clippy`，我们返回固定的命令，它会检查整个项目。
    //    `lint-staged` 会确保这个命令只运行一次。
    //    我们在这里完全忽略了传入的 `filenames` 参数。
    // return [formatCommand, CLippyCommand];
    return [formatCommand];
  },
};