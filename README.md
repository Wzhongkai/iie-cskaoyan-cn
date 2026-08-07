# 信工所考研信息站

面向中国科学院信息工程研究所考生的公益信息站，包含招生政策、2024-2026 年数据、初试经验、复试经验和就业分享。

## 本地预览

```powershell
python -m venv .venv
.venv\Scripts\python -m pip install -r requirements.txt
.venv\Scripts\python -m mkdocs serve
```

打开 `http://127.0.0.1:8000/`。

## 部署到 GitHub Pages

1. 将仓库推送到 GitHub。
2. 在仓库 `Settings > Pages > Build and deployment` 中选择 `GitHub Actions`。
3. 推送到 `main` 或 `master` 后，工作流会自动构建和发布。

## 启用 Giscus 评论

评论配置已经接入，但默认关闭，因为 Giscus 需要仓库创建后的 `repo id` 和 `category id`。

1. 将仓库设为公开并在 `Settings > General > Features` 中开启 Discussions。
2. 安装 [Giscus App](https://github.com/apps/giscus)。
3. 在 [giscus.app/zh-CN](https://giscus.app/zh-CN) 生成配置。
4. 将生成的参数填写到 `mkdocs.yml` 的 `extra.giscus`，并设置 `enabled: true`。

带有以下页面元数据的文章会显示评论区：

```yaml
---
comments: true
---
```

## 内容原则

- 官方政策以信工所和国科大当年发布为准。
- 学生整理数据必须标注年份、样本完整率和原始来源。
- 转载经验贴只做摘要与索引，正文链接回原作者。
- 不公开未经授权的个人信息、导师联系方式或群文件隐私数据。

