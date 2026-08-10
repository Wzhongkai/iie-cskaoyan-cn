<script lang="ts">
  import { Bold, CheckCircle2, Code2, FilePlus2, FileText, Heading2, ImagePlus, Italic, Link2, List, Paperclip, Quote, Send, Sigma, Upload } from '@lucide/svelte';
  import JSZip from 'jszip';
  import { parseDocument } from 'yaml';
  import { renderMarkdown } from '$lib/markdown';
  import type { Category } from '$lib/types';

  let { data }: { data: { categories: Category[] } } = $props();

  let title = $state('');
  let category = $state('');
  let year = $state(String(new Date().getFullYear()));
  let background = $state('');
  let contact = $state('');
  let body = $state('');
  let consent = $state(false);
  let website = $state('');
  let mode = $state<'edit' | 'preview'>('edit');
  let submitting = $state(false);
  let uploading = $state(false);
  let result = $state('');
  let errorMessage = $state('');
  let uploadedFileName = $state('');
  let unresolvedImages = $derived(localImageReferences(body));
  let bodyEditor = $state<HTMLTextAreaElement | undefined>(undefined);

  function frontMatter(source: string) {
    if (!source.startsWith('---')) return { content: source, values: {} as Record<string, unknown> };
    const match = source.match(/^---\s*\r?\n([\s\S]*?)\r?\n---\s*(?:\r?\n|$)/);
    if (!match) return { content: source, values: {} as Record<string, unknown> };
    try {
      const values = parseDocument(match[1]).toJS() as Record<string, unknown>;
      return { content: source.slice(match[0].length), values: values ?? {} };
    } catch {
      throw new Error('文件头信息格式有误，请检查冒号和缩进。');
    }
  }

  function applyFrontMatter(values: Record<string, unknown>) {
    if (typeof values.title === 'string') title = values.title.slice(0, 120);
    if (typeof values.category === 'string' && data.categories.some((item) => item.slug === values.category)) category = values.category;
    const parsedYear = Number(values.year ?? values.date?.toString().slice(0, 4));
    if (Number.isInteger(parsedYear) && parsedYear >= 2010 && parsedYear <= 2100) year = String(parsedYear);
    const summary = values.summary ?? values.excerpt ?? values.description;
    if (typeof summary === 'string') background = summary.slice(0, 240);
  }

  function localImageReferences(source: string) {
    const references = new Set<string>();
    const pattern = /!\[[^\]]*\]\(\s*(?:<([^>]+)>|([^\s)]+))(?:\s+["'][^"']*["'])?\s*\)/gi;
    for (const match of source.matchAll(pattern)) {
      const reference = match[1] ?? match[2];
      if (!/^(?:https?:\/\/|\/uploads\/|data:)/i.test(reference)) references.add(reference);
    }
    return [...references];
  }

  function normalizeBundlePath(value: string) {
    const parts: string[] = [];
    for (const part of value.replaceAll('\\', '/').split('/')) {
      if (!part || part === '.') continue;
      if (part === '..') parts.pop();
      else parts.push(part);
    }
    return parts.join('/');
  }

  function imageMimeType(name: string) {
    const extension = name.split('.').pop()?.toLowerCase();
    return extension === 'png' ? 'image/png' : extension === 'jpg' || extension === 'jpeg' ? 'image/jpeg' : extension === 'webp' ? 'image/webp' : extension === 'gif' ? 'image/gif' : '';
  }

  async function uploadImageFile(file: File) {
    if (file.size > 5 * 1024 * 1024) throw new Error(`图片 ${file.name} 不能超过 5 MB`);
    if (!imageMimeType(file.name)) throw new Error(`图片 ${file.name} 不是 JPEG、PNG、WebP 或 GIF`);
    const form = new FormData();
    form.append('image', file);
    const response = await fetch('/api/v1/uploads', { method: 'POST', body: form });
    const payload = await response.json();
    if (!response.ok) throw new Error(payload.error || `图片 ${file.name} 上传失败`);
    return payload.url as string;
  }

  async function importBundle(file: File) {
    if (file.size > 20 * 1024 * 1024) throw new Error('ZIP 图文包不能超过 20 MB');
    const bundle = await JSZip.loadAsync(file);
    const markdownEntries = Object.values(bundle.files).filter((entry) => !entry.dir && entry.name.toLowerCase().endsWith('.md'));
    if (markdownEntries.length !== 1) throw new Error('ZIP 图文包中需要且只能包含一个 .md 文件');

    const markdownEntry = markdownEntries[0];
    const parsed = frontMatter(await markdownEntry.async('text'));
    applyFrontMatter(parsed.values);
    let importedBody = parsed.content;
    const markdownDirectory = markdownEntry.name.includes('/') ? markdownEntry.name.slice(0, markdownEntry.name.lastIndexOf('/') + 1) : '';
    const entriesByPath = new Map(Object.values(bundle.files).filter((entry) => !entry.dir).map((entry) => [normalizeBundlePath(entry.name).toLowerCase(), entry]));

    function findImageEntry(reference: string) {
      const relativePath = normalizeBundlePath(`${markdownDirectory}${reference}`).toLowerCase();
      const exactEntry = entriesByPath.get(relativePath);
      if (exactEntry) return exactEntry;

      const portablePath = normalizeBundlePath(reference.replace(/^file:\/+/i, '').replace(/^[a-z]:[\\/]/i, '')).toLowerCase();
      const pathParts = portablePath.split('/').filter(Boolean);
      for (let index = 0; index < pathParts.length; index += 1) {
        const suffix = pathParts.slice(index).join('/');
        const matches = [...entriesByPath.entries()].filter(([path]) => path === suffix || path.endsWith(`/${suffix}`));
        if (matches.length === 1) return matches[0][1];
      }
      return undefined;
    }

    for (const reference of localImageReferences(importedBody)) {
      let decodedReference = reference;
      try { decodedReference = decodeURIComponent(reference); } catch { /* Keep the original path when it is not URL encoded. */ }
      const imageEntry = findImageEntry(decodedReference);
      if (!imageEntry) throw new Error(`ZIP 中找不到 Markdown 引用的图片：${reference}`);
      const mimeType = imageMimeType(imageEntry.name);
      if (!mimeType) throw new Error(`不支持的图片格式：${reference}`);
      const imageFile = new File([await imageEntry.async('blob')], imageEntry.name.split('/').pop() ?? 'image', { type: mimeType });
      const uploadedUrl = await uploadImageFile(imageFile);
      importedBody = importedBody.split(reference).join(uploadedUrl);
    }
    body = importedBody;
  }

  async function readMarkdown(event: Event) {
    const input = event.currentTarget as HTMLInputElement;
    const file = input.files?.[0];
    uploadedFileName = '';
    if (!file) return;
    const isZip = file.name.toLowerCase().endsWith('.zip');
    if (!isZip && !file.name.toLowerCase().endsWith('.md') && file.type !== 'text/markdown') { errorMessage = '请选择 .md 文件或包含 Markdown 与图片的 .zip 图文包'; input.value = ''; return; }
    if (!isZip && file.size > 2 * 1024 * 1024) { errorMessage = 'Markdown 文件不能超过 2 MB'; input.value = ''; return; }
    uploading = true;
    try {
      if (isZip) await importBundle(file);
      else {
        const parsed = frontMatter(await file.text());
        applyFrontMatter(parsed.values);
        body = parsed.content;
      }
      uploadedFileName = file.name;
      errorMessage = '';
    } catch (error) { errorMessage = error instanceof Error ? error.message : '文件读取失败'; }
    finally { uploading = false; input.value = ''; }
  }

  function insertAtCursor(before: string, after = '', placeholder = '内容') {
    const start = bodyEditor?.selectionStart ?? body.length;
    const end = bodyEditor?.selectionEnd ?? body.length;
    const selected = body.slice(start, end) || placeholder;
    body = `${body.slice(0, start)}${before}${selected}${after}${body.slice(end)}`;
    requestAnimationFrame(() => { bodyEditor?.focus(); const cursor = start + before.length + selected.length + after.length; bodyEditor?.setSelectionRange(cursor, cursor); });
  }

  function insertLink() { insertAtCursor('[', '](https://)', '链接文字'); }
  function insertImageSyntax() { insertAtCursor('![', '](/uploads/图片地址)', '图片说明'); }

  async function uploadImage(event: Event) {
    const input = event.currentTarget as HTMLInputElement;
    const file = input.files?.[0];
    input.value = '';
    if (!file) return;
    uploading = true; errorMessage = '';
    try {
      const uploadedUrl = await uploadImageFile(file);
      const start = bodyEditor?.selectionStart ?? body.length;
      body = `${body.slice(0, start)}![${file.name.replace(/\.[^.]+$/, '')}](${uploadedUrl})\n${body.slice(start)}`;
      requestAnimationFrame(() => bodyEditor?.focus());
    } catch (error) { errorMessage = error instanceof Error ? error.message : '图片上传失败'; }
    finally { uploading = false; }
  }

  async function submit() {
    if (body.trim().length < 20) { errorMessage = '正文至少需要 20 个字符'; return; }
    if (body.length > 500_000) { errorMessage = '正文不能超过 500000 个字符'; return; }
    submitting = true; result = ''; errorMessage = '';
    try {
      const response = await fetch('/api/v1/submissions', { method: 'POST', headers: { 'content-type': 'application/json' }, body: JSON.stringify({ title, category: category || data.categories[0]?.slug, year: year ? Number(year) : null, background, contact, body_markdown: body, consent, website }) });
      const payload = await response.json();
      if (!response.ok) throw new Error(payload.error || '提交失败');
      result = `提交成功，编号为 ${payload.reference_code}。维护者审核后会根据你留下的联系方式沟通。`;
      title = ''; year = String(new Date().getFullYear()); background = ''; contact = ''; body = ''; consent = false; uploadedFileName = ''; mode = 'edit';
    } catch (error) { errorMessage = error instanceof Error ? error.message : '提交失败，请稍后重试'; }
    finally { submitting = false; }
  }
</script>

<svelte:head><title>投稿 | 信工所考研信息站</title><meta name="description" content="提交 Markdown 经验稿件，支持自动识别文章信息、上传图片和实时预览。" /></svelte:head>

<section class="page-heading"><p class="eyebrow">参与维护</p><h1 class="page-title">写一篇完整的经验稿</h1><p class="page-lead">支持导入 Markdown 文件或带图片的 ZIP 图文包，也可以直接上传图片并预览效果。稿件提交后会进入审核，默认年份为当前年度。</p></section>

<section class="section contribute-section">
  <div class="contribute-grid">
    <aside class="contribute-aside"><div class="aside-icon"><FilePlus2 size={21} /></div><h2>投稿信息</h2><p>标题、年份和摘要会进入内容索引；正文中的远程图片与本站上传图片都会保留。</p><div class="limits"><span><Paperclip size={13} />Markdown 文件 ≤ 2 MB</span><span><ImagePlus size={13} />单张图片 ≤ 5 MB</span><span><Code2 size={13} />正文 ≤ 500000 字符</span></div><a href="/articles">先查看已有内容 <Send size={14} /></a></aside>
    <form class="surface form" onsubmit={(event) => { event.preventDefault(); submit(); }}>
      {#if result}<div class="success-box"><CheckCircle2 size={16} />{result}</div>{/if}
      {#if errorMessage}<div class="error-box">{errorMessage}</div>{/if}
      <div class="form-grid">
        <div class="field full"><label for="title">标题</label><input class="input" id="title" bind:value={title} required minlength="2" maxlength="120" placeholder="例如：2026 复试准备与项目表达复盘" /></div>
        <div class="field"><label for="category">分类</label><select class="select" id="category" bind:value={category}>{#each data.categories as item}<option value={item.slug}>{item.name}</option>{/each}</select></div>
        <div class="field"><label for="year">相关年份 <small>缺省为当前年</small></label><input class="input" id="year" type="number" min="2010" max="2100" bind:value={year} /></div>
        <div class="field"><label for="background">摘要 / 背景 <small>可选</small></label><input class="input" id="background" bind:value={background} maxlength="240" placeholder="会显示在文章列表中" /></div>
        <div class="field"><label for="contact">审核联系 <small>可选，不公开</small></label><input class="input" id="contact" bind:value={contact} maxlength="240" placeholder="邮箱或其他方式" /></div>
        <div class="field full markdown-upload"><label for="markdown-file">Markdown 文件</label><div class="upload-row"><label class="button upload-button" for="markdown-file"><Upload size={16} />{uploading ? '处理中' : '导入 .md / .zip'}</label><input id="markdown-file" type="file" accept=".md,.zip,text/markdown,application/zip" onchange={readMarkdown} disabled={uploading} />{#if uploadedFileName}<span class="upload-name"><FileText size={15} />{uploadedFileName}</span>{:else}<span class="upload-help">ZIP 可包含 Markdown 引用的本地图片目录</span>{/if}</div></div>
        <div class="field full editor-field"><div class="editor-head"><label for="body">正文（Markdown）</label><div class="mode-tabs"><button type="button" class:active={mode === 'edit'} onclick={() => (mode = 'edit')}>编辑</button><button type="button" class:active={mode === 'preview'} onclick={() => (mode = 'preview')}>预览</button></div></div>{#if mode === 'edit'}<div class="toolbar"><button type="button" title="标题" onclick={() => insertAtCursor('## ', '', '小标题')}><Heading2 size={15} /></button><button type="button" title="粗体" onclick={() => insertAtCursor('**', '**')}><Bold size={15} /></button><button type="button" title="斜体" onclick={() => insertAtCursor('*', '*')}><Italic size={15} /></button><button type="button" title="引用" onclick={() => insertAtCursor('> ', '', '引用')}><Quote size={15} /></button><button type="button" title="代码" onclick={() => insertAtCursor('`', '`')}><Code2 size={15} /></button><button type="button" title="公式" onclick={() => insertAtCursor('\n$$\n', '\n$$\n', 'E = mc^2')}><Sigma size={15} /></button><button type="button" title="列表" onclick={() => insertAtCursor('- ', '', '列表项')}><List size={15} /></button><button type="button" title="链接" onclick={insertLink}><Link2 size={15} /></button><label class="toolbar-upload" title="上传图片" for="image-file"><ImagePlus size={15} />{uploading ? '上传中' : '图片'}</label><input id="image-file" type="file" accept="image/png,image/jpeg,image/webp,image/gif" onchange={uploadImage} disabled={uploading} /></div><textarea class="textarea editor" id="body" bind:this={bodyEditor} bind:value={body} required minlength="20" maxlength="500000" placeholder="从背景、时间线、具体做法、结果和不适用边界写起。"></textarea>{:else}<div class="preview prose" aria-live="polite">{@html renderMarkdown(body || '*暂无正文*')}</div>{/if}</div>
        {#if unresolvedImages.length}<div class="field full image-warning">有 {unresolvedImages.length} 个相对路径图片未上传：{unresolvedImages.slice(0, 3).join('、')}。提交后这些路径不会自动从本机迁移。</div>{/if}
        <div class="field full"><label class="check-row"><input type="checkbox" bind:checked={consent} required />我确认内容可以公开，已处理个人隐私并拥有必要授权。</label></div>
        <div class="field honeypot" aria-hidden="true"><label for="website">Website</label><input id="website" tabindex="-1" autocomplete="off" bind:value={website} /></div>
        <div class="field full"><button class="button primary" type="submit" disabled={submitting || uploading}>{#if submitting}提交中...{:else}<Send size={16} />提交审核{/if}</button></div>
      </div>
    </form>
  </div>
</section>

<style>
  .contribute-section { padding-top: 42px; } .contribute-grid { display: grid; grid-template-columns: minmax(230px, .58fr) minmax(0, 1.42fr); align-items: start; gap: 46px; } .contribute-aside { padding: 6px 0; } .aside-icon { display: grid; width: 44px; height: 44px; margin-bottom: 19px; place-items: center; border-radius: 8px; background: var(--accent-soft); color: var(--accent); } .contribute-aside h2 { margin: 0; font-size: 20px; } .contribute-aside p { margin: 12px 0 18px; color: var(--muted); font-size: 12px; line-height: 1.75; } .limits { display: grid; margin-bottom: 22px; gap: 9px; } .limits span { display: flex; align-items: center; color: var(--accent-dark); gap: 7px; font-size: 11px; font-weight: 700; } .contribute-aside a { display: inline-flex; align-items: center; color: var(--accent); gap: 5px; font-size: 12px; font-weight: 750; }
  .form { padding: 28px; box-shadow: var(--shadow); } .form .success-box, .form .error-box { display: flex; margin-bottom: 18px; align-items: flex-start; gap: 8px; } .upload-row { display: flex; min-height: 42px; align-items: center; gap: 12px; } .upload-row input, .toolbar-upload + input { position: absolute; width: 1px; height: 1px; overflow: hidden; opacity: 0; } .upload-button { cursor: pointer; } .upload-name, .upload-help { display: inline-flex; min-width: 0; align-items: center; color: var(--muted); gap: 6px; font-size: 12px; } .upload-name { color: var(--accent-dark); font-weight: 700; overflow-wrap: anywhere; }
  .editor-field { min-width: 0; } .editor-head { display: flex; align-items: center; justify-content: space-between; } .mode-tabs { display: flex; gap: 4px; } .mode-tabs button { min-height: 30px; padding: 0 10px; border: 1px solid var(--line); border-radius: 6px; background: white; color: var(--muted); cursor: pointer; font-size: 10px; font-weight: 750; } .mode-tabs button.active { border-color: var(--accent); background: var(--accent); color: white; } .toolbar { display: flex; margin-top: 9px; padding: 7px; align-items: center; border: 1px solid #cbd5e1; border-bottom: 0; border-radius: 8px 8px 0 0; background: var(--soft); gap: 3px; } .toolbar button, .toolbar-upload { display: inline-flex; width: 30px; height: 30px; padding: 0; align-items: center; justify-content: center; border: 0; border-radius: 5px; background: transparent; color: #526174; cursor: pointer; gap: 4px; } .toolbar button:hover, .toolbar-upload:hover { background: white; color: var(--accent); } .toolbar-upload { width: auto; padding: 0 7px; font-size: 10px; font-weight: 750; } .editor { min-height: 430px; border-radius: 0 0 8px 8px; font-family: ui-monospace, SFMono-Regular, Consolas, monospace; line-height: 1.75; } .preview { min-height: 430px; max-width: none; margin-top: 9px; padding: 20px; border: 1px solid #cbd5e1; border-radius: 8px; overflow: auto; } .image-warning { padding: 10px 12px; border-left: 3px solid var(--amber); background: #fff8e9; color: #6b542d; font-size: 11px; line-height: 1.6; } .honeypot { position: absolute; left: -10000px; width: 1px; height: 1px; overflow: hidden; }
  @media (max-width: 760px) { .contribute-grid { grid-template-columns: 1fr; } } @media (max-width: 480px) { .upload-row { align-items: flex-start; flex-direction: column; gap: 8px; } .toolbar { overflow-x: auto; } }
</style>
