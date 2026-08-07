import { error } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';
import { apiRequest } from '$server/api';
import type { AnnualReportDetail } from '$lib/types';

export const load: PageServerLoad = async ({ params }) => {
  const year = Number(params.year);
  if (!Number.isInteger(year) || year < 2010 || year > 2100) error(404, '年度不存在');
  const report = await apiRequest<AnnualReportDetail>(`/api/v1/reports/${year}`).catch(() => null);
  if (!report) error(404, '未找到该年度报告');
  return { report };
};
