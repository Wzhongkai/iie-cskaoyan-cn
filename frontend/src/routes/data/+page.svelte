<script lang="ts">
  import { AlertTriangle, ArrowRight, BarChart3, CalendarRange, Database } from '@lucide/svelte';
  import type { EChartsCoreOption as EChartsOption } from 'echarts/core';
  import DataChart from '$lib/components/DataChart.svelte';
  import type { AnnualReportOverview } from '$lib/types';

  let { data }: { data: { reports: AnnualReportOverview[] } } = $props();
  const chronological = $derived([...data.reports].sort((a, b) => a.year - b.year));
  const trendOption = $derived<EChartsOption>({
    color: ['#316b91', '#176b53', '#b3423e'],
    tooltip: { trigger: 'axis' },
    legend: { top: 0, data: ['进入复试', '统考录取', '推免录取'], textStyle: { color: '#66706c', fontSize: 11 } },
    grid: { left: 45, right: 20, top: 46, bottom: 35 },
    xAxis: { type: 'category', data: chronological.map((item) => item.year), axisLine: { lineStyle: { color: '#d9dfdb' } } },
    yAxis: { type: 'value', splitLine: { lineStyle: { color: '#edf1ee' } } },
    series: [
      { name: '进入复试', type: 'line', symbolSize: 7, data: chronological.map((item) => item.interviewed_total) },
      { name: '统考录取', type: 'line', symbolSize: 7, data: chronological.map((item) => item.admitted_total) },
      { name: '推免录取', type: 'line', symbolSize: 7, data: chronological.map((item) => item.recommendation_total) }
    ]
  });
</script>

<svelte:head><title>历年招生数据 | 信工所考研信息站</title><meta name="description" content="信工所 2024 至 2026 年保研考研报告归档与历年趋势。" /></svelte:head>

<section class="page-heading"><p class="eyebrow">年度数据档案</p><h1 class="page-title">保研与考研报告库</h1><p class="page-lead">首页只展示最新年度。这里保留每一年的原始口径、分数段、科室、科目和生源结构，避免把不同年份的规则混在一起。</p></section>

<section class="section archive-section">
  <div class="notice"><AlertTriangle size={17} /><span>以下均为当届同学整理的非官方数据。报考人数中的“约”“超过”和样本覆盖率按原报告保留，不能直接当作官方招生承诺。</span></div>
  {#if data.reports.length}
    <div class="section-head trend-head"><div><p class="eyebrow">三年变化</p><h2>复试、统考录取与推免规模</h2><p>趋势用于理解规模，不代表下一年度名额。</p></div></div>
    <div class="trend-chart"><DataChart option={trendOption} height={350} label="历年复试、统考录取和推免人数折线图" /></div>

    <div class="archive-list">
      {#each data.reports as report, index}
        <article class:latest={index === 0}>
          <div class="year-block"><CalendarRange size={18} /><strong>{report.year}</strong><span>{index === 0 ? '最新报告' : '历史归档'}</span></div>
          <div class="report-copy"><h2>{report.title}</h2><p>{report.source_note}</p><div class="inline-stats"><span>报考 {report.exam_applicants_min ?? '—'}+</span><span>复试 {report.interviewed_total ?? '—'}</span><span>统考录取 {report.admitted_total ?? '—'}</span><span>推免 {report.recommendation_total ?? '—'}</span></div></div>
          <a class="button" href={`/data/${report.year}`}>完整数据 <ArrowRight size={15} /></a>
        </article>
      {/each}
    </div>
  {:else}
    <div class="empty surface"><Database size={24} /><strong>年度数据暂时不可用</strong><span>年度数据暂时无法加载，请稍后刷新重试。</span></div>
  {/if}
</section>

<style>
  .archive-section { padding-top: 30px; }
  .notice { display: flex; align-items: flex-start; gap: 10px; } :global(.notice > svg) { flex: none; color: var(--amber); }
  .trend-head { margin-top: 42px; }
  .trend-chart { overflow: hidden; border: 1px solid var(--line); border-radius: 8px; background: white; box-shadow: var(--shadow-sm); }
  .archive-list { margin-top: 34px; border-top: 1px solid var(--line); }
  .archive-list article { display: grid; min-height: 154px; padding: 22px 16px; grid-template-columns: 130px 1fr auto; align-items: center; border-bottom: 1px solid var(--line); border-radius: 8px; gap: 28px; transition: background .2s ease, transform .2s ease; }
  .archive-list article:hover { background: #f8faff; transform: translateX(2px); }
  .archive-list article.latest { border-left: 4px solid var(--accent); padding-left: 18px; background: white; }
  .year-block { display: grid; grid-template-columns: auto 1fr; align-items: center; color: var(--green); gap: 4px 8px; }
  .year-block strong { color: var(--ink); font-size: 28px; }
  .year-block span { grid-column: 1 / -1; color: var(--muted); font-size: 10px; font-weight: 750; }
  .report-copy h2 { margin: 0; font-size: 17px; } .report-copy p { margin: 7px 0 12px; color: var(--muted); font-size: 11px; }
  .inline-stats { display: flex; flex-wrap: wrap; gap: 6px; } .inline-stats span { padding: 5px 8px; border: 1px solid var(--line); border-radius: 5px; background: white; color: #475569; font-size: 10px; font-weight: 700; }
  .empty { display: flex; min-height: 220px; margin-top: 30px; align-items: center; justify-content: center; flex-direction: column; color: var(--muted); gap: 8px; }
  .empty strong { color: var(--ink); }
  @media (max-width: 720px) { .archive-list article { grid-template-columns: 1fr; align-items: start; gap: 14px; } .archive-list article > a { justify-self: start; } }
</style>
