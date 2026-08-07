<script lang="ts">
  import Header from '$components/Header.svelte';
  import Footer from '$components/Footer.svelte';
  import { page } from '$app/state';
  import 'katex/dist/katex.min.css';
  import '../app.css';

  let { children } = $props();
  const isAdmin = $derived(page.url.pathname.startsWith('/admin'));
</script>

{#if isAdmin}
  <main class="admin-root">{@render children()}</main>
{:else}
  <Header />
  <main class="page-shell">{@render children()}</main>
  <Footer />
{/if}
