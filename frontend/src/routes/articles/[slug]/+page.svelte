<script lang="ts">
  import { onMount } from 'svelte';
  import { ArrowLeft, CalendarDays, FileText, LockKeyhole, LogIn, MessageCircle, Reply, Send } from '@lucide/svelte';
  import { extractMarkdownHeadings, renderMarkdown } from '$lib/markdown';
  import type { Article, ArticleComment, Category, GithubProfile } from '$lib/types';

  let { data }: { data: { article: Article; html: string; comments: ArticleComment[]; categories: Category[] } } = $props();
  let unlockedArticle = $state<Article | null>(null);
  let unlockedHtml = $state<string | null>(null);
  let submittedComments = $state<ArticleComment[]>([]);
  let submittedCommentsSlug = $state('');
  let viewer = $state<GithubProfile | null>(null);
  let password = $state('');
  let commentBody = $state('');
  let replyBody = $state('');
  let replyingTo = $state<string | null>(null);
  let unlockError = $state('');
  let commentError = $state('');
  let unlocking = $state(false);
  let posting = $state(false);
  let articleElement = $state<HTMLElement | null>(null);
  let activeHeadingId = $state('');
  const article = $derived(unlockedArticle?.slug === data.article.slug ? unlockedArticle : data.article);
  const html = $derived(unlockedArticle?.slug === data.article.slug && unlockedHtml !== null ? unlockedHtml : data.html);
  const comments = $derived([...data.comments, ...(submittedCommentsSlug === data.article.slug ? submittedComments : [])]);
  const rootComments = $derived(comments.filter((comment) => comment.parent_id === null));
  const date = $derived(new Intl.DateTimeFormat('zh-CN', { dateStyle: 'medium' }).format(new Date(article.updated_at)));
  const categoryLabel = $derived(data.categories.find((item) => item.slug === article.category)?.name ?? article.category);
  const articlePath = $derived(`/articles/${article.slug}`);
  const tocItems = $derived(extractMarkdownHeadings(article.body_markdown).filter((item) => item.text !== article.title));

  function updateActiveHeading() {
    if (!articleElement || tocItems.length === 0) return;
    let current = tocItems[0].id;
    for (const item of tocItems) {
      const heading = articleElement.querySelector<HTMLElement>(`#${item.id}`);
      if (heading && heading.getBoundingClientRect().top <= 140) current = item.id;
    }
    activeHeadingId = current;
  }

  onMount(() => {
    updateActiveHeading();
    window.addEventListener('scroll', updateActiveHeading, { passive: true });
    void fetch('/api/v1/auth/github/me').then(async (response) => {
      if (response.ok) viewer = await response.json();
    });
    return () => window.removeEventListener('scroll', updateActiveHeading);
  });

  async function unlock() {
    unlocking = true; unlockError = '';
    try {
      const response = await fetch(`/api/v1/articles/${encodeURIComponent(article.slug)}/unlock`, {
        method: 'POST', headers: { 'content-type': 'application/json' }, body: JSON.stringify({ password })
      });
      const payload = await response.json();
      if (!response.ok) throw new Error(payload.error || '无法解锁文章');
      unlockedArticle = payload;
      unlockedHtml = renderMarkdown(payload.body_markdown);
      password = '';
    } catch (error) { unlockError = error instanceof Error ? error.message : '无法解锁文章'; }
    finally { unlocking = false; }
  }

  function repliesFor(commentId: string) {
    return comments.filter((comment) => comment.parent_id === commentId);
  }

  async function postComment(parentId: string | null = null) {
    posting = true; commentError = '';
    const body = (parentId ? replyBody : commentBody).trim();
    try {
      const response = await fetch(`/api/v1/articles/${encodeURIComponent(article.slug)}/comments`, {
        method: 'POST', headers: { 'content-type': 'application/json' }, body: JSON.stringify({ body, parent_id: parentId })
      });
      const payload = await response.json();
      if (!response.ok) throw new Error(payload.error || '评论发布失败');
      submittedCommentsSlug = article.slug;
      submittedComments = [...submittedComments, payload];
      if (parentId) { replyBody = ''; replyingTo = null; } else commentBody = '';
    } catch (error) { commentError = error instanceof Error ? error.message : '评论发布失败'; }
    finally { posting = false; }
  }
</script>

<svelte:head><title>{article.title} | 信工所考研信息站</title><meta name="description" content={article.excerpt ?? article.title} /></svelte:head>

<div class="article-page-grid">
  <aside class="article-toc" aria-label="文章目录">
    {#if tocItems.length}
      <strong>目录</strong>
      <nav>
        {#each tocItems as item}
          <a class:active={activeHeadingId === item.id} class:subsection={item.level === 3} href={`#${item.id}`}>{item.text}</a>
        {/each}
      </nav>
    {/if}
  </aside>
  <div class="article-layout">
  <a class="back-link" href="/articles"><ArrowLeft size={15} />返回内容库</a>
  <div class="article-meta-line"><span class="status-pill published">已发布</span><span>{categoryLabel}</span>{#if article.is_protected}<span><LockKeyhole size={14} />访问受保护</span>{/if}<span><CalendarDays size={14} />更新于 {date}</span></div>
  <article class="prose" aria-labelledby="article-title" bind:this={articleElement}>
    <h1 id="article-title">{article.title}</h1>
    {#if article.excerpt}<p class="lead">{article.excerpt}</p>{/if}
    {#if article.is_protected && !article.body_markdown}
      <form class="unlock-box" onsubmit={(event) => { event.preventDefault(); unlock(); }}>
        <LockKeyhole size={20} /><div><strong>这篇文章需要访问密码</strong><p>输入文章发布者提供的密码后查看正文。</p></div>
        <label><span>访问密码</span><input type="password" bind:value={password} required minlength="6" maxlength="128" autocomplete="current-password" /></label>
        {#if unlockError}<p class="unlock-error">{unlockError}</p>{/if}
        <button class="button" type="submit" disabled={unlocking}>{unlocking ? '正在验证' : '查看文章'}</button>
      </form>
    {:else}
      {@html html}
    {/if}
  </article>
  <aside class="article-note surface"><FileText size={18} /><div><strong>资料边界</strong><p>本文为公开资料整理或个人经验，涉及年份、数据与政策时请以当年官方文件为准。</p></div></aside>
  <section class="comments" aria-labelledby="comments-title">
    <div class="comments-head"><div><p class="eyebrow">讨论</p><h2 id="comments-title"><MessageCircle size={18} />评论</h2></div><span>{comments.length} 条</span></div>
    {#if viewer}
      <form class="comment-form" onsubmit={(event) => { event.preventDefault(); postComment(); }}>
        <div class="comment-user">{#if viewer.avatar_url}<img src={viewer.avatar_url} alt="" />{/if}<strong>@{viewer.login}</strong></div>
        <textarea bind:value={commentBody} required minlength="1" maxlength="2000" placeholder="留下你的讨论或补充" aria-label="评论内容"></textarea>
        {#if commentError}<p class="comment-error">{commentError}</p>{/if}
        <button class="button" type="submit" disabled={posting || !commentBody.trim()}><Send size={15} />{posting ? '发布中' : '发布评论'}</button>
      </form>
    {:else}
      <div class="comment-login"><LogIn size={20} /><div><strong>使用 GitHub 账号参与讨论</strong><p>所有访客都可以查看评论，登录后即可发布。</p></div><a class="button" href={`/api/v1/auth/github?return_to=${encodeURIComponent(articlePath)}`}>GitHub 登录</a></div>
    {/if}
    <div class="comment-list">
      {#each rootComments as comment}
        <article class="comment"><div class="comment-author">{#if comment.author_avatar_url}<img src={comment.author_avatar_url} alt="" />{:else}<span>{comment.author_login.slice(0, 1).toUpperCase()}</span>{/if}<strong>@{comment.author_login}</strong><time>{new Intl.DateTimeFormat('zh-CN', { dateStyle: 'medium', timeStyle: 'short' }).format(new Date(comment.created_at))}</time></div><p>{comment.body}</p>{#if viewer}<button class="reply-button" type="button" onclick={() => { replyingTo = replyingTo === comment.id ? null : comment.id; replyBody = ''; }}><Reply size={14} />{replyingTo === comment.id ? '取消回复' : '回复'}</button>{/if}{#if replyingTo === comment.id}<form class="comment-reply-form" onsubmit={(event) => { event.preventDefault(); postComment(comment.id); }}><textarea bind:value={replyBody} required minlength="1" maxlength="2000" placeholder={`回复 @${comment.author_login}`} aria-label={`回复 @${comment.author_login}`}></textarea><button class="button" type="submit" disabled={posting || !replyBody.trim()}><Send size={14} />{posting ? '发布中' : '发布回复'}</button></form>{/if}{#each repliesFor(comment.id) as reply}<div class="comment-reply"><div class="comment-author">{#if reply.author_avatar_url}<img src={reply.author_avatar_url} alt="" />{:else}<span>{reply.author_login.slice(0, 1).toUpperCase()}</span>{/if}<strong>@{reply.author_login}</strong><time>{new Intl.DateTimeFormat('zh-CN', { dateStyle: 'medium', timeStyle: 'short' }).format(new Date(reply.created_at))}</time></div><p>{reply.body}</p></div>{/each}</article>
      {:else}
        <p class="comment-empty">还没有评论，欢迎分享你的补充或问题。</p>
      {/each}
    </div>
  </section>
  </div>
</div>

<style>
  .article-page-grid { display: grid; max-width: 1014px; margin: 0 auto; grid-template-columns: 230px minmax(0, 740px); align-items: start; gap: 44px; }
  .article-layout { width: 100%; max-width: 740px; padding: 37px 0 64px; }
  .article-toc { position: sticky; top: 104px; max-height: calc(100vh - 128px); padding: 39px 0 24px; overflow-y: auto; }
  .article-toc > strong { display: block; margin-bottom: 14px; color: var(--ink); font-size: 14px; }
  .article-toc nav { position: static; display: grid; align-items: stretch; border-left: 1px solid var(--line); gap: 0; }
  .article-toc a { position: relative; display: block; height: auto; padding: 9px 0 9px 16px; border-radius: 0; background: transparent; color: var(--muted); font-size: 13px; font-weight: 700; line-height: 1.5; }
  .article-toc a::before { position: absolute; top: 6px; bottom: 6px; left: -1px; width: 2px; background: transparent; content: ''; }
  .article-toc a:hover, .article-toc a.active { background: transparent; color: var(--accent); }
  .article-toc a.active::before { background: var(--accent); }
  .article-toc a.subsection { padding-left: 29px; font-size: 12px; }
  .back-link { display: inline-flex; align-items: center; color: var(--green); gap: 7px; font-size: 12px; font-weight: 750; }
  .article-meta-line { display: flex; margin-top: 23px; align-items: center; color: var(--muted); gap: 13px; font-size: 11px; }
  .article-meta-line span:last-child { display: inline-flex; align-items: center; gap: 5px; }
  .prose { padding-bottom: 30px; }
  .prose .lead { margin-top: -18px; color: var(--muted); font-size: 16px; line-height: 1.75; }
  .article-note { display: flex; padding: 16px 18px; align-items: flex-start; color: var(--green); gap: 12px; }
  .article-note strong { color: var(--ink); font-size: 13px; }
  .article-note p { margin: 5px 0 0; color: var(--muted); font-size: 12px; line-height: 1.65; }
  .unlock-box { display: grid; margin-top: 28px; padding: 22px; grid-template-columns: auto 1fr; border: 1px solid var(--line); border-radius: 8px; background: #f7faf8; color: var(--green); gap: 10px 13px; }
  .unlock-box strong { color: var(--ink); font-size: 15px; } .unlock-box p { margin: 4px 0 0; color: var(--muted); font-size: 12px; } .unlock-box label { display: grid; grid-column: 1 / -1; gap: 6px; color: var(--ink); font-size: 12px; font-weight: 700; } .unlock-box input, .comment-form textarea { width: 100%; border: 1px solid var(--line); border-radius: 6px; background: white; color: var(--ink); font: inherit; outline: 0; } .unlock-box input { height: 40px; padding: 0 10px; } .unlock-box input:focus, .comment-form textarea:focus { border-color: var(--accent); box-shadow: 0 0 0 3px rgba(var(--accent-rgb), .12); } .unlock-box .button { justify-self: start; } .unlock-error, .comment-error { margin: 0; color: #a73931 !important; font-size: 12px !important; }
  .comments { margin-top: 38px; padding-top: 25px; border-top: 1px solid var(--line); } .comments-head { display: flex; align-items: end; justify-content: space-between; } .comments-head h2 { display: flex; margin: 3px 0 0; align-items: center; font-size: 20px; gap: 8px; } .comments-head > span { color: var(--muted); font-size: 12px; }
  .comment-login, .comment-form { margin-top: 18px; padding: 16px; border: 1px solid var(--line); border-radius: 8px; background: #f8fafb; } .comment-login { display: flex; align-items: center; gap: 12px; } .comment-login :global(svg) { color: var(--green); } .comment-login div { flex: 1; } .comment-login strong, .comment-login p { display: block; } .comment-login strong { font-size: 13px; } .comment-login p { margin: 4px 0 0; color: var(--muted); font-size: 12px; } .comment-login .button { flex: none; }
  .comment-form { display: grid; gap: 10px; } .comment-user, .comment-author { display: flex; align-items: center; gap: 8px; font-size: 12px; } .comment-user img, .comment-author img, .comment-author > span { width: 24px; height: 24px; border-radius: 50%; object-fit: cover; } .comment-author > span { display: grid; place-items: center; background: var(--accent-soft); color: var(--accent-dark); font-size: 10px; font-weight: 800; } .comment-form textarea { min-height: 92px; padding: 10px; resize: vertical; } .comment-form .button { justify-self: start; }
  .comment-list { display: grid; margin-top: 17px; gap: 0; border-top: 1px solid var(--line); } .comment { padding: 16px 2px; border-bottom: 1px solid var(--line); } .comment-author time { margin-left: auto; color: var(--muted); font-size: 11px; } .comment p { margin: 9px 0 0 32px; white-space: pre-wrap; color: var(--ink); font-size: 14px; line-height: 1.7; } .comment-empty { margin: 17px 0; color: var(--muted); font-size: 13px; } .reply-button { display: inline-flex; margin: 10px 0 0 32px; padding: 0; align-items: center; border: 0; background: transparent; color: var(--green); cursor: pointer; gap: 5px; font-size: 11px; font-weight: 750; } .comment-reply-form { display: grid; margin: 12px 0 4px 32px; gap: 8px; } .comment-reply-form textarea { min-height: 66px; padding: 9px; border: 1px solid var(--line); border-radius: 6px; resize: vertical; font: inherit; } .comment-reply-form .button { justify-self: start; } .comment-reply { margin: 14px 0 0 32px; padding: 12px; border-left: 2px solid var(--line); background: #f8fafb; } .comment-reply p { margin-left: 0; }
  @media (max-width: 1040px) { .article-page-grid { display: block; max-width: 740px; } .article-toc { display: none; } }
  @media (max-width: 580px) { .article-meta-line { flex-wrap: wrap; } .comment-login { align-items: flex-start; flex-wrap: wrap; } .comment-login .button { width: 100%; } .comment-author time { font-size: 10px; } }
</style>
