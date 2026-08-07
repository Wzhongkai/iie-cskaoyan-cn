export type ArticleCategory = 'initial' | 'reexam' | 'career' | 'policy' | 'data';

export type Article = {
  id: string;
  slug: string;
  title: string;
  excerpt: string | null;
  body_markdown: string;
  category: ArticleCategory;
  year: number | null;
  status: 'draft' | 'published' | 'archived';
  is_pinned: boolean;
  created_at: string;
  updated_at: string;
  published_at: string | null;
};

export type AdmissionStat = {
  year: number;
  program: string;
  applicants: number | null;
  cutoff: number;
  interviewed: number;
  admitted: number;
  rate: number;
  source_note: string | null;
};

export type Submission = {
  id: string;
  reference_code: string;
  title: string;
  category: ArticleCategory;
  year: number | null;
  background: string | null;
  contact: string | null;
  body_markdown: string;
  status: 'pending' | 'approved' | 'rejected';
  created_at: string;
  reviewed_at: string | null;
  published_article_id: string | null;
};

export type ApiResponse<T> = { data: T } | { error: string };

export type AnnualReportOverview = {
  year: number;
  title: string;
  exam_applicants_min: number | null;
  applicants_note: string | null;
  national_total_cutoff: number | null;
  national_politics_english_cutoff: number | null;
  national_subject_cutoff: number | null;
  academic_cutoff: number | null;
  professional_cutoff: number | null;
  interviewed_total: number | null;
  admitted_total: number | null;
  academic_admitted: number | null;
  professional_admitted: number | null;
  recommendation_total: number | null;
  direct_phd: number | null;
  recommendation_academic: number | null;
  recommendation_professional: number | null;
  exam_source_sample: number | null;
  exam_source_coverage: number | null;
  score_formula: string | null;
  source_file: string;
  source_note: string;
  updated_at: string;
};

export type SchoolTierStat = {
  track: 'recommendation' | 'exam';
  tier: '985' | '211' | 'non_211';
  admitted: number;
  percentage: number;
};

export type SchoolStat = {
  track: 'recommendation' | 'exam';
  tier: '985' | '211' | 'non_211';
  school: string;
  admitted: number;
};

export type SubjectStat = {
  program: 'academic' | 'professional' | 'combined';
  phase: 'initial_subject' | 'admitted_total';
  subject: string;
  highest: number;
  lowest: number;
  average: number;
  median: number;
};

export type ScoreBandStat = {
  program: 'academic' | 'professional' | 'combined';
  band: string;
  band_order: number;
  interviewed: number;
  admitted: number;
  cumulative_interviewed: number;
  cumulative_admitted: number;
  note: string | null;
};

export type LabStat = {
  program: 'academic' | 'professional' | 'combined';
  lab: number;
  admitted: number;
  rejected: number;
  first_choice: number;
  highest: number | null;
  lowest: number | null;
  average: number | null;
  median: number | null;
  note: string | null;
};

export type AnnualReportDetail = {
  overview: AnnualReportOverview;
  school_tiers: SchoolTierStat[];
  schools: SchoolStat[];
  subjects: SubjectStat[];
  score_bands: ScoreBandStat[];
  labs: LabStat[];
};
