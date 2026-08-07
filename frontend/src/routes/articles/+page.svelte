<script lang="ts">
  import { ArrowRight, BookOpen, Search } from '@lucide/svelte';
  import type { Article } from '$lib/types';

  let { data }: { data: { articles: Article[]; category: string | null; q: string | null } } = $props();
  let search = $state('');
  $effect(() => { search = data.q ?? ''; });
  const categories = [
    { value: '', label: '全部内容' },
    { value: 'initial', label: '初试经验' },
    { value: 'reexam', label: '复试经验' },
    { value: 'career', label: '就业分享' },
    { value: 'policy', label: '政策资料' }
  ];
</script>

<svelte:head><title>内容库 | 信工所考研信息站</title></svelte:head>

<section class="page-heading">
  <p class="eyebrow">经验与资料</p>
  <h1 class="page-title">文章、政策与经验</h1>
  <p class="page-lead">这里收录初试、复试、就业和政策资料，可按分类或关键词查找。所有文章发布前都会经过审核。</p>
</section>

<section class="section articles-section">
  <form class="toolbar" method="GET">
    <div class="search-box"><Search size={17} /><input name="q" bind:value={search} placeholder="搜索标题、摘要或正文" aria-label="搜索内容" /><button class="icon-button" type="submit" aria-label="提交搜索"><ArrowRight size={17} /></button></div>
    <div class="category-tabs" aria-label="内容分类">
      {#each categories as item}
        <a class:active={(data.category ?? '') === item.value} href={item.value ? `/articles?category=${item.value}` : '/articles'}>{item.label}</a>
      {/each}
    </div>
  </form>

  <div class="results-head"><span>{data.articles.length} 条结果</span>{#if data.q}<span>关键词：{data.q}</span>{/if}</div>
  {#if data.articles.length}
    <div class="article-grid">
      {#each data.articles as article}
        <a class="content-card" href={`/articles/${article.slug}`}>
          <div class="card-meta"><span>{article.year ?? '资料'}</span><span class="status-pill published">已发布</span></div>
          <h2>{article.title}</h2>
          <p>{article.excerpt ?? '打开文章查看完整内容。'}</p>
          <span class="card-foot">{article.category} <ArrowRight size={15} /></span>
        </a>
      {/each}
    </div>
  {:else}
    <div class="empty-state surface"><BookOpen size={20} /><span>没有找到匹配内容。</span></div>
  {/if}
</section>

<style>
  .articles-section { padding-top: 32px; }
  .toolbar { display: grid; gap: 17px; }
  .search-box { display: flex; max-width: 620px; min-height: 46px; padding-left: 14px; align-items: center; border: 1px solid var(--line); border-radius: 8px; background: var(--paper); color: var(--muted); box-shadow: var(--shadow-sm); gap: 9px; transition: border-color .2s ease, box-shadow .2s ease; }
  .search-box:focus-within { border-color: var(--accent); box-shadow: 0 0 0 3px rgba(var(--accent-rgb), .12); }
  .search-box input { min-width: 0; flex: 1; height: 40px; border: 0; outline: 0; color: var(--ink); }
  .search-box .icon-button { width: 42px; height: 42px; border: 0; border-left: 1px solid var(--line); border-radius: 0; color: var(--green); }
  .category-tabs { display: flex; flex-wrap: wrap; border-bottom: 1px solid var(--line); gap: 3px; }
  .category-tabs a { padding: 9px 12px; border-bottom: 2px solid transparent; color: var(--muted); font-size: 12px; font-weight: 700; }
  .category-tabs a:hover, .category-tabs a.active { border-bottom-color: var(--green); color: var(--green-dark); }
  .results-head { display: flex; margin: 25px 0 13px; color: var(--muted); font-size: 12px; gap: 14px; }
  .article-grid { display: grid; grid-template-columns: repeat(3, minmax(0, 1fr)); gap: 12px; }
  .content-card { display: flex; min-height: 220px; padding: 20px; flex-direction: column; border: 1px solid var(--line); border-radius: 8px; background: var(--paper); box-shadow: var(--shadow-sm); transition: border-color .2s ease, box-shadow .2s ease, transform .2s ease; }
  .content-card:hover { border-color: rgba(var(--accent-rgb), .4); box-shadow: var(--shadow); transform: translateY(-3px); }
  .card-meta { display: flex; align-items: center; justify-content: space-between; color: var(--accent); font-size: 11px; font-weight: 800; }
  .content-card h2 { margin: 23px 0 8px; font-size: 17px; line-height: 1.4; }
  .content-card p { margin: 0; color: var(--muted); font-size: 12px; line-height: 1.7; }
  .card-foot { display: inline-flex; margin-top: auto; padding-top: 19px; align-items: center; color: var(--green); gap: 6px; font-size: 11px; font-weight: 750; }
  @media (max-width: 900px) { .article-grid { grid-template-columns: repeat(2, minmax(0, 1fr)); } }
  @media (max-width: 600px) { .article-grid { grid-template-columns: 1fr; } }
</style>
