use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, FromRow)]
pub(crate) struct Article {
    pub(crate) id: Uuid,
    pub(crate) slug: String,
    pub(crate) title: String,
    pub(crate) excerpt: Option<String>,
    pub(crate) body_markdown: String,
    pub(crate) category: String,
    pub(crate) year: Option<i32>,
    pub(crate) status: String,
    pub(crate) is_pinned: bool,
    pub(crate) created_at: DateTime<Utc>,
    pub(crate) updated_at: DateTime<Utc>,
    pub(crate) published_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, FromRow)]
pub(crate) struct AdmissionStat {
    pub(crate) year: i32,
    pub(crate) program: String,
    pub(crate) applicants: Option<i32>,
    pub(crate) cutoff: i32,
    pub(crate) interviewed: i32,
    pub(crate) admitted: i32,
    pub(crate) rate: f64,
    pub(crate) source_note: Option<String>,
}

#[derive(Debug, Serialize, FromRow)]
pub(crate) struct Submission {
    pub(crate) id: Uuid,
    pub(crate) reference_code: String,
    pub(crate) title: String,
    pub(crate) category: String,
    pub(crate) year: Option<i32>,
    pub(crate) background: Option<String>,
    pub(crate) contact: Option<String>,
    pub(crate) body_markdown: String,
    pub(crate) status: String,
    pub(crate) created_at: DateTime<Utc>,
    pub(crate) reviewed_at: Option<DateTime<Utc>>,
    pub(crate) published_article_id: Option<Uuid>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct ArticleQuery {
    pub(crate) category: Option<String>,
    pub(crate) q: Option<String>,
    pub(crate) limit: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct SubmissionInput {
    pub(crate) title: String,
    pub(crate) category: String,
    pub(crate) year: Option<i32>,
    pub(crate) background: Option<String>,
    pub(crate) contact: Option<String>,
    pub(crate) body_markdown: String,
    pub(crate) consent: bool,
    pub(crate) website: Option<String>,
}

#[derive(Debug, Serialize)]
pub(crate) struct SubmissionReceipt {
    pub(crate) reference_code: String,
}

#[derive(Debug, Serialize)]
pub(crate) struct UploadReceipt {
    pub(crate) url: String,
    pub(crate) filename: String,
    pub(crate) size: usize,
}

#[derive(Debug, Deserialize)]
pub(crate) struct StatusUpdate {
    pub(crate) status: String,
}

#[derive(Debug, Deserialize)]
pub(crate) struct SubmissionQuery {
    pub(crate) status: Option<String>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct ArticleInput {
    pub(crate) slug: String,
    pub(crate) title: String,
    pub(crate) excerpt: Option<String>,
    pub(crate) body_markdown: String,
    pub(crate) category: String,
    pub(crate) year: Option<i32>,
    pub(crate) status: String,
    #[serde(default)]
    pub(crate) is_pinned: bool,
}

#[derive(Debug, Serialize, FromRow)]
pub(crate) struct AnnualReport {
    pub(crate) year: i32,
    pub(crate) title: String,
    pub(crate) exam_applicants_min: Option<i32>,
    pub(crate) applicants_note: Option<String>,
    pub(crate) national_total_cutoff: Option<i32>,
    pub(crate) national_politics_english_cutoff: Option<i32>,
    pub(crate) national_subject_cutoff: Option<i32>,
    pub(crate) academic_cutoff: Option<i32>,
    pub(crate) professional_cutoff: Option<i32>,
    pub(crate) interviewed_total: Option<i32>,
    pub(crate) admitted_total: Option<i32>,
    pub(crate) academic_admitted: Option<i32>,
    pub(crate) professional_admitted: Option<i32>,
    pub(crate) recommendation_total: Option<i32>,
    pub(crate) direct_phd: Option<i32>,
    pub(crate) recommendation_academic: Option<i32>,
    pub(crate) recommendation_professional: Option<i32>,
    pub(crate) exam_source_sample: Option<i32>,
    pub(crate) exam_source_coverage: Option<f64>,
    pub(crate) score_formula: Option<String>,
    pub(crate) source_file: String,
    pub(crate) source_note: String,
    pub(crate) updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, FromRow)]
pub(crate) struct SchoolTierStat {
    pub(crate) track: String,
    pub(crate) tier: String,
    pub(crate) admitted: i32,
    pub(crate) percentage: f64,
}

#[derive(Debug, Serialize, FromRow)]
pub(crate) struct SchoolStat {
    pub(crate) track: String,
    pub(crate) tier: String,
    pub(crate) school: String,
    pub(crate) admitted: i32,
}

#[derive(Debug, Serialize, FromRow)]
pub(crate) struct SubjectStat {
    pub(crate) program: String,
    pub(crate) phase: String,
    pub(crate) subject: String,
    pub(crate) highest: f64,
    pub(crate) lowest: f64,
    pub(crate) average: f64,
    pub(crate) median: f64,
}

#[derive(Debug, Serialize, FromRow)]
pub(crate) struct ScoreBandStat {
    pub(crate) program: String,
    pub(crate) band: String,
    pub(crate) band_order: i32,
    pub(crate) interviewed: i32,
    pub(crate) admitted: i32,
    pub(crate) cumulative_interviewed: i32,
    pub(crate) cumulative_admitted: i32,
    pub(crate) note: Option<String>,
}

#[derive(Debug, Serialize, FromRow)]
pub(crate) struct LabStat {
    pub(crate) program: String,
    pub(crate) lab: i32,
    pub(crate) admitted: i32,
    pub(crate) rejected: i32,
    pub(crate) first_choice: i32,
    pub(crate) highest: Option<f64>,
    pub(crate) lowest: Option<f64>,
    pub(crate) average: Option<f64>,
    pub(crate) median: Option<f64>,
    pub(crate) note: Option<String>,
}

#[derive(Debug, Serialize)]
pub(crate) struct AnnualReportDetail {
    pub(crate) overview: AnnualReport,
    pub(crate) school_tiers: Vec<SchoolTierStat>,
    pub(crate) schools: Vec<SchoolStat>,
    pub(crate) subjects: Vec<SubjectStat>,
    pub(crate) score_bands: Vec<ScoreBandStat>,
    pub(crate) labs: Vec<LabStat>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct AnnualReportInput {
    pub(crate) year: i32,
    pub(crate) title: String,
    pub(crate) exam_applicants_min: Option<i32>,
    pub(crate) applicants_note: Option<String>,
    pub(crate) national_total_cutoff: Option<i32>,
    pub(crate) national_politics_english_cutoff: Option<i32>,
    pub(crate) national_subject_cutoff: Option<i32>,
    pub(crate) academic_cutoff: Option<i32>,
    pub(crate) professional_cutoff: Option<i32>,
    pub(crate) interviewed_total: Option<i32>,
    pub(crate) admitted_total: Option<i32>,
    pub(crate) academic_admitted: Option<i32>,
    pub(crate) professional_admitted: Option<i32>,
    pub(crate) recommendation_total: Option<i32>,
    pub(crate) direct_phd: Option<i32>,
    pub(crate) recommendation_academic: Option<i32>,
    pub(crate) recommendation_professional: Option<i32>,
    pub(crate) exam_source_sample: Option<i32>,
    pub(crate) exam_source_coverage: Option<f64>,
    pub(crate) score_formula: Option<String>,
    pub(crate) source_file: String,
    pub(crate) source_note: String,
}

#[derive(Debug, Deserialize)]
pub(crate) struct AdmissionStatInput {
    pub(crate) year: i32,
    pub(crate) program: String,
    pub(crate) applicants: Option<i32>,
    pub(crate) cutoff: i32,
    pub(crate) interviewed: i32,
    pub(crate) admitted: i32,
    pub(crate) rate: f64,
    pub(crate) source_note: Option<String>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct SchoolTierInput {
    pub(crate) year: i32,
    pub(crate) track: String,
    pub(crate) tier: String,
    pub(crate) admitted: i32,
    pub(crate) percentage: f64,
}

#[derive(Debug, Deserialize)]
pub(crate) struct SchoolInput {
    pub(crate) year: i32,
    pub(crate) track: String,
    pub(crate) tier: String,
    pub(crate) school: String,
    pub(crate) admitted: i32,
}

#[derive(Debug, Deserialize)]
pub(crate) struct SubjectInput {
    pub(crate) year: i32,
    pub(crate) program: String,
    pub(crate) phase: String,
    pub(crate) subject: String,
    pub(crate) highest: f64,
    pub(crate) lowest: f64,
    pub(crate) average: f64,
    pub(crate) median: f64,
}

#[derive(Debug, Deserialize)]
pub(crate) struct ScoreBandInput {
    pub(crate) year: i32,
    pub(crate) program: String,
    pub(crate) band: String,
    pub(crate) band_order: i32,
    pub(crate) interviewed: i32,
    pub(crate) admitted: i32,
    pub(crate) cumulative_interviewed: i32,
    pub(crate) cumulative_admitted: i32,
    pub(crate) note: Option<String>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct LabInput {
    pub(crate) year: i32,
    pub(crate) program: String,
    pub(crate) lab: i32,
    pub(crate) admitted: i32,
    pub(crate) rejected: i32,
    pub(crate) first_choice: i32,
    pub(crate) highest: Option<f64>,
    pub(crate) lowest: Option<f64>,
    pub(crate) average: Option<f64>,
    pub(crate) median: Option<f64>,
    pub(crate) note: Option<String>,
}
