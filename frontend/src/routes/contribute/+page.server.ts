import type { PageServerLoad } from './$types';
import { apiRequest } from '$server/api';
import type { Category } from '$lib/types';

export const load: PageServerLoad = async () => ({
  categories: await apiRequest<Category[]>('/api/v1/categories').catch(() => [])
});
