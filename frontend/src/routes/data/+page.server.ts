import type { PageServerLoad } from './$types';
import { apiRequest } from '$server/api';
import type { AnnualReportOverview } from '$lib/types';

export const load: PageServerLoad = async () => ({
  reports: await apiRequest<AnnualReportOverview[]>('/api/v1/reports').catch(() => [])
});
