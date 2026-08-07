import { error } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';
import { apiRequest } from '$server/api';
import { renderMarkdown } from '$server/markdown';
import type { Article } from '$lib/types';

export const load: PageServerLoad = async ({ params }) => {
  try {
    const article = await apiRequest<Article>(`/api/v1/articles/${encodeURIComponent(params.slug)}`);
    return { article, html: renderMarkdown(article.body_markdown) };
  } catch {
    throw error(404, '文章不存在或尚未发布');
  }
};
