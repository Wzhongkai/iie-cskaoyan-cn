CREATE TABLE IF NOT EXISTS articles (
    id UUID PRIMARY KEY,
    slug TEXT NOT NULL UNIQUE,
    title TEXT NOT NULL,
    excerpt TEXT,
    body_markdown TEXT NOT NULL,
    category TEXT NOT NULL CHECK (category IN ('initial', 'reexam', 'career', 'policy', 'data', 'notice')),
    year INTEGER,
    status TEXT NOT NULL DEFAULT 'draft' CHECK (status IN ('draft', 'published', 'archived')),
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    published_at TIMESTAMPTZ
);

CREATE INDEX IF NOT EXISTS articles_public_idx ON articles (status, published_at DESC);
CREATE INDEX IF NOT EXISTS articles_category_idx ON articles (category, year DESC);

CREATE TABLE IF NOT EXISTS admission_stats (
    year INTEGER NOT NULL,
    program TEXT NOT NULL,
    applicants INTEGER,
    cutoff INTEGER NOT NULL,
    interviewed INTEGER NOT NULL,
    admitted INTEGER NOT NULL,
    rate DOUBLE PRECISION NOT NULL,
    source_note TEXT,
    PRIMARY KEY (year, program)
);

CREATE TABLE IF NOT EXISTS submissions (
    id UUID PRIMARY KEY,
    reference_code TEXT NOT NULL UNIQUE,
    title TEXT NOT NULL,
    category TEXT NOT NULL CHECK (category IN ('initial', 'reexam', 'career', 'policy', 'data', 'notice')),
    year INTEGER,
    background TEXT,
    contact TEXT,
    body_markdown TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'pending' CHECK (status IN ('pending', 'approved', 'rejected')),
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    reviewed_at TIMESTAMPTZ
);

INSERT INTO admission_stats (year, program, applicants, cutoff, interviewed, admitted, rate, source_note) VALUES
    (2026, '学硕', 770, 315, 216, 55, 25.46, '学生整理报告，非官方统计'),
    (2026, '专硕', 770, 318, 216, 90, 41.67, '学生整理报告，非官方统计'),
    (2025, '学硕/专硕', 560, 275, 211, 137, 64.92, '学生整理报告，非官方统计'),
    (2024, '普通计划', 800, 273, 290, 150, 51.72, '学生整理报告，非官方统计')
ON CONFLICT (year, program) DO NOTHING;

INSERT INTO articles (id, slug, title, excerpt, body_markdown, category, year, status, published_at)
VALUES
    (
      '3d0b2e6d-2d5d-4c9a-83ec-5f8be2e71a10',
      'policy-checklist',
      '报考政策：先核对当年官方文件',
      '专业目录、考试科目和复试规程每年都可能调整，历年资料只能帮助理解竞争结构。',
      '## 核对顺序

1. 查看国科大当年硕士招生专业目录，确认专业、研究方向和初试科目。
2. 查看信工所发布的复试规程、复试名单和拟录取名单。
3. 再使用本站历年数据判断竞争结构。

> 涉及报名资格、专业代码、考试科目和录取规则时，必须回到当年官方文件。

官方入口：

- https://iie.cas.cn/
- https://admission.ucas.ac.cn/
- https://yz.chsi.com.cn/',
      'policy', 2026, 'published', now()
    ),
    (
      '8e7e15a4-8fb5-4de1-a2f1-bfe3ba6e4c41',
      'reexam-preparation-framework',
      '复试准备框架：材料、专业与表达',
      '把复试准备拆成材料层、专业层和表达层，避免只背问题清单。',
      '## 材料层

- 一页中文简历，所有项目和奖项都能经得住追问。
- 成绩单、排名证明、证书和官方要求材料按清单整理。
- 准备 1 分钟与 3 分钟两个版本的自我介绍。

## 专业层

以初试专业基础为底，重新梳理数据结构、组成原理、操作系统和计算机网络。针对简历项目准备目标、个人贡献、关键难点、验证方式和失败复盘。

## 表达层

回答不知道的问题时，先说明边界，再给出已有理解和推导过程，不要编造论文内容、项目贡献或导师方向。',
      'reexam', 2026, 'published', now()
    ),
    (
      '43b44a42-4f8e-4e2c-b56d-9fbf98e4f2a0',
      'initial-388',
      '初试 388：从择校到四科节奏',
      '一名网安科班考生的备考复盘，重点是时间安排和复盘方法，不是资料采购清单。',
      '## 可复用的信息

- 择校不只比分数线，还应同时考虑研究方向、培养成本和未来三年生活。
- 数学与 408 需要持续做题，但后期仍要回到教材和错题。
- 固定日程比临时冲刺更可靠，上午数学、下午 408、晚上英语与政治。

## 阅读边界

作者是网安科班，专业课基础和课程背景与跨考生不同。请提炼何时复盘、如何发现漏洞，不要把个人资料名单当作采购清单。',
      'initial', 2026, 'published', now()
    )
ON CONFLICT (slug) DO NOTHING;
