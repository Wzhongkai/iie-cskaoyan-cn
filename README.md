# 信工所考研信息站

面向中国科学院信息工程研究所考生的公益信息站。当前项目是动态全栈应用，前端使用 SvelteKit，后端使用 Rust/Axum，数据存储在 PostgreSQL。

线上地址：<https://iie.cskaoyan.cn/>

## 当前功能

- 首页展示最新年度的完整招生数据和近三年概览
- 按年度查看推免、统考、生源院校、分数段、科目和实验室数据
- 文章列表、文章详情、置顶和状态管理
- Markdown 稿件投稿，支持表格、LaTeX 和图片
- Markdown 与图片 ZIP 导入
- 后台审核投稿、维护文章和年度招生数据
- 图片上传及静态文件访问

## 技术栈

| 模块 | 技术 | 当前版本或要求 |
| --- | --- | --- |
| 前端框架 | Svelte / SvelteKit | Svelte 5.56.8，SvelteKit 2.70.2 |
| 前端构建 | Vite / TypeScript | Vite 7.3.6，TypeScript 5.9.3 |
| 图表 | ECharts | 6.1.0 |
| Markdown | marked / KaTeX / sanitize-html | 15.0.12 / 0.16.22 / 2.17.6 |
| 后端 | Rust / Axum / Tokio | Rust 1.97.1，Axum 0.8，Tokio 1 |
| 数据访问 | SQLx | 0.8 |
| 数据库 | PostgreSQL | 16+；本地 Docker 使用 17-alpine |
| 反向代理 | Nginx | 生产环境 1.24.0 |
| 生产系统 | Ubuntu | 24.04.4 LTS |

依赖的精确版本由 `frontend/package-lock.json` 和 `backend/Cargo.lock` 锁定。

## 开发环境

推荐使用以下环境：

- Node.js 22.12 或更高版本，当前已验证 22.23.2
- npm 10 或更高版本，当前已验证 10.9.8
- Rust stable，当前已验证 `rustc 1.97.1` 和 `cargo 1.97.1`
- Docker Desktop，当前已验证 Docker 29.6.2 和 Compose 5.3.1
- Git 2.x

Docker Desktop 仅用于本地 PostgreSQL。前端和后端直接在本机运行，不需要另外创建应用容器。

## 项目结构

```text
.
├─ frontend/                 SvelteKit 前端和后台管理界面
│  ├─ src/lib/              公共组件、类型、Markdown 和服务端 API 客户端
│  ├─ src/routes/           页面与服务端数据加载
│  └─ static/               公共静态文件
├─ backend/                  Rust API
│  ├─ migrations/           SQLx 数据库迁移
│  └─ src/
│     ├─ handlers/          按业务领域划分的请求处理器
│     ├─ auth.rs            后台令牌验证
│     ├─ models.rs          请求、响应和数据库模型
│     ├─ router.rs          路由及中间件
│     └─ main.rs            应用启动入口
├─ deploy/                   Nginx、systemd 和服务器初始化配置
├─ docker-compose.yml        本地 PostgreSQL
└─ 数据文件/                 招生数据来源 PDF
```

`docs/`、`mkdocs.yml` 和 `overrides/` 是早期静态站点留下的资料，不是当前线上应用的运行入口。当前开发应以 `frontend/` 和 `backend/` 为准。

## 本地启动

以下命令在项目根目录执行。

### 1. 启动 PostgreSQL

确保 Docker Desktop 已启动，然后执行：

```powershell
docker compose up -d postgres
docker compose ps
```

本地数据库监听：

```text
127.0.0.1:55433
```

数据库名是 `iie`，用户名和开发密码均为 `iie_app`。这些凭据只用于本地开发环境。

### 2. 配置后端

```powershell
Copy-Item backend\.env.example backend\.env
```

编辑 `backend/.env`：

```dotenv
DATABASE_URL=postgres://iie_app:iie_app@127.0.0.1:55433/iie
API_BIND=127.0.0.1:9000
ADMIN_TOKEN=替换为至少32位的随机字符串
RUST_LOG=info
UPLOAD_DIR=./uploads
```

变量说明：

| 变量 | 必填 | 默认值 | 用途 |
| --- | --- | --- | --- |
| `DATABASE_URL` | 是 | 无 | PostgreSQL 连接地址 |
| `ADMIN_TOKEN` | 是 | 无 | 后台管理凭据，至少 32 个字符 |
| `API_BIND` | 否 | `127.0.0.1:9000` | API 监听地址 |
| `UPLOAD_DIR` | 否 | `./uploads` | 投稿图片存储目录 |
| `GITHUB_CLIENT_ID` | 评论时必填 | 无 | GitHub OAuth App 的 Client ID |
| `GITHUB_CLIENT_SECRET` | 评论时必填 | 无 | GitHub OAuth App 的 Client secret |
| `GITHUB_OAUTH_REDIRECT_URI` | 评论时必填 | 无 | GitHub 回调地址，例如 `https://iie.cskaoyan.cn/api/v1/auth/github/callback` |
| `RUST_LOG` | 否 | `info` | Rust 日志级别 |

真实的 `.env` 已被 Git 忽略，不要将后台令牌或生产数据库密码提交到仓库。

### 3. 启动后端

打开一个 PowerShell 终端：

```powershell
Set-Location backend
cargo run
```

后端启动时会自动执行 `backend/migrations/` 中尚未执行的数据库迁移。

健康检查：

```text
http://127.0.0.1:9000/api/health
```

### 4. 启动前端

再打开一个 PowerShell 终端：

```powershell
Set-Location frontend
npm ci
npm run dev -- --host 127.0.0.1 --port 4178
```

访问地址：

```text
前台：http://127.0.0.1:4178/
后台：http://127.0.0.1:4178/admin
```

后台没有默认通用密码，使用 `backend/.env` 中配置的 `ADMIN_TOKEN`。

本地开发时，Vite 会把 `/api` 和 `/uploads` 代理到 `http://127.0.0.1:9000`。如需修改目标地址，可设置：

```powershell
$env:API_PROXY_TARGET = "http://127.0.0.1:9000"
```

SvelteKit 服务端请求使用 `API_INTERNAL_URL`，未设置时同样默认为 `http://127.0.0.1:9000`。

## 不使用 Docker

也可以连接本机或远程 PostgreSQL 16+。先创建数据库和用户，然后将 `backend/.env` 中的 `DATABASE_URL` 改为对应地址。数据库表不需要手动创建，后端启动时会自动运行迁移。

## 检查与构建

后端：

```powershell
Set-Location backend
cargo fmt -- --check
cargo check
cargo test
cargo build --release
```

前端：

```powershell
Set-Location frontend
npm ci
npm run check
npm run build
```

生产构建完成后，前端输出位于 `frontend/build/`，后端二进制位于 `backend/target/release/`。

本地运行前端生产构建：

```powershell
Set-Location frontend
$env:API_INTERNAL_URL = "http://127.0.0.1:9000"
$env:HOST = "127.0.0.1"
$env:PORT = "3000"
node build
```

## 生产环境

当前服务器目录：

```text
/srv/iie-cskaoyan/current
├─ frontend/
└─ backend/
```

上传文件独立保存在 `/srv/iie-cskaoyan/uploads`，不能放在某个发布版本的
`backend/uploads` 中。切换 `current` 前需确认该目录仍存在、归属 `deploy:deploy`，并由
`iie-api.service` 的 `UPLOAD_DIR` 环境变量指向它。

服务结构：

| 服务 | 地址 | systemd 服务 |
| --- | --- | --- |
| SvelteKit SSR | `127.0.0.1:3000` | `iie-web.service` |
| Rust API | `127.0.0.1:9000` | `iie-api.service` |
| PostgreSQL | `127.0.0.1:5432` | `postgresql.service` |
| HTTPS 入口 | `80` / `443` | `nginx.service` |

Nginx 将 `/api/` 和 `/uploads/` 转发到 Rust API，其余请求转发到 SvelteKit。生产环境变量位于：

```text
/etc/iie-cskaoyan/api.env
/etc/iie-cskaoyan/web.env
```

评论使用 GitHub OAuth 登录。上线前需在 GitHub 创建 OAuth App，将授权回调 URL 设为
`https://iie.cskaoyan.cn/api/v1/auth/github/callback`，并将其 Client ID、Client secret 与回调
URL 写入 `/etc/iie-cskaoyan/api.env` 的 `GITHUB_CLIENT_ID`、`GITHUB_CLIENT_SECRET`、
`GITHUB_OAUTH_REDIRECT_URI`。缺少这三项时，文章和公开评论仍可查看，但无法登录发布评论。

常用维护命令：

```bash
systemctl status iie-api iie-web nginx postgresql
journalctl -u iie-api -n 100 --no-pager
journalctl -u iie-web -n 100 --no-pager
nginx -t
```

## 数据与安全

- 官方政策以信工所和国科大当年发布的信息为准。
- 学生整理数据需要保留年份、样本完整率和原始来源。
- 不公开未经授权的个人信息、导师联系方式或群文件隐私数据。
- `ADMIN_TOKEN`、数据库密码和生产环境文件不得提交到 Git。
- 上传目录和 PostgreSQL 数据卷需要单独备份，它们不包含在代码仓库中。
