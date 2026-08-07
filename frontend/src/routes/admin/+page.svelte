<script lang="ts">
  import {
    Activity,
    Archive,
    BarChart3,
    BookOpen,
    Check,
    ChevronLeft,
    ChevronRight,
    Database,
    ExternalLink,
    Eye,
    FilePenLine,
    FileText,
    House,
    KeyRound,
    LayoutDashboard,
    LogOut,
    Menu,
    PanelLeftClose,
    Pin,
    Plus,
    RefreshCw,
    Save,
    Search,
    ShieldCheck,
    Trash2,
    Upload,
    X
  } from '@lucide/svelte';
  import ReportDataManager from '$lib/components/ReportDataManager.svelte';
  import type { AnnualReportOverview, Article, ArticleCategory, Submission } from '$lib/types';

  type AdminView = 'overview' | 'homepage' | 'review' | 'articles' | 'stats';
  type ArticleDraft = { slug: string; title: string; excerpt: string; body_markdown: string; category: ArticleCategory; year: string; status: 'draft' | 'published' | 'archived'; is_pinned: boolean };
  type ReportDraft = { year: string; title: string; exam_applicants_min: string; applicants_note: string; national_total_cutoff: string; national_politics_english_cutoff: string; national_subject_cutoff: string; academic_cutoff: string; professional_cutoff: string; interviewed_total: string; admitted_total: string; academic_admitted: string; professional_admitted: string; recommendation_total: string; direct_phd: string; recommendation_academic: string; recommendation_professional: string; exam_source_sample: string; exam_source_coverage: string; score_formula: string; source_file: string; source_note: string };

  const blankArticle = (): ArticleDraft => ({ slug: '', title: '', excerpt: '', body_markdown: '', category: 'initial', year: String(new Date().getFullYear()), status: 'draft', is_pinned: false });
  const blankReport = (): ReportDraft => ({ year: String(new Date().getFullYear()), title: '', exam_applicants_min: '', applicants_note: '', national_total_cutoff: '', national_politics_english_cutoff: '', national_subject_cutoff: '', academic_cutoff: '', professional_cutoff: '', interviewed_total: '', admitted_total: '', academic_admitted: '', professional_admitted: '', recommendation_total: '', direct_phd: '', recommendation_academic: '', recommendation_professional: '', exam_source_sample: '', exam_source_coverage: '', score_formula: '总成绩 = 初试总分 / 10 + 复试成绩 / 2', source_file: '', source_note: '学生整理，非官方统计' });
  const viewTitles: Record<AdminView, [string, string]> = {
    overview: ['工作台概览', '查看内容库状态与待处理事项。'],
    homepage: ['主页管理', '查看主页与历年数据的自动同步状态。所有内容均在招生数据中维护。'],
    review: ['稿件审核', '筛选投稿并完成审核与发布。'],
    articles: ['文章管理', '维护审核中、已隐藏和已发布的文章。'],
    stats: ['招生数据', '统一维护年度总览、生源院校、科目、分数段和科室数据。']
  };
  const categoryLabels: Record<ArticleCategory, string> = { initial: '初试经验', reexam: '复试经验', career: '就业分享', policy: '政策资料', data: '数据纠错' };
  const statusLabels: Record<string, string> = { pending: '待审核', approved: '已通过', rejected: '已驳回', draft: '审核中', published: '已发布', archived: '已隐藏' };

  let token = $state('');
  let authenticated = $state(false);
  let activeView = $state<AdminView>('overview');
  let sidebarCollapsed = $state(false);
  let mobileMenuOpen = $state(false);
  let submissions = $state<Submission[]>([]);
  let articles = $state<Article[]>([]);
  let reports = $state<AnnualReportOverview[]>([]);
  let articleDraft = $state<ArticleDraft>(blankArticle());
  let reportDraft = $state<ReportDraft>(blankReport());
  let editingArticleId = $state<string | null>(null);
  let selectedSubmission = $state<Submission | null>(null);
  let drawer = $state<'submission' | 'article' | 'report' | null>(null);
  let submissionSearch = $state('');
  let submissionFilter = $state<'all' | 'pending' | 'approved' | 'rejected'>('all');
  let articleSearch = $state('');
  let articleFilter = $state<'all' | 'draft' | 'published' | 'archived'>('all');
  let errorMessage = $state('');
  let notice = $state('');
  let loading = $state(false);
  let saving = $state(false);

  if (typeof sessionStorage !== 'undefined') token = sessionStorage.getItem('iie-admin-token') ?? '';

  const pendingCount = $derived(submissions.filter((item) => item.status === 'pending').length);
  const publishedCount = $derived(articles.filter((item) => item.status === 'published').length);
  const reviewCount = $derived(articles.filter((item) => item.status === 'draft').length);
  const homepageYear = $derived(reports[0]?.year ?? null);
  const filteredSubmissions = $derived(submissions.filter((item) => {
    const query = submissionSearch.trim().toLocaleLowerCase();
    const matchesSearch = !query || `${item.reference_code} ${item.title} ${item.contact ?? ''}`.toLocaleLowerCase().includes(query);
    return matchesSearch && (submissionFilter === 'all' || item.status === submissionFilter);
  }));
  const filteredArticles = $derived(articles.filter((item) => {
    const query = articleSearch.trim().toLocaleLowerCase();
    const matchesSearch = !query || `${item.title} ${item.slug} ${item.excerpt ?? ''}`.toLocaleLowerCase().includes(query);
    return matchesSearch && (articleFilter === 'all' || item.status === articleFilter);
  }));

  const numberOrNull = (value: string) => value.trim() === '' ? null : Number(value);
  const dateText = (value: string) => new Date(value).toLocaleString('zh-CN', { month: '2-digit', day: '2-digit', hour: '2-digit', minute: '2-digit' });

  async function parse(response: Response) {
    const payload = await response.json();
    if (!response.ok) throw new Error(payload.error || '请求失败');
    return payload;
  }

  function showNotice(message: string) {
    notice = message;
    setTimeout(() => { if (notice === message) notice = ''; }, 3200);
  }

  async function loadWorkspace() {
    errorMessage = ''; loading = true;
    try {
      const headers = { 'x-admin-token': token };
      const [reviewData, articleData, reportData] = await Promise.all([
        fetch('/api/v1/admin/submissions', { headers }).then(parse),
        fetch('/api/v1/admin/articles', { headers }).then(parse),
        fetch('/api/v1/admin/reports', { headers }).then(parse)
      ]);
      submissions = reviewData;
      articles = articleData;
      reports = reportData;
      authenticated = true;
      sessionStorage.setItem('iie-admin-token', token);
    } catch (error) {
      authenticated = false;
      errorMessage = error instanceof Error ? error.message : '无法读取后台数据';
    } finally { loading = false; }
  }

  function changeView(view: AdminView) {
    activeView = view;
    mobileMenuOpen = false;
    errorMessage = '';
    notice = '';
  }

  async function updateSubmission(id: string, status: 'approved' | 'rejected' | 'pending') {
    saving = true; errorMessage = '';
    try {
      const item = await fetch(`/api/v1/admin/submissions/${id}`, { method: 'PATCH', headers: { 'content-type': 'application/json', 'x-admin-token': token }, body: JSON.stringify({ status }) }).then(parse);
      submissions = submissions.map((submission) => submission.id === id ? item : submission);
      if (selectedSubmission?.id === id) selectedSubmission = item;
      showNotice(status === 'approved' ? '稿件已通过并发布。' : status === 'rejected' ? '稿件已驳回。' : '稿件已恢复待审核。');
    } catch (error) { errorMessage = error instanceof Error ? error.message : '更新失败'; }
    finally { saving = false; }
  }

  function inspectSubmission(item: Submission) { selectedSubmission = item; drawer = 'submission'; }

  function editArticle(article: Article) {
    editingArticleId = article.id;
    articleDraft = { slug: article.slug, title: article.title, excerpt: article.excerpt ?? '', body_markdown: article.body_markdown, category: article.category, year: article.year?.toString() ?? String(new Date().getFullYear()), status: article.status, is_pinned: article.is_pinned };
    drawer = 'article'; errorMessage = '';
  }

  function newArticle() { editingArticleId = null; articleDraft = blankArticle(); drawer = 'article'; errorMessage = ''; }

  async function readArticleMarkdown(event: Event) {
    const input = event.currentTarget as HTMLInputElement;
    const file = input.files?.[0];
    if (!file) return;
    if (!file.name.toLowerCase().endsWith('.md') || file.size > 2 * 1024 * 1024) {
      errorMessage = '请选择不超过 2 MB 的 .md 文件';
      input.value = '';
      return;
    }
    articleDraft.body_markdown = await file.text();
    if (!articleDraft.title) articleDraft.title = file.name.replace(/\.md$/i, '').replace(/[-_]+/g, ' ');
    errorMessage = '';
  }

  async function saveArticle() {
    saving = true; errorMessage = '';
    try {
      const payload = { ...articleDraft, year: numberOrNull(articleDraft.year) };
      const url = editingArticleId ? `/api/v1/admin/articles/${editingArticleId}` : '/api/v1/admin/articles';
      const method = editingArticleId ? 'PATCH' : 'POST';
      const article = await fetch(url, { method, headers: { 'content-type': 'application/json', 'x-admin-token': token }, body: JSON.stringify(payload) }).then(parse);
      articles = editingArticleId ? articles.map((item) => item.id === article.id ? article : item) : [article, ...articles];
      editingArticleId = article.id;
      drawer = null;
      showNotice(article.status === 'published' ? '文章已保存并发布。' : '文章已保存。');
    } catch (error) { errorMessage = error instanceof Error ? error.message : '文章保存失败'; }
    finally { saving = false; }
  }

  async function updateArticleStatus(article: Article, event: Event) {
    const select = event.currentTarget as HTMLSelectElement;
    const status = select.value as Article['status'];
    if (status === article.status) return;
    saving = true; errorMessage = '';
    try {
      const payload = {
        slug: article.slug,
        title: article.title,
        excerpt: article.excerpt,
        body_markdown: article.body_markdown,
        category: article.category,
        year: article.year,
        status,
        is_pinned: article.is_pinned
      };
      const updated = await fetch(`/api/v1/admin/articles/${article.id}`, { method: 'PATCH', headers: { 'content-type': 'application/json', 'x-admin-token': token }, body: JSON.stringify(payload) }).then(parse);
      articles = articles.map((item) => item.id === updated.id ? updated : item);
      if (editingArticleId === updated.id) articleDraft.status = updated.status;
      if (updated.status === 'draft' || updated.status === 'published') {
        submissions = submissions.map((item) => item.published_article_id === updated.id
          ? { ...item, status: updated.status === 'draft' ? 'pending' : 'approved' }
          : item);
      }
    } catch (error) {
      select.value = article.status;
      errorMessage = error instanceof Error ? error.message : '文章状态更新失败';
    } finally { saving = false; }
  }

  async function deleteArticle(article: Article) {
    if (!confirm(`确定永久删除文章“${article.title}”吗？删除后无法恢复。`)) return;
    saving = true; errorMessage = '';
    try {
      const response = await fetch(`/api/v1/admin/articles/${article.id}`, { method: 'DELETE', headers: { 'x-admin-token': token } });
      if (!response.ok) {
        const payload = await response.json().catch(() => null);
        throw new Error(payload?.error || '文章删除失败');
      }
      articles = articles.filter((item) => item.id !== article.id);
      if (editingArticleId === article.id) drawer = null;
      editingArticleId = null;
      showNotice('文章已删除。');
    } catch (error) { errorMessage = error instanceof Error ? error.message : '文章删除失败'; }
    finally { saving = false; }
  }

  function editReport(report: AnnualReportOverview) {
    reportDraft = { year: String(report.year), title: report.title, exam_applicants_min: report.exam_applicants_min?.toString() ?? '', applicants_note: report.applicants_note ?? '', national_total_cutoff: report.national_total_cutoff?.toString() ?? '', national_politics_english_cutoff: report.national_politics_english_cutoff?.toString() ?? '', national_subject_cutoff: report.national_subject_cutoff?.toString() ?? '', academic_cutoff: report.academic_cutoff?.toString() ?? '', professional_cutoff: report.professional_cutoff?.toString() ?? '', interviewed_total: report.interviewed_total?.toString() ?? '', admitted_total: report.admitted_total?.toString() ?? '', academic_admitted: report.academic_admitted?.toString() ?? '', professional_admitted: report.professional_admitted?.toString() ?? '', recommendation_total: report.recommendation_total?.toString() ?? '', direct_phd: report.direct_phd?.toString() ?? '', recommendation_academic: report.recommendation_academic?.toString() ?? '', recommendation_professional: report.recommendation_professional?.toString() ?? '', exam_source_sample: report.exam_source_sample?.toString() ?? '', exam_source_coverage: report.exam_source_coverage?.toString() ?? '', score_formula: report.score_formula ?? '', source_file: report.source_file, source_note: report.source_note };
    drawer = 'report'; errorMessage = '';
  }

  function newReport() { reportDraft = { ...blankReport(), year: String((homepageYear ?? new Date().getFullYear()) + 1) }; drawer = 'report'; errorMessage = ''; }

  async function saveReport() {
    saving = true; errorMessage = '';
    try {
      const numeric = ['exam_applicants_min', 'national_total_cutoff', 'national_politics_english_cutoff', 'national_subject_cutoff', 'academic_cutoff', 'professional_cutoff', 'interviewed_total', 'admitted_total', 'academic_admitted', 'professional_admitted', 'recommendation_total', 'direct_phd', 'recommendation_academic', 'recommendation_professional', 'exam_source_sample'] as const;
      const payload: Record<string, string | number | null> = { ...reportDraft, year: Number(reportDraft.year), exam_source_coverage: numberOrNull(reportDraft.exam_source_coverage) };
      for (const key of numeric) payload[key] = numberOrNull(reportDraft[key]);
      const report = await fetch('/api/v1/admin/reports', { method: 'POST', headers: { 'content-type': 'application/json', 'x-admin-token': token }, body: JSON.stringify(payload) }).then(parse);
      reports = [report, ...reports.filter((item) => item.year !== report.year)].sort((a, b) => b.year - a.year);
      drawer = null;
      showNotice(report.year === Math.max(...reports.map((item) => item.year)) ? `${report.year} 年已保存，并作为主页年度。` : `${report.year} 年报告总览已保存。`);
    } catch (error) { errorMessage = error instanceof Error ? error.message : '年度报告保存失败'; }
    finally { saving = false; }
  }

  function logout() {
    token = ''; authenticated = false; submissions = []; articles = []; reports = []; drawer = null;
    sessionStorage.removeItem('iie-admin-token');
  }
</script>

<svelte:head><title>内容维护后台 | 信工所考研信息站</title><meta name="robots" content="noindex" /></svelte:head>

{#if !authenticated}
  <div class="login-page">
    <form class="login-panel" onsubmit={(event) => { event.preventDefault(); loadWorkspace(); }}>
      <span class="login-mark"><ShieldCheck size={23} /></span>
      <p class="login-brand">IIE CONTENT STUDIO</p>
      <h1>内容维护后台</h1>
      <p class="login-copy">请输入管理令牌，进入内容与数据管理后台。</p>
      <label for="admin-token">管理令牌</label>
      <div class="login-input"><KeyRound size={17} /><input id="admin-token" type="password" bind:value={token} autocomplete="off" /></div>
      {#if errorMessage}<p class="login-error">{errorMessage}</p>{/if}
      <button type="submit" disabled={loading || !token}>{#if loading}<RefreshCw class="spin" size={17} />正在验证{:else}<KeyRound size={17} />登录后台{/if}</button>
    </form>
  </div>
{:else}
  <div class="admin-page">
    <aside class:collapsed={sidebarCollapsed} class="sidebar desktop-sidebar">
      <div class="sidebar-brand"><span>IIE</span>{#if !sidebarCollapsed}<div><strong>内容工作台</strong><small>内容管理</small></div><button onclick={() => sidebarCollapsed = true} aria-label="收起侧边栏" title="收起侧边栏"><PanelLeftClose size={17} /></button>{/if}</div>
      {#if sidebarCollapsed}<button class="expand-button" onclick={() => sidebarCollapsed = false} aria-label="展开侧边栏" title="展开侧边栏"><ChevronRight size={18} /></button>{/if}
      <nav aria-label="后台导航">
        {#each [
          { id: 'overview' as AdminView, label: '概览', icon: LayoutDashboard },
           { id: 'homepage' as AdminView, label: '主页管理', icon: House },
          { id: 'review' as AdminView, label: '稿件审核', icon: FileText },
          { id: 'articles' as AdminView, label: '文章管理', icon: BookOpen },
          { id: 'stats' as AdminView, label: '招生数据', icon: Database }
        ] as item}
          {@const Icon = item.icon}
          <button class:active={activeView === item.id} onclick={() => changeView(item.id)} title={sidebarCollapsed ? item.label : undefined}><Icon size={17} />{#if !sidebarCollapsed}<span>{item.label}</span>{#if item.id === 'review'}<b>{pendingCount}</b>{/if}{/if}</button>
        {/each}
      </nav>
      {#if !sidebarCollapsed}<div class="sidebar-status"><span><Activity size={15} />站点状态</span><strong>服务正常</strong><small>站点与数据服务运行正常</small></div>{/if}
    </aside>

      {#if mobileMenuOpen}
       <div class="mobile-nav-layer"><button class="mobile-scrim" aria-label="关闭菜单" onclick={() => mobileMenuOpen = false}></button><aside class="sidebar mobile-sidebar"><div class="sidebar-brand"><span>IIE</span><div><strong>内容工作台</strong><small>内容管理</small></div><button onclick={() => mobileMenuOpen = false} aria-label="关闭侧边栏"><X size={18} /></button></div><nav aria-label="移动后台导航"><button class:active={activeView === 'overview'} onclick={() => changeView('overview')}><LayoutDashboard size={17} />概览</button><button class:active={activeView === 'homepage'} onclick={() => changeView('homepage')}><House size={17} />主页管理</button><button class:active={activeView === 'review'} onclick={() => changeView('review')}><FileText size={17} />稿件审核 <b>{pendingCount}</b></button><button class:active={activeView === 'articles'} onclick={() => changeView('articles')}><BookOpen size={17} />文章管理</button><button class:active={activeView === 'stats'} onclick={() => changeView('stats')}><Database size={17} />招生数据</button></nav></aside></div>
    {/if}

    <div class:wide={sidebarCollapsed} class="workspace">
      <header class="topbar">
        <div class="topbar-title"><button class="mobile-menu" onclick={() => mobileMenuOpen = true} aria-label="打开菜单"><Menu size={18} /></button><div><small>IIE / 内容管理</small><strong>{viewTitles[activeView][0]}</strong></div></div>
        <div class="topbar-actions"><a href="/" target="_blank" rel="noreferrer">查看网站 <ExternalLink size={14} /></a><button onclick={loadWorkspace} aria-label="刷新数据" title="刷新数据"><RefreshCw class={loading ? 'spin' : undefined} size={16} /></button><button onclick={logout} aria-label="退出登录" title="退出登录"><LogOut size={16} /></button><div class="administrator"><span>WK</span><div><strong>内容管理员</strong><small>站点维护</small></div></div></div>
      </header>

      <main class="admin-content">
        <div class="view-heading"><div><h1>{viewTitles[activeView][0]}</h1><p>{viewTitles[activeView][1]}</p></div>{#if activeView === 'homepage'}<button class="primary-action" onclick={() => changeView('stats')}><Database size={16} />管理招生数据</button>{:else if activeView === 'articles'}<button class="primary-action" onclick={newArticle}><Plus size={16} />新建文章</button>{/if}</div>

        {#if errorMessage}<div class="alert error">{errorMessage}<button onclick={() => errorMessage = ''} aria-label="关闭错误"><X size={15} /></button></div>{/if}
        {#if notice}<div class="alert success"><Check size={15} />{notice}</div>{/if}

        {#if activeView === 'overview'}
          <div class="metric-grid">
            <button onclick={() => changeView('review')}><span class="metric-icon blue"><FileText size={18} /></span><small>待审核稿件</small><strong>{String(pendingCount).padStart(2, '0')}</strong><p>需要维护者处理</p></button>
            <button onclick={() => changeView('articles')}><span class="metric-icon green"><BookOpen size={18} /></span><small>公开文章</small><strong>{String(publishedCount).padStart(2, '0')}</strong><p>当前内容库可见</p></button>
            <button onclick={() => changeView('articles')}><span class="metric-icon amber"><FilePenLine size={18} /></span><small>审核中文章</small><strong>{String(reviewCount).padStart(2, '0')}</strong><p>等待确认后发布</p></button>
            <button onclick={() => changeView('stats')}><span class="metric-icon violet"><BarChart3 size={18} /></span><small>年度报告</small><strong>{String(reports.length).padStart(2, '0')}</strong><p>主页及历年完整数据</p></button>
          </div>

          <div class="overview-grid">
            <section class="panel recent-panel"><div class="panel-head"><div><h2>最近投稿</h2><p>优先处理待审核内容</p></div><button onclick={() => changeView('review')}>查看全部 <ChevronRight size={15} /></button></div><div class="table-scroll"><table><thead><tr><th>稿件</th><th>分类</th><th>提交时间</th><th>状态</th><th></th></tr></thead><tbody>{#each submissions.slice(0, 6) as item}<tr><td><strong>{item.title}</strong><small>{item.reference_code}</small></td><td>{categoryLabels[item.category]}</td><td>{dateText(item.created_at)}</td><td><span class="status {item.status}">{statusLabels[item.status]}</span></td><td><button class="row-action" onclick={() => inspectSubmission(item)} aria-label="查看稿件"><Eye size={15} /></button></td></tr>{:else}<tr><td class="empty-cell" colspan="5">暂无投稿记录</td></tr>{/each}</tbody></table></div></section>
            <section class="panel quick-panel"><div class="panel-head"><div><h2>快捷入口</h2><p>常用维护操作</p></div></div><button onclick={() => { changeView('review'); submissionFilter = 'pending'; }}><span class="quick-icon"><FileText size={18} /></span><div><strong>处理待审稿件</strong><small>{pendingCount} 篇内容等待处理</small></div><span class="quick-chevron"><ChevronRight size={17} /></span></button><button onclick={() => { changeView('articles'); newArticle(); }}><span class="quick-icon"><FilePenLine size={18} /></span><div><strong>撰写新文章</strong><small>创建草稿或直接发布</small></div><span class="quick-chevron"><ChevronRight size={17} /></span></button><button onclick={() => changeView('stats')}><span class="quick-icon"><Database size={18} /></span><div><strong>维护年度报告</strong><small>统一维护主页与历年完整数据</small></div><span class="quick-chevron"><ChevronRight size={17} /></span></button></section>
          </div>
        {:else if activeView === 'homepage'}
          <section class="panel management-panel"><div class="homepage-banner"><span class="metric-icon green"><House size={18} /></span><div><small>当前主页年度</small><strong>{homepageYear ?? '未配置'}</strong><p>主页自动选择招生数据中年份最高的年度报告。新增更高年份后无需在这里重复配置。</p></div><a class="primary-action" href="/" target="_blank" rel="noreferrer">查看主页 <ExternalLink size={15} /></a></div><div class="table-scroll"><table><thead><tr><th>年份</th><th>报告标题</th><th>统考录取</th><th>推免录取</th><th>数据源</th><th>同步状态</th></tr></thead><tbody>{#each reports as report}<tr class:emphasis={report.year === homepageYear}><td><strong>{report.year}</strong></td><td><strong>{report.title}</strong><small>{report.source_note}</small></td><td>{report.admitted_total ?? '未填'}</td><td>{report.recommendation_total ?? '未填'}</td><td class="source-cell">{report.source_file}</td><td>{#if report.year === homepageYear}<span class="status published">主页已同步</span>{:else}<span class="status archived">历史页已同步</span>{/if}</td></tr>{:else}<tr><td class="empty-cell" colspan="6">还没有年度报告，请前往招生数据新增。</td></tr>{/each}</tbody></table></div><p class="result-count">此页面只显示自动同步结果。所有数据统一在“招生数据”维护。</p></section>
        {:else if activeView === 'review'}
          <section class="panel management-panel">
            <div class="filters"><label><Search size={16} /><input bind:value={submissionSearch} placeholder="搜索稿件、编号或联系方式" /></label><select bind:value={submissionFilter}><option value="all">全部状态</option><option value="pending">待审核</option><option value="approved">已通过</option><option value="rejected">已驳回</option></select></div>
            <div class="segmented"><button class:active={submissionFilter === 'all'} onclick={() => submissionFilter = 'all'}>全部 {submissions.length}</button><button class:active={submissionFilter === 'pending'} onclick={() => submissionFilter = 'pending'}>待审核 {pendingCount}</button><button class:active={submissionFilter === 'approved'} onclick={() => submissionFilter = 'approved'}>已通过 {submissions.filter((item) => item.status === 'approved').length}</button><button class:active={submissionFilter === 'rejected'} onclick={() => submissionFilter = 'rejected'}>已驳回 {submissions.filter((item) => item.status === 'rejected').length}</button></div>
            <div class="table-scroll"><table><thead><tr><th>稿件信息</th><th>分类</th><th>年份</th><th>联系方式</th><th>提交时间</th><th>状态</th><th></th></tr></thead><tbody>{#each filteredSubmissions as item}<tr class:emphasis={item.status === 'pending'}><td><strong>{item.title}</strong><small>{item.reference_code}</small></td><td>{categoryLabels[item.category]}</td><td>{item.year ?? '未填写'}</td><td class="truncate-cell">{item.contact || '未提供'}</td><td>{dateText(item.created_at)}</td><td><span class="status {item.status}">{statusLabels[item.status]}</span></td><td><button class="row-action" onclick={() => inspectSubmission(item)} aria-label="查看稿件" title="查看稿件"><Eye size={16} /></button></td></tr>{:else}<tr><td class="empty-cell" colspan="7">没有符合条件的稿件</td></tr>{/each}</tbody></table></div><p class="result-count">当前显示 {filteredSubmissions.length} / {submissions.length} 篇稿件</p>
          </section>
        {:else if activeView === 'articles'}
          <section class="panel management-panel">
            <div class="filters"><label><Search size={16} /><input bind:value={articleSearch} placeholder="搜索标题、文章网址或摘要" /></label><select bind:value={articleFilter}><option value="all">全部状态</option><option value="published">已发布</option><option value="draft">审核中</option><option value="archived">已隐藏</option></select></div>
            <div class="segmented"><button class:active={articleFilter === 'all'} onclick={() => articleFilter = 'all'}>全部 {articles.length}</button><button class:active={articleFilter === 'published'} onclick={() => articleFilter = 'published'}>已发布 {publishedCount}</button><button class:active={articleFilter === 'draft'} onclick={() => articleFilter = 'draft'}>审核中 {reviewCount}</button><button class:active={articleFilter === 'archived'} onclick={() => articleFilter = 'archived'}>已隐藏 {articles.filter((item) => item.status === 'archived').length}</button></div>
            <div class="table-scroll"><table><thead><tr><th>文章</th><th>置顶</th><th>分类</th><th>年份</th><th>更新时间</th><th>状态</th><th>操作</th></tr></thead><tbody>{#each filteredArticles as article}<tr><td><strong>{article.title}</strong><small>{article.slug}</small></td><td>{#if article.is_pinned}<span class="pin-indicator"><Pin size={12} />置顶</span>{:else}—{/if}</td><td>{categoryLabels[article.category]}</td><td>{article.year ?? '未填写'}</td><td>{dateText(article.updated_at)}</td><td><select class="status-select {article.status}" value={article.status} disabled={saving} aria-label={`修改文章 ${article.title} 的状态`} onchange={(event) => updateArticleStatus(article, event)}><option value="draft">审核中</option><option value="archived">已隐藏</option><option value="published">已发布</option></select></td><td><div class="row-actions"><button class="row-action" disabled={saving} onclick={() => editArticle(article)} aria-label={`编辑文章 ${article.title}`} title="编辑文章"><FilePenLine size={16} /></button><button class="row-action danger" disabled={saving} onclick={() => deleteArticle(article)} aria-label={`删除文章 ${article.title}`} title="删除文章"><Trash2 size={16} /></button></div></td></tr>{:else}<tr><td class="empty-cell" colspan="7">没有符合条件的文章</td></tr>{/each}</tbody></table></div><p class="result-count">当前显示 {filteredArticles.length} / {articles.length} 篇文章</p>
          </section>
        {:else}
          <ReportDataManager {token} {reports} onEditReport={editReport} onNewReport={newReport} />
        {/if}
      </main>
    </div>

    {#if drawer}
      <div class="drawer-layer"><button class="drawer-scrim" aria-label="关闭详情" onclick={() => drawer = null}></button><aside class="drawer" aria-label="管理详情面板"><div class="drawer-head"><div><small>{drawer === 'submission' ? '稿件审核' : drawer === 'article' ? '文章编辑' : '年度报告'}</small><h2>{drawer === 'submission' ? '审核稿件' : drawer === 'article' ? (editingArticleId ? '编辑文章' : '新建文章') : '年度报告总览'}</h2></div><button onclick={() => drawer = null} aria-label="关闭"><X size={19} /></button></div>
        {#if drawer === 'submission' && selectedSubmission}<div class="drawer-body"><div class="submission-title"><span class="status {selectedSubmission.status}">{statusLabels[selectedSubmission.status]}</span><h3>{selectedSubmission.title}</h3><p>{selectedSubmission.reference_code} · {categoryLabels[selectedSubmission.category]} · {selectedSubmission.year ?? '未标年份'}</p></div><dl class="submission-meta"><div><dt>投稿背景</dt><dd>{selectedSubmission.background || '未填写'}</dd></div><div><dt>联系信息</dt><dd>{selectedSubmission.contact || '未提供'}</dd></div><div><dt>提交时间</dt><dd>{new Date(selectedSubmission.created_at).toLocaleString('zh-CN')}</dd></div></dl><div class="markdown-preview"><span>Markdown 正文</span><pre>{selectedSubmission.body_markdown}</pre></div></div><div class="drawer-actions"><button class="secondary-button" disabled={saving} onclick={() => updateSubmission(selectedSubmission!.id, 'rejected')}><X size={16} />驳回</button>{#if selectedSubmission.status !== 'pending'}<button class="secondary-button" disabled={saving} onclick={() => updateSubmission(selectedSubmission!.id, 'pending')}><Archive size={16} />恢复待审</button>{/if}<button class="primary-action" disabled={saving} onclick={() => updateSubmission(selectedSubmission!.id, 'approved')}><Check size={16} />通过并发布</button></div>
        {:else if drawer === 'article'}<form class="drawer-form" onsubmit={(event) => { event.preventDefault(); saveArticle(); }}><div class="drawer-body form-fields"><label class="full">标题<input bind:value={articleDraft.title} required maxlength="120" /></label><label>文章网址（英文）<input bind:value={articleDraft.slug} required pattern="[a-z0-9-]+" placeholder="reexam-notes-2026" /></label><label>状态<select bind:value={articleDraft.status}><option value="draft">审核中</option><option value="archived">已隐藏</option><option value="published">已发布</option></select></label><label>分类<select bind:value={articleDraft.category}><option value="initial">初试经验</option><option value="reexam">复试经验</option><option value="career">就业分享</option><option value="policy">政策资料</option><option value="data">数据纠错</option></select></label><label>相关年份<input type="number" min="2010" max="2100" bind:value={articleDraft.year} /></label><label class="pin-field"><input type="checkbox" bind:checked={articleDraft.is_pinned} /><span><Pin size={14} />在文章列表置顶</span></label><label class="full">摘要<input maxlength="240" bind:value={articleDraft.excerpt} /></label><div class="full markdown-field"><label for="article-markdown-body">Markdown 正文</label><input class="file-input" id="admin-md-file" type="file" accept=".md,text/markdown" onchange={readArticleMarkdown} /><label class="upload-button" for="admin-md-file"><Upload size={15} />导入 .md</label><textarea id="article-markdown-body" bind:value={articleDraft.body_markdown} required minlength="20" maxlength="500000"></textarea></div></div><div class="drawer-actions">{#if editingArticleId}<button type="button" class="danger-button" disabled={saving} onclick={() => { const article = articles.find((item) => item.id === editingArticleId); if (article) deleteArticle(article); }}><Trash2 size={16} />删除文章</button>{/if}<span class="action-spacer"></span><button type="button" class="secondary-button" onclick={() => drawer = null}>取消</button><button class="primary-action" type="submit" disabled={saving}><Save size={16} />{saving ? '保存中...' : '保存修改'}</button></div></form>
        {:else if drawer === 'report'}<form class="drawer-form" onsubmit={(event) => { event.preventDefault(); saveReport(); }}><div class="drawer-body form-fields"><label>年份<input type="number" min="2010" max="2100" bind:value={reportDraft.year} required /></label><label>报告标题<input maxlength="160" bind:value={reportDraft.title} required /></label><label>统考报考下限<input type="number" min="0" bind:value={reportDraft.exam_applicants_min} /></label><label>进入复试<input type="number" min="0" bind:value={reportDraft.interviewed_total} /></label><label>统考录取<input type="number" min="0" bind:value={reportDraft.admitted_total} /></label><label>学硕录取<input type="number" min="0" bind:value={reportDraft.academic_admitted} /></label><label>专硕录取<input type="number" min="0" bind:value={reportDraft.professional_admitted} /></label><label>推免录取<input type="number" min="0" bind:value={reportDraft.recommendation_total} /></label><label>直博录取<input type="number" min="0" bind:value={reportDraft.direct_phd} /></label><label>推免学硕<input type="number" min="0" bind:value={reportDraft.recommendation_academic} /></label><label>推免专硕<input type="number" min="0" bind:value={reportDraft.recommendation_professional} /></label><label>考研生源样本<input type="number" min="0" bind:value={reportDraft.exam_source_sample} /></label><label>考研样本覆盖率<input type="number" min="0" max="100" step="0.01" bind:value={reportDraft.exam_source_coverage} /></label><label>国家线总分<input type="number" min="0" bind:value={reportDraft.national_total_cutoff} /></label><label>国家线政治/英语<input type="number" min="0" bind:value={reportDraft.national_politics_english_cutoff} /></label><label>国家线专业课<input type="number" min="0" bind:value={reportDraft.national_subject_cutoff} /></label><label>学硕复试线<input type="number" min="0" bind:value={reportDraft.academic_cutoff} /></label><label>专硕复试线<input type="number" min="0" bind:value={reportDraft.professional_cutoff} /></label><label class="full">报考人数口径<input bind:value={reportDraft.applicants_note} /></label><label class="full">总成绩计算方式<input bind:value={reportDraft.score_formula} /></label><label class="full">资料来源文件<input bind:value={reportDraft.source_file} required /></label><label class="full">来源说明<input bind:value={reportDraft.source_note} required /></label></div><div class="drawer-actions"><button type="button" class="secondary-button" onclick={() => drawer = null}>取消</button><button class="primary-action" type="submit" disabled={saving}><Save size={16} />{saving ? '保存中...' : '保存年度报告'}</button></div></form>
        {/if}
      </aside></div>
    {/if}
  </div>
{/if}

<style>
  :global(.admin-root) { min-height: 100dvh; }
  button, input, select, textarea { font: inherit; }
  button { letter-spacing: 0; }
  .login-page { display: grid; min-height: 100dvh; padding: 24px; place-items: center; background: #eef4f8; color: #111820; }
  .login-panel { width: min(100%, 390px); padding: 30px; border: 1px solid #d7e2e9; border-radius: 12px; background: white; box-shadow: 0 24px 70px rgba(28,49,65,.12); }
  .login-mark { display: grid; width: 50px; height: 50px; place-items: center; border-radius: 12px; background: #14261f; color: white; }
  .login-brand { margin: 25px 0 7px; color: #8b9aa4; font-size: 10px; font-weight: 850; }
  .login-panel h1 { margin: 0; font-size: 26px; }
  .login-copy { margin: 9px 0 25px; color: #748590; font-size: 13px; line-height: 1.7; }
  .login-panel > label { display: block; margin-bottom: 8px; color: #536571; font-size: 12px; font-weight: 800; }
  .login-input { display: flex; height: 48px; padding: 0 13px; align-items: center; border: 1px solid #d8e2e9; border-radius: 8px; background: #f9fbfc; color: #8a9aa4; gap: 9px; }
  .login-input:focus-within { border-color: #176b53; box-shadow: 0 0 0 3px rgba(23,107,83,.1); }
  .login-input input { min-width: 0; flex: 1; border: 0; outline: 0; background: transparent; }
  .login-panel > button { display: flex; width: 100%; height: 48px; margin-top: 18px; align-items: center; justify-content: center; border: 0; border-radius: 8px; background: #176b53; color: white; gap: 8px; font-size: 13px; font-weight: 850; cursor: pointer; }
  .login-panel > button:disabled { opacity: .5; }
  .login-error { margin: 10px 0 0; color: #ae463c; font-size: 12px; font-weight: 700; }
  .spin { animation: spin .8s linear infinite; }
  @keyframes spin { to { transform: rotate(360deg); } }

  .admin-page { min-height: 100dvh; background: #eef4f8; color: #111820; }
  .sidebar { position: fixed; z-index: 40; inset: 0 auto 0 0; display: flex; width: 248px; padding: 20px 16px; flex-direction: column; border-right: 1px solid #dce5ec; background: rgba(248,251,253,.97); transition: width .25s ease; }
  .sidebar.collapsed { width: 82px; padding-inline: 12px; }
  .sidebar-brand { display: flex; height: 46px; align-items: center; gap: 11px; }
  .sidebar-brand > span { display: grid; width: 40px; height: 40px; flex: none; place-items: center; border-radius: 12px; background: #14261f; color: white; font-size: 11px; font-weight: 900; box-shadow: 0 10px 25px rgba(20,38,31,.17); }
  .sidebar-brand div { min-width: 0; flex: 1; }
  .sidebar-brand strong, .sidebar-brand small { display: block; white-space: nowrap; }
  .sidebar-brand strong { font-size: 14px; }
  .sidebar-brand small { margin-top: 3px; color: #8b9aa5; font-size: 9px; font-weight: 800; }
  .sidebar-brand button, .expand-button { display: grid; width: 32px; height: 32px; padding: 0; place-items: center; border: 0; border-radius: 8px; background: transparent; color: #81909d; cursor: pointer; }
  .expand-button { margin: 22px auto 0; }
  .sidebar nav { display: grid; margin-top: 36px; gap: 6px; }
  .sidebar nav button { display: flex; width: 100%; height: 44px; padding: 0 12px; align-items: center; border: 0; border-radius: 9px; background: transparent; color: #667684; gap: 11px; font-size: 13px; font-weight: 750; cursor: pointer; }
  .sidebar nav button:hover { background: white; color: #111820; }
  .sidebar nav button.active { background: #14261f; color: white; box-shadow: 0 9px 22px rgba(20,38,31,.15); }
  .sidebar.collapsed nav button { justify-content: center; padding: 0; }
  .sidebar nav b { min-width: 22px; margin-left: auto; padding: 2px 6px; border-radius: 10px; background: #e8eef1; color: #6e7d88; font-size: 10px; text-align: center; }
  .sidebar nav button.active b { background: rgba(255,255,255,.14); color: white; }
  .sidebar-status { margin-top: auto; padding: 15px; border: 1px solid #dce5ec; border-radius: 10px; background: rgba(255,255,255,.75); }
  .sidebar-status span { display: flex; align-items: center; color: #6a7b86; gap: 6px; font-size: 11px; font-weight: 750; }
  .sidebar-status strong, .sidebar-status small { display: block; }
  .sidebar-status strong { margin-top: 11px; color: #176b53; font-size: 12px; }
  .sidebar-status small { margin-top: 5px; color: #98a5ae; font-size: 10px; }
  .workspace { min-height: 100dvh; padding-left: 248px; transition: padding .25s ease; }
  .workspace.wide { padding-left: 82px; }
  .topbar { position: sticky; z-index: 20; top: 0; display: flex; height: 74px; padding: 0 30px; align-items: center; justify-content: space-between; border-bottom: 1px solid #dce5ec; background: rgba(238,244,248,.92); backdrop-filter: blur(16px); }
  .topbar-title { display: flex; align-items: center; gap: 12px; }
  .topbar-title small, .topbar-title strong { display: block; }
  .topbar-title small { color: #9aa7b1; font-size: 9px; font-weight: 850; }
  .topbar-title strong { margin-top: 4px; font-size: 16px; }
  .mobile-menu { display: none; }
  .topbar-actions { display: flex; align-items: center; gap: 9px; }
  .topbar-actions > a { display: inline-flex; height: 38px; padding: 0 12px; align-items: center; border: 1px solid #dce5ec; border-radius: 8px; background: white; color: #687985; gap: 7px; font-size: 11px; font-weight: 800; }
  .topbar-actions > button, .mobile-menu { width: 38px; height: 38px; padding: 0; border: 1px solid #dce5ec; border-radius: 8px; background: white; color: #71818c; cursor: pointer; }
  .administrator { display: flex; margin-left: 7px; padding-left: 15px; align-items: center; border-left: 1px solid #dce5ec; gap: 9px; }
  .administrator > span { display: grid; width: 36px; height: 36px; place-items: center; border-radius: 50%; background: #dcebe5; color: #176b53; font-size: 11px; font-weight: 900; }
  .administrator strong, .administrator small { display: block; }
  .administrator strong { font-size: 11px; }
  .administrator small { margin-top: 2px; color: #96a2ab; font-size: 9px; }
  .admin-content { width: min(1440px, 100%); margin: 0 auto; padding: 32px; }
  .view-heading { display: flex; margin-bottom: 24px; align-items: flex-end; justify-content: space-between; gap: 18px; }
  .view-heading h1 { margin: 0; font-size: 27px; }
  .view-heading p { margin: 7px 0 0; color: #7d8c97; font-size: 13px; }
  .primary-action, .secondary-button { display: inline-flex; min-height: 40px; padding: 0 15px; align-items: center; justify-content: center; border: 0; border-radius: 8px; background: #14261f; color: white; gap: 7px; font-size: 12px; font-weight: 850; cursor: pointer; white-space: nowrap; }
  .primary-action:hover { background: #263b33; }
  .secondary-button { border: 1px solid #d8e2e9; background: white; color: #52636f; }
  .alert { display: flex; min-height: 42px; margin-bottom: 16px; padding: 10px 13px; align-items: center; border-radius: 8px; gap: 8px; font-size: 12px; font-weight: 700; }
  .alert.error { border: 1px solid #eccfc9; background: #fff3f0; color: #9d4339; }
  .alert.success { border: 1px solid #cfe6dc; background: #edf8f3; color: #176b53; }
  .alert button { display: grid; margin-left: auto; padding: 0; border: 0; background: transparent; color: inherit; cursor: pointer; }
  .metric-grid { display: grid; grid-template-columns: repeat(4, minmax(0, 1fr)); gap: 15px; }
  .metric-grid > button { display: grid; min-height: 155px; padding: 18px; grid-template-columns: 1fr auto; border: 1px solid #dce5ec; border-radius: 10px; background: white; color: #111820; text-align: left; box-shadow: 0 10px 30px rgba(36,62,82,.045); cursor: pointer; }
  .metric-grid > button:hover { border-color: #aac2b8; transform: translateY(-1px); }
  .metric-grid small { color: #788894; font-size: 11px; font-weight: 750; }
  .metric-grid strong { align-self: end; font-size: 28px; }
  .metric-grid p { margin: 4px 0 0; align-self: end; color: #8d9aa3; font-size: 10px; font-weight: 650; }
  .metric-icon { display: grid; width: 38px; height: 38px; grid-area: 1 / 2 / 3 / 3; place-items: center; border-radius: 10px; }
  .metric-icon.blue { background: #e8f2ff; color: #316b91; }
  .metric-icon.green { background: #e9f6f0; color: #176b53; }
  .metric-icon.amber { background: #fff2e4; color: #bb632e; }
  .metric-icon.violet { background: #eeeafb; color: #6b58b2; }
  .overview-grid { display: grid; margin-top: 18px; grid-template-columns: minmax(0, 1.45fr) minmax(285px, .55fr); gap: 18px; }
  .panel { overflow: hidden; border: 1px solid #dce5ec; border-radius: 10px; background: white; box-shadow: 0 10px 30px rgba(36,62,82,.045); }
  .panel-head { display: flex; min-height: 70px; padding: 15px 18px; align-items: center; justify-content: space-between; border-bottom: 1px solid #edf1f4; }
  .panel-head h2 { margin: 0; font-size: 14px; }
  .panel-head p { margin: 5px 0 0; color: #94a0aa; font-size: 10px; }
  .panel-head > button { display: flex; align-items: center; border: 0; background: transparent; color: #176b53; gap: 3px; font-size: 11px; font-weight: 800; cursor: pointer; }
  .quick-panel > button { display: flex; width: 100%; min-height: 76px; padding: 13px 17px; align-items: center; border: 0; border-bottom: 1px solid #edf1f4; background: white; color: #111820; text-align: left; gap: 12px; cursor: pointer; }
  .quick-panel > button:last-child { border-bottom: 0; }
  .quick-panel > button:hover { background: #f8fbfa; }
  .quick-icon { display: grid; width: 38px; height: 38px; flex: none; place-items: center; border-radius: 10px; background: #edf4f1; color: #176b53; }
  .quick-panel button div { min-width: 0; flex: 1; }
  .quick-panel strong, .quick-panel small { display: block; }
  .quick-panel strong { font-size: 12px; }
  .quick-panel small { margin-top: 5px; color: #93a0a9; font-size: 10px; }
  .quick-chevron { color: #9aa7b0; }
  .management-panel { min-height: 380px; }
  .filters { display: flex; padding: 15px; align-items: center; border-bottom: 1px solid #edf1f4; gap: 10px; }
  .filters label { display: flex; width: min(100%, 480px); height: 40px; padding: 0 12px; align-items: center; border: 1px solid #dfe7ec; border-radius: 8px; background: #fbfcfd; color: #92a0aa; gap: 8px; }
  .filters input { min-width: 0; flex: 1; border: 0; outline: 0; background: transparent; color: #1c2932; font-size: 12px; }
  .filters select { height: 40px; margin-left: auto; padding: 0 31px 0 11px; border: 1px solid #dfe7ec; border-radius: 8px; outline: 0; background: white; color: #52636f; font-size: 11px; font-weight: 750; }
  .segmented { display: flex; padding: 10px 15px; overflow-x: auto; border-bottom: 1px solid #edf1f4; background: #f7fafb; gap: 3px; }
  .segmented button { min-height: 34px; padding: 0 12px; flex: none; border: 0; border-radius: 7px; background: transparent; color: #74838e; font-size: 11px; font-weight: 800; cursor: pointer; }
  .segmented button.active { background: white; color: #111820; box-shadow: 0 2px 8px rgba(33,51,64,.08); }
  .table-scroll { overflow-x: auto; }
  table { width: 100%; border-collapse: collapse; text-align: left; }
  thead { background: #fbfcfd; color: #99a6af; font-size: 9px; font-weight: 850; }
  th { height: 38px; padding: 0 16px; border-bottom: 1px solid #edf1f4; white-space: nowrap; }
  td { height: 58px; padding: 9px 16px; border-bottom: 1px solid #f0f3f5; color: #586a76; font-size: 11px; font-weight: 650; white-space: nowrap; }
  tr:last-child td { border-bottom: 0; }
  tbody tr:hover, tbody tr.emphasis { background: #fbfdfc; }
  td strong, td small { display: block; }
  td strong { max-width: 360px; overflow: hidden; color: #1c2932; font-size: 12px; text-overflow: ellipsis; }
  td small { max-width: 280px; margin-top: 5px; overflow: hidden; color: #9ba6ae; font-size: 9px; text-overflow: ellipsis; }
  .truncate-cell { max-width: 190px; overflow: hidden; text-overflow: ellipsis; }
  .source-cell { max-width: 240px; overflow: hidden; text-overflow: ellipsis; }
  .status { display: inline-flex; min-height: 24px; padding: 3px 8px; align-items: center; border-radius: 12px; background: #edf1f3; color: #687985; font-size: 9px; font-weight: 850; }
  .status.pending, .status.draft { background: #fff2d9; color: #8b5d11; }
  .status.approved, .status.published { background: #e1f1eb; color: #176b53; }
  .status.rejected, .status.archived { background: #f8e5e3; color: #943b37; }
  .status-select { min-width: 82px; height: 30px; padding: 0 24px 0 9px; border: 1px solid #dce5ec; border-radius: 6px; outline: 0; font-size: 10px; font-weight: 800; cursor: pointer; }
  .status-select.draft { border-color: #ead8ad; background: #fff8e8; color: #805a16; }
  .status-select.published { border-color: #bcdccf; background: #edf8f3; color: #176b53; }
  .status-select.archived { border-color: #e5c9c5; background: #fff3f1; color: #943b37; }
  .status-select:disabled { cursor: wait; opacity: .65; }
  .row-action { display: grid; width: 32px; height: 32px; padding: 0; place-items: center; border: 0; border-radius: 7px; background: transparent; color: #71818c; cursor: pointer; }
  .row-action:hover { background: #eaf1ee; color: #176b53; }
  .row-actions { display: flex; align-items: center; gap: 2px; }
  .row-action:disabled { cursor: not-allowed; opacity: .45; }
  .row-action.danger:hover { background: #fbeceb; color: #b3453f; }
  .empty-cell { height: 120px; color: #9aa7b1; text-align: center; }
  .result-count { margin: 0; padding: 11px 16px; border-top: 1px solid #edf1f4; color: #9aa7b1; font-size: 9px; font-weight: 700; }
  .homepage-banner { display: flex; min-height: 120px; padding: 20px; align-items: center; border-bottom: 1px solid #edf1f4; background: #fbfdfc; gap: 14px; }
  .homepage-banner > div { min-width: 0; flex: 1; } .homepage-banner small, .homepage-banner strong { display: block; } .homepage-banner small { color: #84928c; font-size: 9px; font-weight: 850; } .homepage-banner strong { margin-top: 4px; font-size: 28px; } .homepage-banner p { margin: 6px 0 0; color: #74837c; font-size: 10px; line-height: 1.6; }
  .drawer-layer, .mobile-nav-layer { position: fixed; z-index: 80; inset: 0; }
  .drawer-scrim, .mobile-scrim { position: absolute; inset: 0; width: 100%; border: 0; background: rgba(17,24,32,.38); backdrop-filter: blur(2px); }
  .drawer { position: absolute; inset: 0 0 0 auto; display: flex; width: min(620px, 100%); flex-direction: column; background: #f7fafb; box-shadow: -18px 0 55px rgba(17,24,32,.15); }
  .drawer-head { display: flex; min-height: 74px; padding: 15px 20px; align-items: center; justify-content: space-between; border-bottom: 1px solid #dce5ec; background: white; }
  .drawer-head small { color: #8d9ba5; font-size: 9px; font-weight: 850; }
  .drawer-head h2 { margin: 4px 0 0; font-size: 18px; }
  .drawer-head > button { display: grid; width: 36px; height: 36px; padding: 0; place-items: center; border: 1px solid #dce5ec; border-radius: 8px; background: white; color: #71818c; cursor: pointer; }
  .drawer-body { min-height: 0; padding: 22px; flex: 1; overflow: auto; }
  .submission-title { padding-bottom: 19px; border-bottom: 1px solid #dfe7ec; }
  .submission-title h3 { margin: 12px 0 7px; font-size: 20px; line-height: 1.4; }
  .submission-title p { margin: 0; color: #7c8c97; font-size: 11px; }
  .submission-meta { display: grid; margin: 18px 0; grid-template-columns: repeat(2, minmax(0, 1fr)); gap: 10px; }
  .submission-meta div { padding: 13px; border: 1px solid #dfe7ec; border-radius: 8px; background: white; }
  .submission-meta div:first-child { grid-column: 1 / -1; }
  .submission-meta dt { color: #8d9ba5; font-size: 9px; font-weight: 800; }
  .submission-meta dd { margin: 6px 0 0; color: #34434d; font-size: 11px; line-height: 1.6; }
  .markdown-preview > span { display: block; margin-bottom: 8px; color: #697b86; font-size: 10px; font-weight: 850; }
  .markdown-preview pre { max-height: 400px; margin: 0; overflow: auto; padding: 17px; border: 1px solid #dfe7ec; border-radius: 8px; background: white; color: #34434d; font: 12px/1.8 ui-monospace, SFMono-Regular, Consolas, monospace; white-space: pre-wrap; }
  .drawer-actions { display: flex; min-height: 72px; padding: 14px 20px; align-items: center; justify-content: flex-end; border-top: 1px solid #dce5ec; background: white; gap: 8px; }
  .action-spacer { flex: 1; }
  .danger-button { display: inline-flex; min-height: 40px; padding: 0 14px; align-items: center; border: 1px solid #e7c3bf; border-radius: 8px; background: #fff5f3; color: #a43d37; gap: 7px; font-size: 12px; font-weight: 850; cursor: pointer; white-space: nowrap; }
  .danger-button:hover { background: #fbe8e5; }
  .danger-button:disabled { cursor: not-allowed; opacity: .5; }
  .drawer-form { display: flex; min-height: 0; flex: 1; flex-direction: column; overflow: hidden; }
  .form-fields { display: grid; grid-template-columns: repeat(2, minmax(0, 1fr)); align-content: start; gap: 15px; }
  .form-fields > label, .markdown-field > label:first-child { display: grid; color: #536571; gap: 7px; font-size: 10px; font-weight: 850; }
  .form-fields .full { grid-column: 1 / -1; }
  .form-fields input:not(.file-input):not([type='checkbox']), .form-fields select, .form-fields textarea { width: 100%; min-height: 42px; padding: 9px 11px; border: 1px solid #d5e0e7; border-radius: 7px; outline: 0; background: white; color: #1c2932; font-size: 12px; }
  .form-fields .pin-field { display: flex; min-height: 42px; padding: 0 11px; align-items: center; border: 1px solid #d5e0e7; border-radius: 7px; background: white; cursor: pointer; }
  .pin-field input { width: 15px; height: 15px; margin: 0 8px 0 0; accent-color: #176b53; }
  .pin-field span, .pin-indicator { display: inline-flex; align-items: center; color: #176b53; gap: 5px; font-size: 10px; font-weight: 800; }
  .form-fields input:focus, .form-fields select:focus, .form-fields textarea:focus { border-color: #176b53; box-shadow: 0 0 0 3px rgba(23,107,83,.1); }
  .form-fields input:disabled { cursor: not-allowed; background: #edf2f4; color: #75848e; }
  .form-fields textarea { min-height: 330px; resize: vertical; font-family: ui-monospace, SFMono-Regular, Consolas, monospace; line-height: 1.7; }
  .markdown-field { position: relative; }
  .file-input { position: absolute; width: 1px; height: 1px; opacity: 0; }
  .upload-button { position: absolute; z-index: 1; top: -5px; right: 0; display: inline-flex; height: 30px; padding: 0 9px; align-items: center; border: 1px solid #d5e0e7; border-radius: 7px; background: white; color: #52636f; gap: 5px; font-size: 9px; cursor: pointer; }
  .mobile-nav-layer { display: none; }
  @media (max-width: 1120px) { .metric-grid { grid-template-columns: repeat(2, 1fr); } .overview-grid { grid-template-columns: 1fr; } }
  @media (max-width: 820px) {
    .desktop-sidebar { display: none; }
    .workspace, .workspace.wide { padding-left: 0; }
    .mobile-menu { display: grid; place-items: center; }
    .mobile-nav-layer { display: block; }
    .mobile-sidebar { display: flex; }
    .topbar { padding-inline: 16px; }
    .administrator { display: none; }
    .admin-content { padding: 23px 16px; }
  }
  @media (max-width: 600px) {
    .topbar-actions > a { display: none; }
    .view-heading { align-items: flex-start; flex-direction: column; }
    .metric-grid { grid-template-columns: 1fr; }
    .metric-grid > button { min-height: 130px; }
    .filters { align-items: stretch; flex-direction: column; }
    .filters label { width: 100%; }
    .filters select { width: 100%; margin-left: 0; }
    .form-fields { grid-template-columns: 1fr; }
    .form-fields .full { grid-column: auto; }
    .submission-meta { grid-template-columns: 1fr; }
    .submission-meta div:first-child { grid-column: auto; }
    .drawer-actions { flex-wrap: wrap; }
  }
  .admin-page { background: #f4f7fb; color: var(--ink); }
  .login-shell { background: #f4f7fb; }
  .login-panel { border-color: var(--line); border-radius: 8px; box-shadow: var(--shadow); }
  .login-mark { background: linear-gradient(135deg, var(--accent), var(--accent-secondary)); box-shadow: 0 7px 18px rgba(var(--accent-rgb),.2); }
  .login-input:focus-within { border-color: var(--accent); box-shadow: 0 0 0 3px rgba(var(--accent-rgb),.12); }
  .login-panel > button { background: linear-gradient(135deg, var(--accent), var(--accent-secondary)); box-shadow: 0 7px 18px rgba(var(--accent-rgb),.18); }
  .sidebar { border-color: var(--line); background: rgba(248,250,252,.97); }
  .sidebar nav button.active { background: #0f172a; box-shadow: 0 9px 22px rgba(15,23,42,.16); }
  .topbar { border-color: var(--line); background: rgba(244,247,251,.9); }
  .primary-action { background: linear-gradient(135deg, var(--accent), var(--accent-secondary)); box-shadow: 0 7px 18px rgba(var(--accent-rgb),.18); }
  .primary-action:hover { background: linear-gradient(135deg, var(--accent), var(--accent-secondary)); filter: brightness(1.06); }
  .panel, .metric-grid > button { border-color: var(--line); border-radius: 8px; box-shadow: var(--shadow-sm); }
  .metric-grid > button:hover { border-color: rgba(var(--accent-rgb),.35); box-shadow: var(--shadow); }
  .row-action:hover { background: var(--accent-soft); color: var(--accent); }
  .pin-field span, .pin-indicator { color: var(--accent); }
  .form-fields input:focus, .form-fields select:focus, .form-fields textarea:focus { border-color: var(--accent); box-shadow: 0 0 0 3px rgba(var(--accent-rgb),.12); }
  .status.approved, .status.published { background: var(--accent-soft); color: var(--accent-dark); }
  .status-select.published { border-color: rgba(var(--accent-rgb),.28); background: var(--accent-soft); color: var(--accent-dark); }
  @media (prefers-reduced-motion: reduce) { .sidebar, .workspace, .metric-grid > button { transition: none; } }
</style>
