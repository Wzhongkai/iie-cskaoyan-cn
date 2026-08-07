<script lang="ts">
  import { ArrowLeft, ArrowRight, BarChart3, BookOpen, Building2, GraduationCap, ListFilter, Table2 } from '@lucide/svelte';
  import type { EChartsCoreOption as EChartsOption } from 'echarts/core';
  import DataChart from '$lib/components/DataChart.svelte';
  import type { AnnualReportDetail, LabStat, ScoreBandStat, SchoolTierStat } from '$lib/types';

  let { data }: { data: { report: AnnualReportDetail } } = $props();
  const report = $derived(data.report);
  const overview = $derived(report.overview);
  let schoolTrack = $state<'recommendation' | 'exam'>('recommendation');
  let schoolTier = $state<'all' | '985' | '211' | 'non_211'>('all');
  let schoolQuery = $state('');
  let scoreProgram = $state<'combined' | 'academic' | 'professional'>('combined');
  let labProgram = $state<'combined' | 'academic' | 'professional'>('combined');
  const tierNames: Record<string, string> = { '985': '985（含国科大）', '211': '211（非 985）', non_211: '双非' };
  const programNames: Record<string, string> = { combined: '不区分学硕/专硕', academic: '学硕', professional: '专硕' };

  const visibleSchools = $derived(report.schools.filter((item) => item.track === schoolTrack && (schoolTier === 'all' || item.tier === schoolTier) && (!schoolQuery.trim() || item.school.toLowerCase().includes(schoolQuery.trim().toLowerCase()))));
  const visibleBands = $derived(report.score_bands.filter((item) => item.program === scoreProgram));
  const visibleLabs = $derived(labProgram === 'combined' && report.labs.filter((item) => item.program === 'combined').length === 0
    ? Array.from({ length: 11 }, (_, index) => {
        const academic = report.labs.find((item) => item.program === 'academic' && item.lab === index + 1);
        const professional = report.labs.find((item) => item.program === 'professional' && item.lab === index + 1);
        return {
          program: 'combined' as const,
          lab: index + 1,
          admitted: (academic?.admitted ?? 0) + (professional?.admitted ?? 0),
          rejected: (academic?.rejected ?? 0) + (professional?.rejected ?? 0),
          first_choice: (academic?.first_choice ?? 0) + (professional?.first_choice ?? 0),
          highest: null,
          lowest: null,
          average: null,
          median: null,
          note: [academic?.note, professional?.note].filter(Boolean).join('；') || null
        };
      })
    : report.labs.filter((item) => item.program === labProgram));
  const tierData = $derived(report.school_tiers.filter((item) => item.track === schoolTrack));
  const schoolTrackLabel = $derived(schoolTrack === 'recommendation' ? '推免' : '考研');
  const schoolTrackScope = $derived(schoolTrack === 'recommendation'
    ? `${overview.recommendation_total ?? '—'} 人，完整统计`
    : `${overview.exam_source_sample ?? '—'} 人样本，覆盖 ${overview.exam_source_coverage ?? '—'}%`);

  function pieOption(items: SchoolTierStat[]): EChartsOption {
    return { color: ['#176b53', '#316b91', '#d19a37'], tooltip: { trigger: 'item', formatter: '{b}<br/>{c} 人 · {d}%' }, legend: { bottom: 0, itemWidth: 10, itemHeight: 10, textStyle: { color: '#66706c', fontSize: 11 } }, series: [{ type: 'pie', radius: ['47%', '70%'], center: ['50%', '43%'], label: { formatter: '{d}%', fontSize: 11 }, itemStyle: { borderColor: '#fff', borderWidth: 3 }, data: items.map((item) => ({ name: tierNames[item.tier], value: item.admitted })) }] };
  }

  function scoreOption(items: ScoreBandStat[]): EChartsOption {
    return { color: ['#316b91', '#d19a37', '#176b53'], tooltip: { trigger: 'axis' }, legend: { top: 0, data: ['复试人数', '录取人数', '分段录取率'], textStyle: { color: '#66706c', fontSize: 11 } }, grid: { left: 46, right: 48, top: 44, bottom: 60 }, xAxis: { type: 'category', data: items.map((item) => item.band), axisLabel: { rotate: 35, color: '#66706c', fontSize: 10 } }, yAxis: [{ type: 'value', name: '人数', splitLine: { lineStyle: { color: '#edf1ee' } } }, { type: 'value', name: '录取率', min: 0, max: 100, axisLabel: { formatter: '{value}%' }, splitLine: { show: false } }], series: [{ name: '复试人数', type: 'bar', data: items.map((item) => item.interviewed), barMaxWidth: 24 }, { name: '录取人数', type: 'bar', data: items.map((item) => item.admitted), barMaxWidth: 24 }, { name: '分段录取率', type: 'line', yAxisIndex: 1, symbolSize: 6, data: items.map((item) => Number(((item.admitted / item.interviewed) * 100).toFixed(2))) }] };
  }

  function labOption(items: LabStat[]): EChartsOption {
    return { color: ['#316b91', '#d19a37', '#176b53'], tooltip: { trigger: 'axis' }, legend: { top: 0, data: ['拟录取', '未录取', '一志愿复试'], textStyle: { color: '#66706c', fontSize: 11 } }, grid: { left: 42, right: 20, top: 42, bottom: 38 }, xAxis: { type: 'category', data: items.map((item) => `${item.lab}室`), axisLabel: { color: '#66706c' } }, yAxis: { type: 'value', splitLine: { lineStyle: { color: '#edf1ee' } } }, series: [{ name: '拟录取', type: 'bar', data: items.map((item) => item.admitted), barMaxWidth: 20 }, { name: '未录取', type: 'bar', data: items.map((item) => item.rejected), barMaxWidth: 20 }, { name: '一志愿复试', type: 'line', data: items.map((item) => item.first_choice), symbolSize: 6 }] };
  }
</script>

<svelte:head><title>{overview.year} 年保研考研详细数据 | 信工所考研信息站</title><meta name="description" content={`${overview.year} 年信工所保研考研报告的院校、分数段、科室和科目明细。`} /></svelte:head>

<section class="page-heading detail-heading"><a class="back-link" href="/data"><ArrowLeft size={14} />返回年度档案</a><p class="eyebrow">{overview.year} 年年度报告</p><h1 class="page-title">{overview.title}</h1><p class="page-lead">{overview.source_note}。可按招生方式、院校层次、培养类型和科室查看对应数据。</p></section>

<section class="section" id="overview">
  <div class="section-head"><div><p class="eyebrow">01 / 总览</p><h2>招生规模与分数线</h2></div></div>
  <div class="overview-grid">
    <div class="overview-facts">
      <div><span>统考报考</span><strong>{overview.exam_applicants_min ?? '—'}+</strong><small>{overview.applicants_note}</small></div>
      <div><span>进复试</span><strong>{overview.interviewed_total ?? '—'}</strong><small>占报考人数的筛选结果</small></div>
      <div><span>统考录取</span><strong>{overview.admitted_total ?? '—'}</strong><small>学硕 {overview.academic_admitted} · 专硕 {overview.professional_admitted}</small></div>
      <div><span>推免录取</span><strong>{overview.recommendation_total ?? '—'}</strong><small>直博 {overview.direct_phd} · 硕士 {Number(overview.recommendation_academic ?? 0) + Number(overview.recommendation_professional ?? 0)}</small></div>
    </div>
    <div class="cutoff-box"><div><span>国家线 / 总分</span><strong>{overview.national_total_cutoff}</strong></div><div><span>学硕复试线</span><strong>{overview.academic_cutoff}</strong></div><div><span>专硕复试线</span><strong>{overview.professional_cutoff}</strong></div><p>{overview.score_formula}</p></div>
  </div>
</section>

<section class="section" id="schools">
  <div class="section-head"><div><p class="eyebrow">02 / 生源学校</p><h2>推免与考研生源院校明细</h2><p>{overview.year} 年推免 {overview.recommendation_total ?? '—'} 人、考研统计样本 {overview.exam_source_sample ?? '—'} 人。下面保留报告中的学校与人数，并提供 985、211、双非筛选。</p></div></div>
  <div class="switch-row"><div class="segmented"><button class:active={schoolTrack === 'recommendation'} onclick={() => (schoolTrack = 'recommendation')}><GraduationCap size={14} />推免 {overview.recommendation_total}</button><button class:active={schoolTrack === 'exam'} onclick={() => (schoolTrack = 'exam')}><BookOpen size={14} />考研 {overview.exam_source_sample} 样本</button></div><div class="school-tools"><select bind:value={schoolTier} aria-label="院校层次"><option value="all">全部层次</option><option value="985">985</option><option value="211">211（非 985）</option><option value="non_211">双非</option></select><label><ListFilter size={14} /><input bind:value={schoolQuery} placeholder="搜索学校" aria-label="搜索学校" /></label></div></div>
  <div class="tier-layout"><div class="tier-chart"><div class="tier-chart-head"><strong>{schoolTrackLabel}生源层次</strong><span>{schoolTrackScope}</span></div><DataChart option={pieOption(tierData)} height={280} label={`${schoolTrackLabel}生源层次饼图`} /></div><div class="tier-summary">{#each tierData as item}<div><strong>{item.admitted}</strong><span>{tierNames[item.tier]}</span><small>{item.percentage.toFixed(2)}%</small></div>{/each}<p>{schoolTrackLabel}口径：{schoolTrackScope}。报告中的学校名次按录取人数排序；同人数学校再按名称展示。</p></div></div>
  <div class="table-wrap school-table"><table><thead><tr><th>#</th><th>院校</th><th>层次</th><th>录取人数</th></tr></thead><tbody>{#each visibleSchools as item, index}<tr><td>{index + 1}</td><td><strong>{item.school}</strong></td><td><span class={`tier tier-${item.tier}`}>{tierNames[item.tier]}</span></td><td class="number">{item.admitted}</td></tr>{:else}<tr><td colspan="4" class="empty-cell">没有匹配的院校</td></tr>{/each}</tbody></table></div>
  <p class="table-caption">共 {visibleSchools.length} 所学校显示。完整院校明细来自报告原表；“国科大计入 985”的口径随报告保留。{#if overview.year === 2026 && schoolTrack === 'exam' && schoolTier === 'non_211'}报告的双非层次汇总为 55 人，但逐校明细相加为 54 人，本站同时保留两种原始数值并标注该差异。{/if}</p>
</section>

<section class="section" id="scores">
  <div class="section-head"><div><p class="eyebrow">03 / 分数段</p><h2>各分数段复试与录取</h2><p>折线为分段录取率，表格补充累计复试人数、累计录取人数和备注。</p></div><div class="segmented"><button class:active={scoreProgram === 'combined'} onclick={() => (scoreProgram = 'combined')}>合计</button><button class:active={scoreProgram === 'academic'} onclick={() => (scoreProgram = 'academic')}>学硕</button><button class:active={scoreProgram === 'professional'} onclick={() => (scoreProgram = 'professional')}>专硕</button></div></div>
  <div class="chart-frame"><div class="chart-label"><BarChart3 size={15} />{programNames[scoreProgram]}分数段</div><DataChart option={scoreOption(visibleBands)} height={390} label={`${overview.year} 年${programNames[scoreProgram]}分数段图`} /></div>
  <div class="table-wrap"><table><thead><tr><th>分数段</th><th>进复试</th><th>录取</th><th>分段录取率</th><th>累计复试</th><th>累计录取</th><th>累计录取率</th><th>备注</th></tr></thead><tbody>{#each visibleBands as item}<tr><td><strong>{item.band}</strong></td><td>{item.interviewed}</td><td class="number">{item.admitted}</td><td>{(item.admitted / item.interviewed * 100).toFixed(2)}%</td><td>{item.cumulative_interviewed}</td><td>{item.cumulative_admitted}</td><td>{(item.cumulative_admitted / item.cumulative_interviewed * 100).toFixed(2)}%</td><td>{item.note ?? '—'}</td></tr>{/each}</tbody></table></div>
</section>

<section class="section" id="labs">
  <div class="section-head"><div><p class="eyebrow">04 / 科室</p><h2>11 个科室的复试与录取</h2><p>一志愿复试人数、拟录取、未录取和科室录取分数全部保留；备注中的调入、调出和特殊计划来自报告。</p></div><div class="segmented"><button class:active={labProgram === 'combined'} onclick={() => (labProgram = 'combined')}>合计</button><button class:active={labProgram === 'academic'} onclick={() => (labProgram = 'academic')}>学硕</button><button class:active={labProgram === 'professional'} onclick={() => (labProgram = 'professional')}>专硕</button></div></div>
  {#if labProgram === 'combined'}<div class="lab-note">2026 报告按学硕、专硕分别列出科室，因此合计视图为页面按科室相加计算；原始两套数据仍可切换查看。</div>{/if}
  <div class="chart-frame"><div class="chart-label"><Building2 size={15} />{programNames[labProgram]}科室人数</div><DataChart option={labOption(visibleLabs)} height={360} label={`${overview.year} 年${programNames[labProgram]}科室录取图`} /></div>
  <div class="table-wrap"><table><thead><tr><th>科室</th><th>拟录取</th><th>未录取</th><th>一志愿复试</th><th>录取率</th><th>最高分</th><th>最低分</th><th>均分</th><th>中位数</th><th>备注</th></tr></thead><tbody>{#each visibleLabs as item}<tr><td><strong>{item.lab}室</strong></td><td class="number">{item.admitted}</td><td>{item.rejected}</td><td>{item.first_choice}</td><td>{(item.admitted / item.first_choice * 100).toFixed(2)}%</td><td>{item.highest ?? '—'}</td><td>{item.lowest ?? '—'}</td><td>{item.average ?? '—'}</td><td>{item.median ?? '—'}</td><td>{item.note ?? '—'}</td></tr>{/each}</tbody></table></div>
</section>

<section class="section" id="subjects">
  <div class="section-head"><div><p class="eyebrow">05 / 科目与总成绩</p><h2>初试单科、初复试与总成绩</h2><p>最高、最低、平均和中位数来自对应报告统计样本，避免只展示一个“平均分”。</p></div></div>
  <div class="subject-grid">{#each ['initial_subject', 'admitted_total'] as phase}<article class="subject-card"><h3>{phase === 'initial_subject' ? '初试单科成绩' : '拟录取初试 / 复试 / 总成绩'}</h3>{#each report.subjects.filter((item) => item.phase === phase) as item}<div class="subject-row"><span>{programNames[item.program] ?? item.program} · {item.subject}</span><strong>{item.average}</strong><small>最高 {item.highest} · 最低 {item.lowest} · 中位 {item.median}</small></div>{/each}</article>{/each}</div>
</section>

<section class="section source-section"><div class="source-card"><Table2 size={18} /><div><strong>来源与使用边界</strong><p>{overview.source_file} · {overview.source_note}。本站仅对资料进行整理和展示，不代表招生单位官方口径；政策、名额和复试线请以当年正式通知为准。</p></div><a class="button" href="/sources">资料说明 <ArrowRight size={15} /></a></div></section>

<style>
  .detail-heading { position: relative; padding-top: 24px; }
  .back-link { display: inline-flex; margin-bottom: 32px; align-items: center; color: var(--green); gap: 5px; font-size: 12px; font-weight: 750; }
  .overview-grid { display: grid; overflow: hidden; grid-template-columns: 1.3fr .7fr; border: 1px solid var(--line); border-radius: 8px; background: white; box-shadow: var(--shadow-sm); }
  .overview-facts { display: grid; grid-template-columns: repeat(2, 1fr); }
  .overview-facts div { min-height: 132px; padding: 20px; border-right: 1px solid var(--line); border-bottom: 1px solid var(--line); } .overview-facts div:nth-child(even) { border-right: 0; }
  .overview-facts span, .overview-facts strong, .overview-facts small { display: block; } .overview-facts span { color: var(--muted); font-size: 11px; font-weight: 750; } .overview-facts strong { margin-top: 8px; color: var(--green-dark); font-size: 31px; } .overview-facts small { margin-top: 7px; color: #7d8983; font-size: 10px; }
  .cutoff-box { display: grid; padding: 20px; align-content: center; grid-template-columns: repeat(3, 1fr); gap: 12px; } .cutoff-box div { padding: 13px 9px; border-left: 3px solid var(--accent); border-radius: 5px; background: var(--accent-soft); } .cutoff-box span, .cutoff-box strong { display: block; } .cutoff-box span { color: var(--muted); font-size: 10px; } .cutoff-box strong { margin-top: 8px; font-size: 26px; } .cutoff-box p { grid-column: 1 / -1; margin: 5px 0 0; color: var(--muted); font-size: 11px; line-height: 1.7; }
  .switch-row { display: flex; align-items: center; justify-content: space-between; gap: 15px; } .segmented { display: flex; flex-wrap: wrap; gap: 4px; } .segmented button { display: inline-flex; min-height: 36px; padding: 0 12px; align-items: center; border: 1px solid var(--line); border-radius: 7px; background: white; color: var(--muted); cursor: pointer; gap: 6px; font-size: 11px; font-weight: 750; transition: background .2s ease, border-color .2s ease, color .2s ease; } .segmented button.active { border-color: var(--accent); background: var(--accent); color: white; }
  .school-tools { display: flex; align-items: center; gap: 7px; } .school-tools select, .school-tools label { min-height: 34px; padding: 0 10px; border: 1px solid var(--line); border-radius: 4px; background: white; color: var(--muted); font-size: 11px; } .school-tools label { display: flex; align-items: center; gap: 6px; } .school-tools input { width: 120px; border: 0; outline: 0; color: var(--ink); font-size: 11px; }
  .tier-layout { display: grid; margin: 18px 0 22px; overflow: hidden; grid-template-columns: 1fr 1fr; border: 1px solid var(--line); border-radius: 8px; background: white; box-shadow: var(--shadow-sm); } .tier-chart { border-right: 1px solid var(--line); } .tier-chart-head { display: flex; padding: 16px 18px 0; align-items: center; justify-content: space-between; gap: 12px; } .tier-chart-head strong { color: var(--ink); font-size: 12px; } .tier-chart-head span { color: var(--muted); font-size: 10px; font-weight: 700; text-align: right; } .tier-summary { display: grid; padding: 22px; align-content: center; grid-template-columns: repeat(3, 1fr); gap: 9px; } .tier-summary div { padding: 12px; border-radius: 5px; background: var(--soft); } .tier-summary strong, .tier-summary span, .tier-summary small { display: block; } .tier-summary strong { color: var(--accent-dark); font-size: 25px; } .tier-summary span { margin-top: 5px; color: var(--muted); font-size: 10px; } .tier-summary small { margin-top: 3px; color: #7c8798; font-size: 10px; } .tier-summary p { grid-column: 1 / -1; margin: 3px 0 0; color: var(--muted); font-size: 10px; line-height: 1.6; }
  .table-wrap { margin-top: 17px; } .school-table { max-height: 580px; overflow: auto; } .school-table thead { position: sticky; top: 0; z-index: 2; } td strong { color: var(--ink); } td.number { color: var(--green-dark); font-weight: 800; } .tier { display: inline-flex; padding: 4px 7px; border-radius: 3px; font-size: 10px; font-weight: 750; } .tier-985 { background: #e9f3ee; color: var(--green-dark); } .tier-211 { background: #e9eff5; color: #315d7b; } .tier-non_211 { background: #fbf1dc; color: #8e651e; } .table-caption { margin: 10px 0 0; color: var(--muted); font-size: 10px; }
  .chart-frame { margin-bottom: 17px; overflow: hidden; border: 1px solid var(--line); border-radius: 8px; background: white; box-shadow: var(--shadow-sm); } .chart-label { display: flex; padding: 15px 17px 0; align-items: center; color: var(--accent-dark); gap: 7px; font-size: 12px; font-weight: 800; } .lab-note { margin-bottom: 12px; padding: 11px 13px; border-left: 3px solid var(--amber); background: #fff8e9; color: #6b542d; font-size: 11px; }
  .subject-grid { display: grid; grid-template-columns: 1fr 1fr; gap: 12px; } .subject-card { overflow: hidden; border: 1px solid var(--line); border-radius: 8px; background: white; box-shadow: var(--shadow-sm); } .subject-card h3 { margin: 0; padding: 15px; border-bottom: 1px solid var(--line); font-size: 14px; } .subject-row { display: grid; padding: 12px 15px; grid-template-columns: 1fr auto; border-bottom: 1px solid #edf1ee; gap: 4px 12px; } .subject-row span { color: var(--muted); font-size: 11px; } .subject-row strong { color: var(--accent-dark); font-size: 16px; } .subject-row small { grid-column: 1 / -1; color: #7c8798; font-size: 10px; }
  .source-section { padding-top: 20px; } .source-card { display: flex; padding: 18px; align-items: flex-start; border: 1px solid var(--line); border-radius: 8px; background: var(--accent-soft); color: var(--accent); gap: 12px; } .source-card strong { color: var(--ink); font-size: 13px; } .source-card p { max-width: 700px; margin: 5px 0 0; color: var(--muted); font-size: 11px; line-height: 1.7; } .source-card a { margin-left: auto; flex: none; }
  @media (max-width: 840px) { .overview-grid, .tier-layout, .subject-grid { grid-template-columns: 1fr; } .tier-chart { border-right: 0; border-bottom: 1px solid var(--line); } .cutoff-box { padding-top: 0; } .switch-row { align-items: flex-start; flex-direction: column; } .school-tools { width: 100%; } .school-tools label { flex: 1; } .school-tools input { width: 100%; } }
  @media (max-width: 560px) { .overview-facts { grid-template-columns: 1fr; } .overview-facts div { border-right: 0; } .cutoff-box { grid-template-columns: 1fr; } .cutoff-box p { grid-column: auto; } .tier-summary { grid-template-columns: 1fr; } .source-card { flex-wrap: wrap; } .source-card a { margin-left: 0; } }
</style>
