import type { PageServerLoad } from './$types';
import { apiRequest } from '$server/api';
import type { Article, Category } from '$lib/types';

export const load: PageServerLoad = async ({ url }) => {
  const query = new URLSearchParams({ limit: '100' });
  const category = url.searchParams.get('category');
  const q = url.searchParams.get('q');
  if (category) query.set('category', category);
  if (q) query.set('q', q);
  const [articles, categories] = await Promise.all([
    apiRequest<Article[]>(`/api/v1/articles?${query.toString()}`).catch(() => []),
    apiRequest<Category[]>('/api/v1/categories').catch(() => [])
  ]);
  return { articles, categories, category, q };
};
