<script lang="ts">
  import { ArrowRight, BarChart3, BookOpen, CheckCircle2, FilePlus2, Pin, Search, ShieldCheck, Users } from '@lucide/svelte';
  import type { EChartsCoreOption as EChartsOption } from 'echarts/core';
  import DataChart from '$lib/components/DataChart.svelte';
  import type { AnnualReportDetail, AnnualReportOverview, Article, Category, ScoreBandStat, SchoolTierStat } from '$lib/types';

  let { data }: { data: { report: AnnualReportDetail | null; reports: AnnualReportOverview[]; recent: Article[]; categories: Category[] } } = $props();
  const report = $derived(data.report);
  const overview = $derived(report?.overview);
  const recommendationTiers = $derived(report?.school_tiers.filter((item) => item.track === 'recommendation') ?? []);
  const examTiers = $derived(report?.school_tiers.filter((item) => item.track === 'exam') ?? []);
  const combinedBands = $derived(report?.score_bands.filter((item) => item.program === 'combined') ?? []);
  const topRecommendationSchools = $derived(report?.schools.filter((item) => item.track === 'recommendation' && item.tier === '985').slice(0, 8) ?? []);
  const topExamSchools = $derived(report?.schools.filter((item) => item.track === 'exam' && item.tier === '985').slice(0, 8) ?? []);

  const tierNames: Record<string, string> = { '985': '985（含国科大）', '211': '211（非 985）', non_211: '双非' };
  const categoryDescriptions: Record<string, string> = { initial: '科目节奏、资料选择和复盘方法。', reexam: '项目表达、专业问答和时间线。', career: '方向、城市、样本和统计口径。' };

  function pieOption(items: SchoolTierStat[]): EChartsOption {
    return {
      color: ['#0052ff', '#4d7cff', '#94a3b8'],
      tooltip: { trigger: 'item', formatter: '{b}<br/>{c} 人 · {d}%' },
      legend: { bottom: 0, itemWidth: 10, itemHeight: 10, textStyle: { color: '#66706c', fontSize: 11 } },
      series: [{ type: 'pie', radius: ['48%', '72%'], center: ['50%', '43%'], avoidLabelOverlap: true, itemStyle: { borderColor: '#fff', borderWidth: 3 }, label: { formatter: '{d}%', color: '#394440', fontSize: 11 }, data: items.map((item) => ({ name: tierNames[item.tier], value: item.admitted })) }]
    };
  }

  function scoreOption(items: ScoreBandStat[]): EChartsOption {
    return {
      color: ['#4d7cff', '#94a3b8', '#0052ff'],
      tooltip: { trigger: 'axis' },
      legend: { top: 0, data: ['复试人数', '录取人数', '分段录取率'], textStyle: { color: '#66706c', fontSize: 11 } },
      grid: { left: 42, right: 48, top: 42, bottom: 50 },
      xAxis: { type: 'category', data: items.map((item) => item.band), axisLabel: { rotate: 35, color: '#64748b', fontSize: 10 }, axisLine: { lineStyle: { color: '#e2e8f0' } } },
      yAxis: [{ type: 'value', name: '人数', splitLine: { lineStyle: { color: '#eef2f7' } } }, { type: 'value', name: '录取率', min: 30, max: 100, axisLabel: { formatter: '{value}%' }, splitLine: { show: false } }],
      series: [
        { name: '复试人数', type: 'bar', data: items.map((item) => item.interviewed), barMaxWidth: 24 },
        { name: '录取人数', type: 'bar', data: items.map((item) => item.admitted), barMaxWidth: 24 },
        { name: '分段录取率', type: 'line', yAxisIndex: 1, smooth: false, symbolSize: 6, data: items.map((item) => Number(((item.admitted / item.interviewed) * 100).toFixed(2))) }
      ]
    };
  }
</script>

<svelte:head>
  <title>信工所考研信息站 | 2026 保研考研数据</title>
  <meta name="description" content="2026 年中国科学院信息工程研究所推免、统考、生源学校、分数段与科室数据，以及动态经验文章。" />
</svelte:head>

{#if report && overview}
  <section class="home-hero section">
    <div class="hero-copy">
      <p class="eyebrow">{overview.year} 年度报告 · 学生公益维护</p>
      <h1>信工所保研与考研数据全景</h1>
      <p class="hero-lead">从报考、复试到录取，再拆到生源层次、分数段和 11 个科室。所有图表都对应可核对的数据表，不以单个分数替代完整判断。</p>
      <div class="hero-actions">
        <a class="button primary" href={`/data/${overview.year}`}><BarChart3 size={16} />打开 {overview.year} 完整报告</a>
        <a class="button" href="/articles"><Search size={16} />浏览经验文章</a>
      </div>
      <div class="trust-line"><ShieldCheck size={15} />{overview.source_note} · 数据完整口径见年度详情</div>
    </div>

    <div class="hero-ledger">
      <div class="ledger-head"><span>{overview.year} 数据快照</span><strong>REPORT / {overview.year}</strong></div>
      <div class="ledger-row"><span>统考报考</span><strong>约 {overview.exam_applicants_min ?? '—'} 人</strong><small>{overview.applicants_note}</small></div>
      <div class="ledger-row"><span>进入复试</span><strong>{overview.interviewed_total ?? '—'} 人</strong><small>含报告注明的专项计划</small></div>
      <div class="ledger-row"><span>统考录取</span><strong>{overview.admitted_total ?? '—'} 人</strong><small>学硕 {overview.academic_admitted} · 专硕 {overview.professional_admitted}</small></div>
      <div class="ledger-row accent"><span>推免录取</span><strong>{overview.recommendation_total ?? '—'} 人</strong><small>直博 {overview.direct_phd} · 学硕 {overview.recommendation_academic} · 专硕 {overview.recommendation_professional}</small></div>
    </div>
  </section>

  <section class="section compact-section">
    <div class="section-head"><div><p class="eyebrow">核心门槛</p><h2>国家线、复试线和录取规模放在一起看</h2></div><a class="text-link" href={`/data/${overview.year}#overview`}>查看口径 <ArrowRight size={15} /></a></div>
    <div class="metric-strip">
      <div><span>国家线</span><strong>{overview.national_total_cutoff}</strong><small>政治/英语 {overview.national_politics_english_cutoff} · 专业课 {overview.national_subject_cutoff}</small></div>
      <div><span>学硕复试线</span><strong>{overview.academic_cutoff}</strong><small>单科线 40 / 70</small></div>
      <div><span>专硕复试线</span><strong>{overview.professional_cutoff}</strong><small>单科线 40 / 60</small></div>
      <div><span>复试录取率</span><strong>{overview.interviewed_total && overview.admitted_total ? ((overview.admitted_total / overview.interviewed_total) * 100).toFixed(2) : '—'}%</strong><small>{overview.interviewed_total} 进复试 / {overview.admitted_total} 录取</small></div>
    </div>
  </section>

  <section class="section three-year-section">
    <div class="section-head"><div><p class="eyebrow">近三年概览</p><h2>同一组指标，按年度并列</h2><p>只做规模与门槛对照；每年的完整口径进入独立年度页面。</p></div><a class="button" href="/data">历年数据档案 <ArrowRight size={15} /></a></div>
    <div class="year-overview">
      {#each data.reports as item, index}
        <a class:current={index === 0} href={`/data/${item.year}`}>
          <div class="year-title"><strong>{item.year}</strong><span>{index === 0 ? '主页年度' : '历史归档'}</span></div>
          <dl><div><dt>报考</dt><dd>{item.exam_applicants_min ?? '—'}+</dd></div><div><dt>进复试</dt><dd>{item.interviewed_total ?? '—'}</dd></div><div><dt>统考录取</dt><dd>{item.admitted_total ?? '—'}</dd></div><div><dt>推免</dt><dd>{item.recommendation_total ?? '—'}</dd></div></dl>
          <p>学硕线 {item.academic_cutoff ?? '—'} · 专硕线 {item.professional_cutoff ?? '—'} <ArrowRight size={13} /></p>
        </a>
      {/each}
    </div>
  </section>

  <section class="section">
    <div class="section-head"><div><p class="eyebrow">生源结构</p><h2>推免与考研不是同一套分布</h2><p>“双非是否友好”需要分别看两种招生路径。考研样本覆盖 {overview.exam_source_coverage}%（{overview.exam_source_sample}/{overview.admitted_total}）。</p></div><a class="button" href={`/data/${overview.year}#schools`}>全部院校明细 <ArrowRight size={15} /></a></div>
    <div class="chart-grid">
      <article class="chart-panel"><div class="panel-title"><span>推免生源层次</span><strong>{overview.recommendation_total} 人 · 完整统计</strong></div><DataChart option={pieOption(recommendationTiers)} height={290} label="推免录取生源层次饼图" /></article>
      <article class="chart-panel"><div class="panel-title"><span>考研生源层次</span><strong>{overview.exam_source_sample} 人样本</strong></div><DataChart option={pieOption(examTiers)} height={290} label="考研录取生源层次饼图" /></article>
    </div>
    <div class="school-preview">
      <div><h3>推免主要 985 生源院校（含国科大）</h3>{#each topRecommendationSchools as item, index}<a href={`/data/${overview.year}#schools`}><b>{String(index + 1).padStart(2, '0')}</b><span>{item.school}</span><strong>{item.admitted} 人</strong></a>{/each}</div>
      <div><h3>考研主要 985 生源院校（含国科大）</h3>{#each topExamSchools as item, index}<a href={`/data/${overview.year}#schools`}><b>{String(index + 1).padStart(2, '0')}</b><span>{item.school}</span><strong>{item.admitted} 人</strong></a>{/each}</div>
    </div>
  </section>

  <section class="section">
    <div class="section-head"><div><p class="eyebrow">分数段</p><h2>高分更稳，但复试仍在改变结果</h2><p>柱形表示每个分数段的复试和录取人数，折线表示分段录取率。详情页另列累计录取率和学硕、专硕拆分。</p></div><a class="button" href={`/data/${overview.year}#scores`}>学硕 / 专硕拆分 <ArrowRight size={15} /></a></div>
    <article class="wide-chart"><DataChart option={scoreOption(combinedBands)} height={390} label="2026 考研各分数段复试及录取统计" /></article>
  </section>
{:else}
  <section class="missing-report section"><BarChart3 size={28} /><h1>年度报告数据暂时不可用</h1><p>页面暂时未能载入年度数据，请稍后刷新重试。</p></section>
{/if}

<section class="section">
  <div class="section-head"><div><p class="eyebrow">经验分类</p><h2>数据之外，还要看真实经历</h2></div><a class="button" href="/contribute"><FilePlus2 size={15} />提交内容</a></div>
  <div class="workflow-grid">
    {#each data.categories.slice(0, 3) as category, index}
      <a class="workflow-item" href={`/articles?category=${category.slug}`}><span>{String(index + 1).padStart(2, '0')}</span><div><h3>{category.name}</h3><p>{categoryDescriptions[category.slug] ?? '浏览该分类下的经验、资料与讨论。'}</p></div><ArrowRight size={17} /></a>
    {/each}
  </div>
</section>

<section class="section latest-section">
  <div class="section-head"><div><p class="eyebrow">最近更新</p><h2>文章与公告</h2></div><a class="text-link" href="/articles">查看全部 <ArrowRight size={15} /></a></div>
  {#if data.recent.length}
    <div class="article-list">
      {#each data.recent as article}
        <a class="article-row" href={`/articles/${article.slug}`}><span class="article-meta">{#if article.is_pinned}<Pin size={12} />置顶 · {/if}{article.year ?? '资料'}</span><span><strong>{article.title}</strong><small>{article.excerpt ?? '查看文章详情'}</small></span><ArrowRight size={17} /></a>
      {/each}
    </div>
  {:else}
    <div class="empty-state surface"><BookOpen size={20} /><span>暂时还没有公开内容。</span></div>
  {/if}
</section>

<style>
  .home-hero { display: grid; min-height: 560px; padding: 84px 0; grid-template-columns: minmax(0, 1.08fr) minmax(360px, .92fr); align-items: center; gap: clamp(42px, 7vw, 96px); }
  h1 { max-width: 690px; margin: 0; font-size: 54px; line-height: 1.12; }
  .hero-lead { max-width: 650px; margin: 22px 0 30px; color: var(--muted); font-size: 16px; line-height: 1.85; }
  .hero-actions { display: flex; flex-wrap: wrap; gap: 10px; }
  .trust-line { display: flex; margin-top: 22px; align-items: center; color: var(--accent); gap: 7px; font-size: 11px; font-weight: 700; }
  .hero-ledger { overflow: hidden; border: 1px solid var(--line); border-radius: 10px; background: white; box-shadow: var(--shadow); }
  .ledger-head, .ledger-row { display: grid; padding: 15px 20px; grid-template-columns: 1fr auto; gap: 4px 18px; }
  .ledger-head { background: #0f172a; color: #cbd5e1; font-size: 10px; font-weight: 800; }
  .ledger-head strong { color: white; }
  .ledger-row { min-height: 82px; align-content: center; border-top: 1px solid var(--line); }
  .ledger-row span { color: var(--muted); font-size: 12px; font-weight: 750; }
  .ledger-row strong { font-size: 24px; }
  .ledger-row small { grid-column: 1 / -1; color: #7c8798; font-size: 10px; }
  .ledger-row.accent { border-left: 4px solid var(--accent); background: #f8faff; }
  .compact-section { padding-top: 48px; padding-bottom: 52px; }
  .three-year-section { padding-top: 60px; }
  .year-overview { display: grid; grid-template-columns: repeat(3, 1fr); gap: 12px; }
  .year-overview > a { min-width: 0; padding: 20px; border: 1px solid var(--line); border-top: 3px solid transparent; border-radius: 8px; background: white; box-shadow: var(--shadow-sm); transition: border-color .2s ease, box-shadow .2s ease, transform .2s ease; }
  .year-overview > a.current { border-top-color: var(--accent); box-shadow: 0 10px 24px rgba(var(--accent-rgb), .1); }
  .year-overview > a:hover { border-color: rgba(var(--accent-rgb), .38); box-shadow: var(--shadow); transform: translateY(-2px); }
  .year-title { display: flex; align-items: center; justify-content: space-between; }
  .year-title strong { font-size: 24px; } .year-title span { color: var(--muted); font-size: 10px; font-weight: 750; }
  .year-overview dl { display: grid; margin: 18px 0; grid-template-columns: repeat(2, 1fr); gap: 10px; }
  .year-overview dl div { padding: 9px; border-radius: 5px; background: var(--soft); } .year-overview dt { color: var(--muted); font-size: 9px; } .year-overview dd { margin: 5px 0 0; color: var(--accent-dark); font-size: 16px; font-weight: 800; }
  .year-overview p { display: flex; margin: 0; align-items: center; justify-content: space-between; color: var(--muted); font-size: 10px; }
  .metric-strip { display: grid; grid-template-columns: repeat(4, 1fr); gap: 12px; }
  .metric-strip div { min-height: 132px; padding: 21px; border: 1px solid var(--line); border-radius: 8px; background: white; box-shadow: var(--shadow-sm); transition: transform .2s ease, box-shadow .2s ease; }
  .metric-strip div:hover { box-shadow: var(--shadow); transform: translateY(-2px); }
  .metric-strip span, .metric-strip strong, .metric-strip small { display: block; }
  .metric-strip span { color: var(--muted); font-size: 11px; font-weight: 750; }
  .metric-strip strong { margin-top: 8px; color: var(--accent-dark); font-size: 31px; }
  .metric-strip small { margin-top: 7px; color: #7c8798; font-size: 10px; line-height: 1.5; }
  .chart-grid { display: grid; grid-template-columns: repeat(2, minmax(0, 1fr)); gap: 12px; }
  .chart-panel { overflow: hidden; border: 1px solid var(--line); border-radius: 8px; background: white; box-shadow: var(--shadow-sm); }
  .panel-title { display: flex; padding: 16px 18px 0; justify-content: space-between; color: var(--ink); font-size: 13px; font-weight: 800; }
  .panel-title strong { color: var(--muted); font-size: 10px; }
  .school-preview { display: grid; margin-top: 14px; grid-template-columns: 1fr 1fr; gap: 12px; }
  .school-preview > div { overflow: hidden; border: 1px solid var(--line); border-radius: 8px; background: white; box-shadow: var(--shadow-sm); }
  .school-preview h3 { margin: 0; padding: 15px 14px; border-bottom: 1px solid var(--line); font-size: 13px; }
  .school-preview a { display: grid; min-height: 42px; padding: 8px 14px; grid-template-columns: 28px 1fr auto; align-items: center; border-bottom: 1px solid #eef2f7; gap: 8px; font-size: 12px; transition: color .2s ease, background .2s ease; }
  .school-preview a:hover { background: #f8faff; color: var(--accent); }
  .school-preview b { color: #a1aaa6; font-family: ui-monospace, monospace; font-size: 10px; }
  .school-preview strong { color: var(--accent-dark); font-size: 11px; }
  .wide-chart { overflow: hidden; border: 1px solid var(--line); border-radius: 8px; background: white; box-shadow: var(--shadow-sm); }
  .workflow-grid { display: grid; grid-template-columns: repeat(3, 1fr); gap: 10px; }
  .workflow-item { display: grid; min-height: 126px; padding: 20px; grid-template-columns: auto 1fr auto; border: 1px solid var(--line); border-top: 3px solid var(--accent); border-radius: 8px; background: white; box-shadow: var(--shadow-sm); gap: 14px; transition: border-color .2s ease, box-shadow .2s ease, transform .2s ease; }
  .workflow-item:hover { border-color: rgba(var(--accent-rgb), .4); box-shadow: var(--shadow); transform: translateY(-2px); }
  .workflow-item > span { color: var(--accent); font-family: ui-monospace, monospace; font-size: 10px; font-weight: 800; }
  .workflow-item h3 { margin: 0 0 7px; font-size: 16px; } .workflow-item p { margin: 0; color: var(--muted); font-size: 12px; line-height: 1.65; }
  .article-list { border-top: 1px solid var(--line); }
  .article-row { display: grid; min-height: 76px; padding: 14px 12px; grid-template-columns: 120px 1fr auto; align-items: center; border-bottom: 1px solid var(--line); border-radius: 6px; gap: 18px; transition: background .2s ease, color .2s ease; }
  .article-row:hover { background: #f8faff; color: var(--accent); }
  .article-meta { display: flex; align-items: center; color: var(--accent); gap: 4px; font-size: 11px; font-weight: 800; }
  .article-row strong, .article-row small { display: block; } .article-row small { margin-top: 4px; color: var(--muted); font-size: 11px; }
  .text-link { display: inline-flex; align-items: center; color: var(--accent); gap: 5px; font-size: 13px; font-weight: 750; }
  .empty-state, .missing-report { display: flex; min-height: 190px; align-items: center; justify-content: center; flex-direction: column; color: var(--muted); gap: 8px; text-align: center; }
  .missing-report h1 { color: var(--ink); font-size: 28px; }
  .missing-report p { margin: 0; }
  @media (max-width: 900px) { .home-hero { padding: 64px 0; grid-template-columns: 1fr; gap: 36px; } .metric-strip { grid-template-columns: repeat(2, 1fr); } }
  @media (max-width: 700px) { h1 { font-size: 38px; } .chart-grid, .school-preview, .workflow-grid, .year-overview { grid-template-columns: 1fr; } .article-row { grid-template-columns: 1fr auto; } .article-meta { grid-column: 1 / -1; } }
  @media (max-width: 480px) { .home-hero { padding: 48px 0; } .metric-strip { grid-template-columns: 1fr; } .hero-actions .button { width: 100%; } .hero-ledger { min-width: 0; } }
</style>
