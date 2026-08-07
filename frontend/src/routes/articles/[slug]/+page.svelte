<script lang="ts">
  import { ArrowLeft, CalendarDays, FileText } from '@lucide/svelte';
  import type { Article } from '$lib/types';
  let { data }: { data: { article: Article; html: string } } = $props();
  const date = $derived(new Intl.DateTimeFormat('zh-CN', { dateStyle: 'medium' }).format(new Date(data.article.updated_at)));
</script>

<svelte:head><title>{data.article.title} | 信工所考研信息站</title><meta name="description" content={data.article.excerpt ?? data.article.title} /></svelte:head>

<div class="article-layout">
  <a class="back-link" href="/articles"><ArrowLeft size={15} />返回内容库</a>
  <div class="article-meta-line"><span class="status-pill published">已发布</span><span>{data.article.category}</span><span><CalendarDays size={14} />更新于 {date}</span></div>
  <article class="prose" aria-labelledby="article-title">
    <h1 id="article-title">{data.article.title}</h1>
    {#if data.article.excerpt}<p class="lead">{data.article.excerpt}</p>{/if}
    {@html data.html}
  </article>
  <aside class="article-note surface"><FileText size={18} /><div><strong>资料边界</strong><p>本文为公开资料整理或个人经验，涉及年份、数据与政策时请以当年官方文件为准。</p></div></aside>
</div>

<style>
  .article-layout { max-width: 820px; padding: 37px 0 64px; }
  .back-link { display: inline-flex; align-items: center; color: var(--green); gap: 7px; font-size: 12px; font-weight: 750; }
  .article-meta-line { display: flex; margin-top: 23px; align-items: center; color: var(--muted); gap: 13px; font-size: 11px; }
  .article-meta-line span:last-child { display: inline-flex; align-items: center; gap: 5px; }
  .prose { padding-bottom: 30px; }
  .prose .lead { margin-top: -18px; color: var(--muted); font-size: 16px; line-height: 1.75; }
  .article-note { display: flex; padding: 16px 18px; align-items: flex-start; color: var(--green); gap: 12px; }
  .article-note strong { color: var(--ink); font-size: 13px; }
  .article-note p { margin: 5px 0 0; color: var(--muted); font-size: 12px; line-height: 1.65; }
</style>
