<script lang="ts">
  import { BookOpen, Database, FilePenLine, Menu, Search, ShieldCheck, X } from '@lucide/svelte';
  import { page } from '$app/state';

  let open = false;
  const links = [
    { href: '/', label: '总览', icon: ShieldCheck },
    { href: '/articles', label: '内容库', icon: BookOpen },
    { href: '/data', label: '招生数据', icon: Database },
    { href: '/contribute', label: '我要投稿', icon: FilePenLine }
  ];
</script>

<header class="site-header">
  <div class="header-inner">
    <a class="brand" href="/" aria-label="信工所考研信息站首页">
      <span class="brand-mark">IIE</span>
      <span><strong>信工所考研信息站</strong><small>招生数据与经验分享</small></span>
    </a>

    <button class="icon-button mobile-toggle" type="button" aria-label={open ? '关闭菜单' : '打开菜单'} onclick={() => (open = !open)}>
      {#if open}<X size={20} />{:else}<Menu size={20} />{/if}
    </button>

    <nav class:open aria-label="主导航">
      {#each links as link}
        <a class:active={page.url.pathname === link.href || (link.href !== '/' && page.url.pathname.startsWith(link.href))} href={link.href} onclick={() => (open = false)}>
          <svelte:component this={link.icon} size={16} strokeWidth={1.8} />
          <span>{link.label}</span>
        </a>
      {/each}
      <a class="search-link" href="/articles?focus=search" onclick={() => (open = false)}><Search size={16} />搜索</a>
    </nav>
  </div>
</header>
