易读性优先的刷题体验

# Rust 项目须知

- 在 `VSCode` 中，想要 `CodeLLDB`、 `rust-analyzer` 插件工作，需要 `Cargo` 配置文件，仅仅是 *.ts 文件是不行的
  - `Cargo` 相当于前端开发的 `npm` + `Vite`，有包管理器、打包器等作用
  - 算法题放在 `/src/bin` 文件夹下，可以自动识别为可执行文件，不用在 `Cargo.toml` 中显性的引入了