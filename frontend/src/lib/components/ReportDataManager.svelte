<script lang="ts">
  import { onMount } from 'svelte';
  import { FilePenLine, Plus, RefreshCw, Save, Trash2, X } from '@lucide/svelte';
  import type { AnnualReportDetail, AnnualReportOverview, LabStat, SchoolStat, SchoolTierStat, ScoreBandStat, SubjectStat } from '$lib/types';

  type Dataset = 'school_tiers' | 'schools' | 'subjects' | 'score_bands' | 'labs';
  type DataItem = SchoolTierStat | SchoolStat | SubjectStat | ScoreBandStat | LabStat;
  type Draft = Record<string, string>;

  let { token, reports, onEditReport, onNewReport }: { token: string; reports: AnnualReportOverview[]; onEditReport: (report: AnnualReportOverview) => void; onNewReport: () => void } = $props();
  let selectedYear = $state(new Date().getFullYear());
  let dataset = $state<Dataset>('schools');
  let detail = $state<AnnualReportDetail | null>(null);
  let loading = $state(false);
  let saving = $state(false);
  let errorMessage = $state('');
  let notice = $state('');
  let drawerOpen = $state(false);
  let editing = $state(false);
  let editingItem = $state<DataItem | null>(null);
  let draft = $state<Draft>({});

  const datasetLabels: Record<Dataset, string> = { school_tiers: '生源层次', schools: '生源院校', subjects: '科目统计', score_bands: '分数段', labs: '科室数据' };
  const endpoint: Record<Dataset, string> = { school_tiers: 'report-school-tiers', schools: 'report-schools', subjects: 'report-subjects', score_bands: 'report-score-bands', labs: 'report-labs' };
  const currentReport = $derived(reports.find((item) => item.year === selectedYear) ?? null);

  const text = (value: unknown) => value == null ? '' : String(value);
  const optionalNumber = (value: string) => value.trim() === '' ? null : Number(value);
  const programLabel = (value: string) => ({ academic: '学硕', professional: '专硕', combined: '合计' })[value] ?? value;
  const trackLabel = (value: string) => value === 'recommendation' ? '推免' : '统考';
  const tierLabel = (value: string) => value === 'non_211' ? '双非' : value;

  async function loadDetail() {
    loading = true; errorMessage = '';
    try {
      const response = await fetch(`/api/v1/reports/${selectedYear}`);
      const payload = await response.json();
      if (!response.ok) throw new Error(payload.error || '年度数据读取失败');
      detail = payload;
    } catch (error) { errorMessage = error instanceof Error ? error.message : '年度数据读取失败'; }
    finally { loading = false; }
  }

  function blankDraft(): Draft {
    if (dataset === 'school_tiers') return { track: 'recommendation', tier: '985', admitted: '0', percentage: '0' };
    if (dataset === 'schools') return { track: 'recommendation', tier: '985', school: '', admitted: '1' };
    if (dataset === 'subjects') return { program: 'academic', phase: 'initial_subject', subject: '', highest: '', lowest: '', average: '', median: '' };
    if (dataset === 'score_bands') return { program: 'combined', band: '', band_order: '1', interviewed: '0', admitted: '0', cumulative_interviewed: '0', cumulative_admitted: '0', note: '' };
    return { program: 'combined', lab: '1', admitted: '0', rejected: '0', first_choice: '0', highest: '', lowest: '', average: '', median: '', note: '' };
  }

  function newItem() { editing = false; editingItem = null; draft = blankDraft(); drawerOpen = true; errorMessage = ''; }

  function editItem(item: DataItem) {
    editing = true;
    editingItem = item;
    draft = Object.fromEntries(Object.entries(item).map(([key, value]) => [key, text(value)]));
    drawerOpen = true; errorMessage = '';
  }

  function payloadFromDraft(): Record<string, string | number | null> {
    const payload: Record<string, string | number | null> = { year: selectedYear };
    if (dataset === 'school_tiers') return { ...payload, track: draft.track, tier: draft.tier, admitted: Number(draft.admitted), percentage: Number(draft.percentage) };
    if (dataset === 'schools') return { ...payload, track: draft.track, tier: draft.tier, school: draft.school, admitted: Number(draft.admitted) };
    if (dataset === 'subjects') return { ...payload, program: draft.program, phase: draft.phase, subject: draft.subject, highest: Number(draft.highest), lowest: Number(draft.lowest), average: Number(draft.average), median: Number(draft.median) };
    if (dataset === 'score_bands') return { ...payload, program: draft.program, band: draft.band, band_order: Number(draft.band_order), interviewed: Number(draft.interviewed), admitted: Number(draft.admitted), cumulative_interviewed: Number(draft.cumulative_interviewed), cumulative_admitted: Number(draft.cumulative_admitted), note: draft.note || null };
    return { ...payload, program: draft.program, lab: Number(draft.lab), admitted: Number(draft.admitted), rejected: Number(draft.rejected), first_choice: Number(draft.first_choice), highest: optionalNumber(draft.highest), lowest: optionalNumber(draft.lowest), average: optionalNumber(draft.average), median: optionalNumber(draft.median), note: draft.note || null };
  }

  async function saveItem() {
    saving = true; errorMessage = '';
    try {
      const response = await fetch(`/api/v1/admin/${endpoint[dataset]}`, { method: 'POST', headers: { 'content-type': 'application/json', 'x-admin-token': token }, body: JSON.stringify(payloadFromDraft()) });
      const payload = await response.json().catch(() => null);
      if (!response.ok) throw new Error(payload?.error || '数据保存失败');
      await loadDetail(); drawerOpen = false; editingItem = null; notice = editing ? '数据修改已保存。' : '新数据已添加。';
    } catch (error) { errorMessage = error instanceof Error ? error.message : '数据保存失败'; }
    finally { saving = false; }
  }

  async function deleteItem(item: DataItem) {
    if (!confirm(`确定删除这条${datasetLabels[dataset]}记录吗？删除后网站会立即移除。`)) return;
    saving = true; errorMessage = '';
    try {
      const response = await fetch(`/api/v1/admin/${endpoint[dataset]}`, { method: 'DELETE', headers: { 'content-type': 'application/json', 'x-admin-token': token }, body: JSON.stringify({ year: selectedYear, ...item }) });
      if (!response.ok) { const payload = await response.json().catch(() => null); throw new Error(payload?.error || '数据删除失败'); }
      await loadDetail(); drawerOpen = false; editingItem = null; notice = '数据已删除。';
    } catch (error) { errorMessage = error instanceof Error ? error.message : '数据删除失败'; }
    finally { saving = false; }
  }

  async function selectYear(event: Event) { selectedYear = Number((event.currentTarget as HTMLSelectElement).value); drawerOpen = false; await loadDetail(); }
  onMount(async () => {
    selectedYear = reports[0]?.year ?? selectedYear;
    await loadDetail();
  });
</script>

<section class="manager-panel">
  <div class="manager-head">
    <div><small>当前维护年度</small><select value={selectedYear} onchange={selectYear}>{#each reports as report}<option value={report.year}>{report.year} 年{report.year === reports[0]?.year ? '（主页）' : ''}</option>{/each}</select></div>
    <div class="head-copy"><strong>{currentReport?.title ?? '尚未创建年度报告'}</strong><span>这里保存的年度总览和明细会直接显示在网站的年度报告中。</span></div>
    <div class="head-actions">{#if currentReport}<button onclick={() => onEditReport(currentReport)}><FilePenLine size={15} />编辑年度总览</button>{/if}<button class="primary" onclick={onNewReport}><Plus size={15} />新增年度</button></div>
  </div>

  {#if errorMessage}<div class="message error">{errorMessage}<button aria-label="关闭错误" onclick={() => errorMessage = ''}><X size={14} /></button></div>{/if}
  {#if notice}<div class="message success">{notice}<button aria-label="关闭提示" onclick={() => notice = ''}><X size={14} /></button></div>{/if}

  <div class="summary-grid">
    <div><span>生源层次</span><strong>{detail?.school_tiers.length ?? 0}</strong></div><div><span>生源院校</span><strong>{detail?.schools.length ?? 0}</strong></div><div><span>科目统计</span><strong>{detail?.subjects.length ?? 0}</strong></div><div><span>分数段</span><strong>{detail?.score_bands.length ?? 0}</strong></div><div><span>科室数据</span><strong>{detail?.labs.length ?? 0}</strong></div>
  </div>

  <div class="dataset-bar"><div>{#each Object.entries(datasetLabels) as [key, label]}<button class:active={dataset === key} onclick={() => { dataset = key as Dataset; drawerOpen = false; }}>{label}<b>{detail?.[key as Dataset].length ?? 0}</b></button>{/each}</div><button class="add-button" onclick={newItem}><Plus size={15} />新增{datasetLabels[dataset]}</button></div>

  {#if loading}<div class="empty"><span class="spin"><RefreshCw size={20} /></span>正在读取年度数据</div>
  {:else}<div class="table-scroll"><table>
    {#if dataset === 'school_tiers'}<thead><tr><th>招生方式</th><th>学校层次</th><th>录取人数</th><th>占比</th><th>操作</th></tr></thead><tbody>{#each detail?.school_tiers ?? [] as item}<tr><td>{trackLabel(item.track)}</td><td><strong>{tierLabel(item.tier)}</strong></td><td>{item.admitted}</td><td>{item.percentage.toFixed(2)}%</td><td><div class="actions"><button onclick={() => editItem(item)} title="编辑"><FilePenLine size={15} /></button><button class="danger" onclick={() => deleteItem(item)} title="删除"><Trash2 size={15} /></button></div></td></tr>{:else}<tr><td colspan="5" class="empty">暂无生源层次数据</td></tr>{/each}</tbody>
    {:else if dataset === 'schools'}<thead><tr><th>招生方式</th><th>学校层次</th><th>学校</th><th>录取人数</th><th>操作</th></tr></thead><tbody>{#each detail?.schools ?? [] as item}<tr><td>{trackLabel(item.track)}</td><td>{tierLabel(item.tier)}</td><td><strong>{item.school}</strong></td><td>{item.admitted}</td><td><div class="actions"><button onclick={() => editItem(item)} title="编辑"><FilePenLine size={15} /></button><button class="danger" onclick={() => deleteItem(item)} title="删除"><Trash2 size={15} /></button></div></td></tr>{:else}<tr><td colspan="5" class="empty">暂无院校数据</td></tr>{/each}</tbody>
    {:else if dataset === 'subjects'}<thead><tr><th>项目</th><th>阶段</th><th>科目</th><th>最高 / 最低</th><th>平均 / 中位</th><th>操作</th></tr></thead><tbody>{#each detail?.subjects ?? [] as item}<tr><td>{programLabel(item.program)}</td><td>{item.phase === 'initial_subject' ? '初试科目' : '录取成绩'}</td><td><strong>{item.subject}</strong></td><td>{item.highest} / {item.lowest}</td><td>{item.average} / {item.median}</td><td><div class="actions"><button onclick={() => editItem(item)} title="编辑"><FilePenLine size={15} /></button><button class="danger" onclick={() => deleteItem(item)} title="删除"><Trash2 size={15} /></button></div></td></tr>{:else}<tr><td colspan="6" class="empty">暂无科目数据</td></tr>{/each}</tbody>
    {:else if dataset === 'score_bands'}<thead><tr><th>项目</th><th>分数段</th><th>排序</th><th>复试 / 录取</th><th>累计复试 / 录取</th><th>备注</th><th>操作</th></tr></thead><tbody>{#each detail?.score_bands ?? [] as item}<tr><td>{programLabel(item.program)}</td><td><strong>{item.band}</strong></td><td>{item.band_order}</td><td>{item.interviewed} / {item.admitted}</td><td>{item.cumulative_interviewed} / {item.cumulative_admitted}</td><td>{item.note ?? '-'}</td><td><div class="actions"><button onclick={() => editItem(item)} title="编辑"><FilePenLine size={15} /></button><button class="danger" onclick={() => deleteItem(item)} title="删除"><Trash2 size={15} /></button></div></td></tr>{:else}<tr><td colspan="7" class="empty">暂无分数段数据</td></tr>{/each}</tbody>
    {:else}<thead><tr><th>项目</th><th>科室</th><th>录取 / 未录取</th><th>一志愿</th><th>最高 / 最低</th><th>平均 / 中位</th><th>备注</th><th>操作</th></tr></thead><tbody>{#each detail?.labs ?? [] as item}<tr><td>{programLabel(item.program)}</td><td><strong>{item.lab} 室</strong></td><td>{item.admitted} / {item.rejected}</td><td>{item.first_choice}</td><td>{item.highest ?? '-'} / {item.lowest ?? '-'}</td><td>{item.average ?? '-'} / {item.median ?? '-'}</td><td>{item.note ?? '-'}</td><td><div class="actions"><button onclick={() => editItem(item)} title="编辑"><FilePenLine size={15} /></button><button class="danger" onclick={() => deleteItem(item)} title="删除"><Trash2 size={15} /></button></div></td></tr>{:else}<tr><td colspan="8" class="empty">暂无科室数据</td></tr>{/each}</tbody>{/if}
  </table></div>{/if}
</section>

{#if drawerOpen}<div class="drawer-layer"><button class="scrim" aria-label="关闭编辑器" onclick={() => drawerOpen = false}></button><form class="drawer" onsubmit={(event) => { event.preventDefault(); saveItem(); }}><header><div><small>{editing ? 'EDIT DATA' : 'NEW DATA'}</small><h2>{editing ? '编辑' : '新增'}{datasetLabels[dataset]}</h2></div><button type="button" aria-label="关闭" onclick={() => drawerOpen = false}><X size={18} /></button></header><div class="fields">
  {#if dataset === 'school_tiers'}<label>招生方式<select bind:value={draft.track} disabled={editing}><option value="recommendation">推免</option><option value="exam">统考</option></select></label><label>学校层次<select bind:value={draft.tier} disabled={editing}><option value="985">985（含国科大）</option><option value="211">211（非 985）</option><option value="non_211">双非</option></select></label><label>录取人数<input type="number" min="0" bind:value={draft.admitted} required /></label><label>占比 (%)<input type="number" min="0" max="100" step="0.01" bind:value={draft.percentage} required /></label>
  {:else if dataset === 'schools'}<label>招生方式<select bind:value={draft.track} disabled={editing}><option value="recommendation">推免</option><option value="exam">统考</option></select></label><label>学校层次<select bind:value={draft.tier}><option value="985">985（含国科大）</option><option value="211">211（非 985）</option><option value="non_211">双非</option></select></label><label class="full">学校名称<input bind:value={draft.school} disabled={editing} required maxlength="120" /></label><label>录取人数<input type="number" min="1" bind:value={draft.admitted} required /></label>
  {:else if dataset === 'subjects'}<label>项目<select bind:value={draft.program} disabled={editing}><option value="academic">学硕</option><option value="professional">专硕</option><option value="combined">合计</option></select></label><label>阶段<select bind:value={draft.phase} disabled={editing}><option value="initial_subject">初试科目</option><option value="admitted_total">录取成绩</option></select></label><label class="full">科目名称<input bind:value={draft.subject} disabled={editing} required /></label><label>最高分<input type="number" min="0" step="0.01" bind:value={draft.highest} required /></label><label>最低分<input type="number" min="0" step="0.01" bind:value={draft.lowest} required /></label><label>平均分<input type="number" min="0" step="0.01" bind:value={draft.average} required /></label><label>中位数<input type="number" min="0" step="0.01" bind:value={draft.median} required /></label>
  {:else if dataset === 'score_bands'}<label>项目<select bind:value={draft.program} disabled={editing}><option value="academic">学硕</option><option value="professional">专硕</option><option value="combined">合计</option></select></label><label>分数段<input bind:value={draft.band} disabled={editing} placeholder="350-359" required /></label><label>显示顺序<input type="number" min="0" bind:value={draft.band_order} required /></label><label>复试人数<input type="number" min="0" bind:value={draft.interviewed} required /></label><label>录取人数<input type="number" min="0" bind:value={draft.admitted} required /></label><label>累计复试<input type="number" min="0" bind:value={draft.cumulative_interviewed} required /></label><label>累计录取<input type="number" min="0" bind:value={draft.cumulative_admitted} required /></label><label class="full">备注<input bind:value={draft.note} /></label>
  {:else}<label>项目<select bind:value={draft.program} disabled={editing}><option value="academic">学硕</option><option value="professional">专硕</option><option value="combined">合计</option></select></label><label>科室编号<input type="number" min="1" max="20" bind:value={draft.lab} disabled={editing} required /></label><label>录取人数<input type="number" min="0" bind:value={draft.admitted} required /></label><label>未录取人数<input type="number" min="0" bind:value={draft.rejected} required /></label><label>一志愿人数<input type="number" min="0" bind:value={draft.first_choice} required /></label><label>最高分<input type="number" min="0" step="0.01" bind:value={draft.highest} /></label><label>最低分<input type="number" min="0" step="0.01" bind:value={draft.lowest} /></label><label>平均分<input type="number" min="0" step="0.01" bind:value={draft.average} /></label><label>中位数<input type="number" min="0" step="0.01" bind:value={draft.median} /></label><label class="full">备注<input bind:value={draft.note} /></label>{/if}
</div><footer>{#if editingItem}<button type="button" class="delete" onclick={() => deleteItem(editingItem!)}><Trash2 size={15} />删除</button>{/if}<span></span><button type="button" onclick={() => drawerOpen = false}>取消</button><button class="save" disabled={saving}><Save size={15} />{saving ? '保存中...' : '保存'}</button></footer></form></div>{/if}

<style>
  button, input, select { font: inherit; letter-spacing: 0; }
  .manager-panel { min-height: 520px; overflow: hidden; border: 1px solid #dce5ec; border-radius: 10px; background: white; box-shadow: 0 10px 30px rgba(36,62,82,.045); }
  .manager-head { display: grid; min-height: 112px; padding: 18px; grid-template-columns: 150px 1fr auto; align-items: center; border-bottom: 1px solid #e7edf1; gap: 18px; }
  .manager-head small, .manager-head strong, .manager-head span { display: block; } .manager-head small { margin-bottom: 7px; color: #8b99a3; font-size: 9px; font-weight: 850; }
  .manager-head select { width: 140px; height: 40px; padding: 0 10px; border: 1px solid #d7e2e8; border-radius: 7px; background: #f8fafb; color: #17232b; font-weight: 850; }
  .head-copy { min-width: 0; } .head-copy strong { overflow: hidden; font-size: 15px; text-overflow: ellipsis; white-space: nowrap; } .head-copy span { margin-top: 7px; color: #83919b; font-size: 10px; }
  .head-actions, .actions { display: flex; gap: 5px; } .head-actions button, .add-button { display: inline-flex; height: 38px; padding: 0 12px; align-items: center; border: 1px solid #d7e2e8; border-radius: 7px; background: white; color: #52636f; gap: 6px; font-size: 10px; font-weight: 850; cursor: pointer; white-space: nowrap; } .head-actions .primary, .add-button { border-color: #14261f; background: #14261f; color: white; }
  .message { display: flex; min-height: 40px; padding: 9px 14px; align-items: center; border-bottom: 1px solid; font-size: 11px; font-weight: 750; } .message button { margin-left: auto; border: 0; background: transparent; color: inherit; } .message.error { border-color: #eccfc9; background: #fff3f0; color: #9d4339; } .message.success { border-color: #cfe6dc; background: #edf8f3; color: #176b53; }
  .summary-grid { display: grid; grid-template-columns: repeat(5, 1fr); border-bottom: 1px solid #e7edf1; background: #fbfcfd; } .summary-grid div { padding: 13px 16px; border-right: 1px solid #e7edf1; } .summary-grid div:last-child { border-right: 0; } .summary-grid span, .summary-grid strong { display: block; } .summary-grid span { color: #8997a1; font-size: 9px; font-weight: 800; } .summary-grid strong { margin-top: 5px; font-size: 18px; }
  .dataset-bar { display: flex; min-height: 58px; padding: 10px 14px; align-items: center; border-bottom: 1px solid #e7edf1; gap: 12px; } .dataset-bar > div { display: flex; overflow-x: auto; gap: 3px; } .dataset-bar > div button { display: inline-flex; height: 34px; padding: 0 10px; align-items: center; border: 0; border-radius: 6px; background: transparent; color: #75848e; gap: 6px; font-size: 10px; font-weight: 800; white-space: nowrap; } .dataset-bar > div button.active { background: #edf4f1; color: #176b53; } .dataset-bar b { min-width: 18px; padding: 2px 5px; border-radius: 9px; background: #e4eaed; color: #70808b; font-size: 8px; } .dataset-bar .add-button { margin-left: auto; }
  .table-scroll { overflow: auto; } table { width: 100%; border-collapse: collapse; text-align: left; } thead { background: #fbfcfd; color: #95a2ab; font-size: 9px; } th { height: 38px; padding: 0 15px; border-bottom: 1px solid #e7edf1; white-space: nowrap; } td { height: 52px; padding: 8px 15px; border-bottom: 1px solid #eef2f4; color: #5c6d78; font-size: 10px; white-space: nowrap; } td strong { color: #1c2932; font-size: 11px; } tbody tr:hover { background: #fbfdfc; }
  .actions button { display: grid; width: 30px; height: 30px; padding: 0; place-items: center; border: 0; border-radius: 6px; background: transparent; color: #71818c; cursor: pointer; } .actions button:hover { background: #eaf1ee; color: #176b53; } .actions .danger:hover { background: #fbeceb; color: #b3453f; }
  .empty { height: 150px; color: #8d9ba5; text-align: center; } div.empty { display: flex; align-items: center; justify-content: center; gap: 8px; font-size: 11px; } .spin { animation: spin .8s linear infinite; } @keyframes spin { to { transform: rotate(360deg); } }
  .drawer-layer { position: fixed; z-index: 100; inset: 0; } .scrim { position: absolute; inset: 0; width: 100%; border: 0; background: rgba(17,24,32,.4); } .drawer { position: absolute; inset: 0 0 0 auto; display: flex; width: min(560px,100%); flex-direction: column; background: #f7fafb; box-shadow: -18px 0 55px rgba(17,24,32,.17); }
  .drawer header { display: flex; min-height: 74px; padding: 15px 20px; align-items: center; justify-content: space-between; border-bottom: 1px solid #dce5ec; background: white; } .drawer header small { color: #8d9ba5; font-size: 9px; font-weight: 850; } .drawer header h2 { margin: 4px 0 0; font-size: 18px; } .drawer header button { display: grid; width: 34px; height: 34px; place-items: center; border: 1px solid #dce5ec; border-radius: 7px; background: white; }
  .fields { display: grid; padding: 22px; flex: 1; align-content: start; grid-template-columns: 1fr 1fr; overflow: auto; gap: 14px; } .fields label { display: grid; color: #536571; gap: 7px; font-size: 10px; font-weight: 850; } .fields .full { grid-column: 1 / -1; } .fields input, .fields select { width: 100%; min-height: 42px; padding: 8px 10px; border: 1px solid #d5e0e7; border-radius: 7px; outline: 0; background: white; color: #1c2932; } .fields input:focus, .fields select:focus { border-color: #176b53; box-shadow: 0 0 0 3px rgba(23,107,83,.1); } .fields :disabled { background: #edf2f4; color: #788690; }
  .drawer footer { display: flex; min-height: 72px; padding: 14px 20px; align-items: center; border-top: 1px solid #dce5ec; background: white; gap: 8px; } .drawer footer span { flex: 1; } .drawer footer button { display: inline-flex; min-height: 40px; padding: 0 14px; align-items: center; border: 1px solid #d5e0e7; border-radius: 7px; background: white; color: #52636f; gap: 6px; font-size: 11px; font-weight: 850; } .drawer footer .save { border-color: #14261f; background: #14261f; color: white; } .drawer footer .delete { border-color: #e7c3bf; background: #fff5f3; color: #a43d37; }
  .manager-panel { border-color: var(--line); border-radius: 8px; box-shadow: var(--shadow-sm); }
  .head-actions .primary, .add-button, .drawer footer .save { border-color: var(--accent); background: linear-gradient(135deg, var(--accent), var(--accent-secondary)); color: white; }
  .dataset-bar > div button.active { background: var(--accent-soft); color: var(--accent); }
  .actions button:hover { background: var(--accent-soft); color: var(--accent); }
  .fields input:focus, .fields select:focus { border-color: var(--accent); box-shadow: 0 0 0 3px rgba(var(--accent-rgb),.12); }
  .message.success { border-color: #c9d9ff; background: var(--accent-soft); color: var(--accent-dark); }
  @media (max-width: 900px) { .manager-head { grid-template-columns: 1fr; } .head-actions { flex-wrap: wrap; } .summary-grid { grid-template-columns: repeat(2, 1fr); } .dataset-bar { align-items: stretch; flex-direction: column; } .dataset-bar .add-button { margin-left: 0; align-self: flex-start; } }
  @media (max-width: 560px) { .fields { grid-template-columns: 1fr; } .fields .full { grid-column: auto; } .drawer footer { flex-wrap: wrap; } }
</style>
