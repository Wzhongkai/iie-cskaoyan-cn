import type { PageServerLoad } from './$types';
import { apiRequest } from '$server/api';
import type { Article } from '$lib/types';

export const load: PageServerLoad = async ({ url }) => {
  const query = new URLSearchParams({ limit: '100' });
  const category = url.searchParams.get('category');
  const q = url.searchParams.get('q');
  if (category) query.set('category', category);
  if (q) query.set('q', q);
  const articles = await apiRequest<Article[]>(`/api/v1/articles?${query.toString()}`).catch(() => []);
  return { articles, category, q };
};
