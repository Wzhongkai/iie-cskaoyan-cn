import type { PageServerLoad } from './$types';
import { apiRequest } from '$server/api';
import type { AnnualReportDetail, AnnualReportOverview, Article } from '$lib/types';

export const load: PageServerLoad = async () => {
  const [report, reports, recent] = await Promise.all([
    apiRequest<AnnualReportDetail>('/api/v1/reports/latest').catch(() => null),
    apiRequest<AnnualReportOverview[]>('/api/v1/reports').catch(() => []),
    apiRequest<Article[]>('/api/v1/articles?limit=4').catch(() => [])
  ]);

  return { report, reports: reports.slice(0, 3), recent };
};
