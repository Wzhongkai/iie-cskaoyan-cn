# 信工所考研信息站

一个面向考研信息整理与经验分享的全栈开源项目。项目包含招生数据展示、Markdown 文章、投稿、图片上传、文章分类、密码保护、GitHub 登录评论与回复，以及后台管理界面。

这是一个可自行部署的模板，不隶属于任何招生单位。页面中的政策、招生计划和分数信息应以当年官方文件为准。

## 功能

- 按年度浏览招生数据、专业、科室、分数段和来源
- 文章发布、编辑、分类、置顶、隐藏和 Markdown/KaTeX 渲染
- 文章访问密码保护
- Markdown 文件或 ZIP 图文包投稿，支持相对路径和 Windows/Linux 绝对路径图片
- GitHub OAuth 评论、一级评论回复和后台按文章删除评论
- 后台管理文章、分类、投稿和招生数据

## 技术栈

- 前端：Svelte 5、SvelteKit、Vite、TypeScript
- 后端：Rust、Axum、SQLx、Tokio
- 数据库：PostgreSQL 16+
- 本地数据库：Docker Compose
- 生产运行：Node.js SSR、Rust API、Nginx、systemd

## 目录

```text
frontend/           SvelteKit 前端和后台
backend/            Rust API、数据库迁移和配置示例
deploy/             Nginx、systemd 和初始化脚本模板
docs/               早期静态资料和 Markdown 文档
docker-compose.yml  本地 PostgreSQL
```

## 环境要求

- Node.js 22.12+
- npm 10+
- Rust stable（建议使用 rustup）
- Docker Desktop 或 Docker Engine + Compose
- PostgreSQL 16+（如果不使用 Docker）

## 本地开发

以下命令在项目根目录执行。

### 1. 启动数据库

```bash
docker compose up -d postgres
docker compose ps
```

默认数据库连接为 `postgres://iie_app:iie_app@127.0.0.1:55433/iie`。生产环境请使用独立的数据库用户和随机密码，不要复用该开发密码。

### 2. 配置后端

```bash
cp backend/.env.example backend/.env
```

编辑 `backend/.env`：

```dotenv
DATABASE_URL=postgres://iie_app:iie_app@127.0.0.1:55433/iie
API_BIND=127.0.0.1:9000
ADMIN_TOKEN=请替换为至少32位的随机字符串
RUST_LOG=info
UPLOAD_DIR=./uploads

# 启用 GitHub 评论时填写
GITHUB_CLIENT_ID=
GITHUB_CLIENT_SECRET=
GITHUB_OAUTH_REDIRECT_URI=http://127.0.0.1:9000/api/v1/auth/github/callback
```

生成管理员令牌示例：

```bash
openssl rand -hex 32
```

`backend/.env` 已被 Git 忽略，严禁提交真实令牌、OAuth Secret 或数据库密码。

### 3. 启动 API

```bash
cd backend
cargo run
```

API 会在启动时自动执行 `backend/migrations/` 中的 SQLx 迁移。健康检查：<http://127.0.0.1:9000/api/health>。

### 4. 启动前端

另开一个终端：

```bash
cd frontend
npm ci
npm run dev -- --host 127.0.0.1 --port 4178
```

访问 <http://127.0.0.1:4178/>，后台为 `/admin`。后台使用 `backend/.env` 中的 `ADMIN_TOKEN`。

开发服务器会把 `/api` 和 `/uploads` 代理到 `http://127.0.0.1:9000`。如 API 不在默认地址，可设置 `API_PROXY_TARGET`；SvelteKit SSR 使用 `API_INTERNAL_URL`。

## GitHub 评论配置

1. 在 GitHub 的 Developer settings 创建 OAuth App。
2. 本地回调地址填写 `http://127.0.0.1:9000/api/v1/auth/github/callback`。
3. 将 Client ID、Client Secret 和回调地址写入 `backend/.env`。
4. 重启 API。所有人可以查看评论，只有 GitHub 登录用户可以发布评论或回复。

部署到自己的域名后，必须在 OAuth App 中把回调地址改成：

```text
https://你的域名/api/v1/auth/github/callback
```

## 检查与构建

```bash
cd backend
cargo fmt -- --check
cargo check
cargo test
cargo build --release --locked

cd ../frontend
npm ci
npm run check
npm run build
```

SvelteKit SSR 运行时需要生产依赖。部署时除了 `frontend/build/`，还要上传 `frontend/package.json` 和 `frontend/package-lock.json`，然后在发布目录执行：

```bash
npm ci --omit=dev
node build
```

只上传 `build/` 会导致服务端找不到 `marked` 等依赖并返回 500。

## 生产部署模板

`deploy/` 提供 systemd、Nginx 和数据库初始化模板。部署前请先：

- 将 `deploy/nginx.conf` 中的 `example.com` 和证书路径替换为自己的域名配置；
- 按实际安装目录修改两个 systemd service 中的路径；
- 使用 HTTPS，并把 GitHub OAuth 回调地址改为自己的 HTTPS 地址；
- 将环境变量放在服务器权限为 `600` 的独立文件中，不要放入发布目录或 Git。

典型发布目录可以使用 `/srv/your-app/current`，上传文件建议放在独立的持久化目录，例如 `/srv/your-app/uploads`。切换 release 前先备份 PostgreSQL 和上传目录，确认 API 健康检查通过后再重启 Web 服务。

## 安全与隐私

- 不要提交 `.env`、数据库 dump、上传目录、私钥、OAuth Secret、PAT 或管理员令牌。
- 生产数据库请使用独立用户、强密码和定期备份。
- 评论依赖 GitHub OAuth；不要在前端暴露 Client Secret。
- 用户投稿可能包含个人信息，公开前应取得授权并进行脱敏。
- 项目内容仅供信息整理，招生政策以官方来源为准。

## 许可证

仓库当前未预设开源许可证。正式发布前请根据代码、文档、示例数据和 PDF 资料的授权情况选择并添加合适的 LICENSE 文件。
