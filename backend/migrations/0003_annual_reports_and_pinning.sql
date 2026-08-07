ALTER TABLE articles ADD COLUMN IF NOT EXISTS is_pinned BOOLEAN NOT NULL DEFAULT FALSE;

CREATE TABLE annual_reports (
    year INTEGER PRIMARY KEY CHECK (year BETWEEN 2010 AND 2100),
    title TEXT NOT NULL,
    exam_applicants_min INTEGER,
    applicants_note TEXT,
    national_total_cutoff INTEGER,
    national_politics_english_cutoff INTEGER,
    national_subject_cutoff INTEGER,
    academic_cutoff INTEGER,
    professional_cutoff INTEGER,
    interviewed_total INTEGER,
    admitted_total INTEGER,
    academic_admitted INTEGER,
    professional_admitted INTEGER,
    recommendation_total INTEGER,
    direct_phd INTEGER,
    recommendation_academic INTEGER,
    recommendation_professional INTEGER,
    exam_source_sample INTEGER,
    exam_source_coverage DOUBLE PRECISION,
    score_formula TEXT,
    source_file TEXT NOT NULL,
    source_note TEXT NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE report_school_tiers (
    year INTEGER NOT NULL REFERENCES annual_reports(year) ON DELETE CASCADE,
    track TEXT NOT NULL CHECK (track IN ('recommendation', 'exam')),
    tier TEXT NOT NULL CHECK (tier IN ('985', '211', 'non_211')),
    admitted INTEGER NOT NULL CHECK (admitted >= 0),
    percentage DOUBLE PRECISION NOT NULL CHECK (percentage >= 0 AND percentage <= 100),
    PRIMARY KEY (year, track, tier)
);

CREATE TABLE report_schools (
    year INTEGER NOT NULL REFERENCES annual_reports(year) ON DELETE CASCADE,
    track TEXT NOT NULL CHECK (track IN ('recommendation', 'exam')),
    tier TEXT NOT NULL CHECK (tier IN ('985', '211', 'non_211')),
    school TEXT NOT NULL,
    admitted INTEGER NOT NULL CHECK (admitted > 0),
    PRIMARY KEY (year, track, school)
);

CREATE TABLE report_subject_stats (
    year INTEGER NOT NULL REFERENCES annual_reports(year) ON DELETE CASCADE,
    program TEXT NOT NULL,
    phase TEXT NOT NULL CHECK (phase IN ('initial_subject', 'admitted_total')),
    subject TEXT NOT NULL,
    highest DOUBLE PRECISION NOT NULL,
    lowest DOUBLE PRECISION NOT NULL,
    average DOUBLE PRECISION NOT NULL,
    median DOUBLE PRECISION NOT NULL,
    PRIMARY KEY (year, program, phase, subject)
);

CREATE TABLE report_score_bands (
    year INTEGER NOT NULL REFERENCES annual_reports(year) ON DELETE CASCADE,
    program TEXT NOT NULL,
    band TEXT NOT NULL,
    band_order INTEGER NOT NULL,
    interviewed INTEGER NOT NULL,
    admitted INTEGER NOT NULL,
    cumulative_interviewed INTEGER NOT NULL,
    cumulative_admitted INTEGER NOT NULL,
    note TEXT,
    PRIMARY KEY (year, program, band)
);

CREATE TABLE report_lab_stats (
    year INTEGER NOT NULL REFERENCES annual_reports(year) ON DELETE CASCADE,
    program TEXT NOT NULL,
    lab INTEGER NOT NULL CHECK (lab BETWEEN 1 AND 20),
    admitted INTEGER NOT NULL,
    rejected INTEGER NOT NULL,
    first_choice INTEGER NOT NULL,
    highest DOUBLE PRECISION,
    lowest DOUBLE PRECISION,
    average DOUBLE PRECISION,
    median DOUBLE PRECISION,
    note TEXT,
    PRIMARY KEY (year, program, lab)
);

INSERT INTO annual_reports (
    year, title, exam_applicants_min, applicants_note, national_total_cutoff,
    national_politics_english_cutoff, national_subject_cutoff, academic_cutoff,
    professional_cutoff, interviewed_total, admitted_total, academic_admitted,
    professional_admitted, recommendation_total, direct_phd,
    recommendation_academic, recommendation_professional, exam_source_sample,
    exam_source_coverage, score_formula, source_file, source_note
) VALUES
    (2026, '2026 年信息工程研究所保研考研数据报告', 770, '报告表述为约 770 人', 264, 35, 53, 315, 318, 216, 145, 55, 90, 248, 108, 94, 46, 134, 92.4, '总成绩 = 初试总分 / 10 + 复试成绩 / 2', '2026信息工程研究所保研考研数据报告.pdf', '2026 级同学整理，非信工所官方统计'),
    (2025, '2025 年信工所考研与保研数据报告', 560, '报告表述为超过 560 人', 260, 34, 51, 275, 261, 211, 137, 59, 78, 247, 95, 92, 60, 130, 91.0, '总成绩 = 初试总分 / 10 + 复试成绩 / 2', '25新版信工所考研与保研数据报告  (2).pdf', '2025 级同学整理，非信工所官方统计'),
    (2024, '2024 年信工所保研考研数据报告', 790, '报告表述为 790 人以上，估计 800 出头', 273, 37, 56, 273, 273, 290, 150, 56, 94, 206, 68, 96, 42, 86, 57.0, '总成绩 = 初试总分 / 10 + 复试成绩 / 2', '24信工所保研考研数据(正式版) (1).pdf', '2024 级同学整理，非信工所官方统计')
ON CONFLICT (year) DO NOTHING;

INSERT INTO report_school_tiers (year, track, tier, admitted, percentage) VALUES
    (2026, 'recommendation', '985', 85, 34.27), (2026, 'recommendation', '211', 97, 39.11), (2026, 'recommendation', 'non_211', 66, 26.61),
    (2026, 'exam', '985', 44, 32.84), (2026, 'exam', '211', 35, 26.12), (2026, 'exam', 'non_211', 55, 41.04),
    (2025, 'recommendation', '985', 69, 27.9), (2025, 'recommendation', '211', 90, 36.4), (2025, 'recommendation', 'non_211', 88, 35.6),
    (2025, 'exam', '985', 39, 30.0), (2025, 'exam', '211', 45, 34.62), (2025, 'exam', 'non_211', 46, 35.38),
    (2024, 'recommendation', '985', 76, 36.9), (2024, 'recommendation', '211', 69, 33.5), (2024, 'recommendation', 'non_211', 61, 29.6),
    (2024, 'exam', '985', 34, 39.5), (2024, 'exam', '211', 27, 31.5), (2024, 'exam', 'non_211', 25, 29.0)
ON CONFLICT DO NOTHING;

INSERT INTO report_schools (year, track, tier, school, admitted) VALUES
    (2026,'recommendation','985','中国科学院大学',19),(2026,'recommendation','985','北京大学',9),(2026,'recommendation','985','山东大学',9),(2026,'recommendation','985','西北工业大学',4),(2026,'recommendation','985','哈尔滨工业大学',3),(2026,'recommendation','985','南开大学',3),(2026,'recommendation','985','中央民族大学',3),(2026,'recommendation','985','四川大学',3),(2026,'recommendation','985','东南大学',2),(2026,'recommendation','985','中国农业大学',2),(2026,'recommendation','985','中国海洋大学',2),(2026,'recommendation','985','东北大学',2),(2026,'recommendation','985','湖南大学',2),(2026,'recommendation','985','电子科技大学',2),(2026,'recommendation','985','清华大学',2),(2026,'recommendation','985','武汉大学',2),(2026,'recommendation','985','重庆大学',2),(2026,'recommendation','985','天津大学',2),(2026,'recommendation','985','大连理工大学',2),(2026,'recommendation','985','华中科技大学',2),(2026,'recommendation','985','华东师范大学',1),(2026,'recommendation','985','北京航空航天大学',1),(2026,'recommendation','985','中山大学',1),(2026,'recommendation','985','中国科学技术大学',1),(2026,'recommendation','985','上海交通大学',1),(2026,'recommendation','985','吉林大学',1),(2026,'recommendation','985','厦门大学',1),(2026,'recommendation','985','西安交通大学',1),
    (2026,'recommendation','211','北京科技大学',12),(2026,'recommendation','211','西安电子科技大学',10),(2026,'recommendation','211','北京工业大学',6),(2026,'recommendation','211','北京交通大学',6),(2026,'recommendation','211','北京邮电大学',5),(2026,'recommendation','211','中国石油大学（华东）',4),(2026,'recommendation','211','中国传媒大学',3),(2026,'recommendation','211','武汉理工大学',3),(2026,'recommendation','211','北京林业大学',3),(2026,'recommendation','211','太原理工大学',3),(2026,'recommendation','211','河北工业大学',3),(2026,'recommendation','211','东北师范大学',2),(2026,'recommendation','211','中国矿业大学',2),(2026,'recommendation','211','东华大学',2),(2026,'recommendation','211','中国地质大学（武汉）',2),(2026,'recommendation','211','哈尔滨工程大学',2),(2026,'recommendation','211','福州大学',2),(2026,'recommendation','211','郑州大学',2),(2026,'recommendation','211','安徽大学',2),(2026,'recommendation','211','南京航空航天大学',2),(2026,'recommendation','211','南昌大学',2),(2026,'recommendation','211','华北电力大学',2),(2026,'recommendation','211','中南财经政法大学',1),(2026,'recommendation','211','合肥工业大学',1),(2026,'recommendation','211','华南师范大学',1),(2026,'recommendation','211','华中师范大学',1),(2026,'recommendation','211','华北电力大学（保定）',1),(2026,'recommendation','211','华中农业大学',1),(2026,'recommendation','211','内蒙古大学',1),(2026,'recommendation','211','中国矿业大学（北京）',1),(2026,'recommendation','211','云南大学',1),(2026,'recommendation','211','湖南师范大学',1),(2026,'recommendation','211','河海大学',1),(2026,'recommendation','211','宁夏大学',1),(2026,'recommendation','211','石河子大学',1),(2026,'recommendation','211','西南大学',1),(2026,'recommendation','211','苏州大学',1),(2026,'recommendation','211','西南财经大学',1),(2026,'recommendation','211','贵州大学',1),
    (2026,'recommendation','non_211','青岛大学',7),(2026,'recommendation','non_211','北方工业大学',6),(2026,'recommendation','non_211','河南大学',5),(2026,'recommendation','non_211','广州大学',4),(2026,'recommendation','non_211','湖北大学',3),(2026,'recommendation','non_211','南方科技大学',2),(2026,'recommendation','non_211','中北大学',2),(2026,'recommendation','non_211','河南理工大学',2),(2026,'recommendation','non_211','南京林业大学',2),(2026,'recommendation','non_211','山东理工大学',2),(2026,'recommendation','non_211','南京信息工程大学',2),(2026,'recommendation','non_211','南京邮电大学',2),(2026,'recommendation','non_211','广东工业大学',1),(2026,'recommendation','non_211','山东科技大学',1),(2026,'recommendation','non_211','桂林电子科技大学',1),(2026,'recommendation','non_211','河南科技大学',1),(2026,'recommendation','non_211','浙江工业大学',1),(2026,'recommendation','non_211','河北农业大学',1),(2026,'recommendation','non_211','渤海大学',1),(2026,'recommendation','non_211','湘潭大学',1),(2026,'recommendation','non_211','福建师范大学',1),(2026,'recommendation','non_211','燕山大学',1),(2026,'recommendation','non_211','西南石油大学',1),(2026,'recommendation','non_211','辽宁工程技术大学',1),(2026,'recommendation','non_211','首都师范大学',1),(2026,'recommendation','non_211','北京语言大学',1),(2026,'recommendation','non_211','黑龙江大学',1),(2026,'recommendation','non_211','东北电力大学',1),(2026,'recommendation','non_211','东北石油大学',1),(2026,'recommendation','non_211','中南民族大学',1),(2026,'recommendation','non_211','北京工商大学',1),(2026,'recommendation','non_211','北京信息科技大学',1),(2026,'recommendation','non_211','中国民航大学',1),(2026,'recommendation','non_211','天津工业大学',1),(2026,'recommendation','non_211','国际关系学院',1),(2026,'recommendation','non_211','华北理工大学',1),(2026,'recommendation','non_211','安徽农业大学',1),(2026,'recommendation','non_211','杭州电子科技大学',1),(2026,'recommendation','non_211','曲阜师范大学',1),
    (2026,'exam','985','山东大学',9),(2026,'exam','985','西安交通大学',5),(2026,'exam','985','华中科技大学',4),(2026,'exam','985','南开大学',4),(2026,'exam','985','吉林大学',4),(2026,'exam','985','中国海洋大学',2),(2026,'exam','985','武汉大学',2),(2026,'exam','985','中央民族大学',1),(2026,'exam','985','东北大学',1),(2026,'exam','985','北京理工大学',1),(2026,'exam','985','北京师范大学',1),(2026,'exam','985','南京大学',1),(2026,'exam','985','北京航空航天大学',1),(2026,'exam','985','哈尔滨工业大学（威海）',1),(2026,'exam','985','哈尔滨工业大学',1),(2026,'exam','985','同济大学',1),(2026,'exam','985','国防科技大学',1),(2026,'exam','985','天津大学',1),(2026,'exam','985','复旦大学',1),(2026,'exam','985','湖南大学',1),(2026,'exam','985','西北工业大学',1),
    (2026,'exam','211','郑州大学',5),(2026,'exam','211','北京科技大学',4),(2026,'exam','211','安徽大学',4),(2026,'exam','211','西安电子科技大学',3),(2026,'exam','211','太原理工大学',2),(2026,'exam','211','中国矿业大学（北京）',2),(2026,'exam','211','北京交通大学',2),(2026,'exam','211','中国石油大学（北京）',1),(2026,'exam','211','云南大学',1),(2026,'exam','211','北京林业大学',1),(2026,'exam','211','北京工业大学',1),(2026,'exam','211','北京化工大学',1),(2026,'exam','211','四川农业大学',1),(2026,'exam','211','南京理工大学',1),(2026,'exam','211','北京邮电大学',1),(2026,'exam','211','暨南大学',1),(2026,'exam','211','河海大学',1),(2026,'exam','211','武汉理工大学',1),(2026,'exam','211','福州大学',1),(2026,'exam','211','湖南师范大学',1),
    (2026,'exam','non_211','河南大学',4),(2026,'exam','non_211','青岛大学',3),(2026,'exam','non_211','天津工业大学',2),(2026,'exam','non_211','南京信息工程大学',2),(2026,'exam','non_211','江西财经大学',2),(2026,'exam','non_211','浙江财经大学',2),(2026,'exam','non_211','郑州轻工业大学',2),(2026,'exam','non_211','集美大学',2),(2026,'exam','non_211','重庆邮电大学',2),(2026,'exam','non_211','浙江科技大学',2),(2026,'exam','non_211','河南工业大学',2),(2026,'exam','non_211','山东农业大学',2),(2026,'exam','non_211','南京邮电大学',2),(2026,'exam','non_211','河南理工大学',2),(2026,'exam','non_211','福建农林大学',2),(2026,'exam','non_211','北京语言大学',1),(2026,'exam','non_211','河南科技大学',1),(2026,'exam','non_211','重庆理工大学',1),(2026,'exam','non_211','西安邮电大学',1),(2026,'exam','non_211','齐鲁工业大学',1),(2026,'exam','non_211','华北理工大学',1),(2026,'exam','non_211','中南民族大学',1),(2026,'exam','non_211','上海电力大学',1),(2026,'exam','non_211','华东交通大学',1),(2026,'exam','non_211','大同大学',1),(2026,'exam','non_211','河北建筑工程学院',1),(2026,'exam','non_211','河北科技大学',1),(2026,'exam','non_211','江苏科技大学',1),(2026,'exam','non_211','江西农业大学',1),(2026,'exam','non_211','成都信息工程大学',1),(2026,'exam','non_211','安徽理工大学',1),(2026,'exam','non_211','山东科技大学',1),(2026,'exam','non_211','广州大学',1),(2026,'exam','non_211','西南民族大学',1),(2026,'exam','non_211','深圳技术大学',1),(2026,'exam','non_211','河南师范大学',1)
ON CONFLICT DO NOTHING;

INSERT INTO report_subject_stats (year, program, phase, subject, highest, lowest, average, median) VALUES
    (2026,'academic','initial_subject','政治',74,48,59,59),(2026,'academic','initial_subject','英语一',85,41,67,69),(2026,'academic','initial_subject','数学一',150,76,109,109),(2026,'academic','initial_subject','专业课',121,84,105,105),
    (2026,'professional','initial_subject','政治',70,48,58,58),(2026,'professional','initial_subject','英语二',92,58,76,76),(2026,'professional','initial_subject','数学二',150,82,120,120),(2026,'professional','initial_subject','专业课',119,66,96,95),
    (2026,'academic','admitted_total','初试成绩',388,315,343,339),(2026,'academic','admitted_total','复试成绩',98.44,79.03,87,87),(2026,'academic','admitted_total','总成绩',83.25,71.52,78,77),
    (2026,'professional','admitted_total','初试成绩',405,318,355,352),(2026,'professional','admitted_total','复试成绩',97.31,77.27,87,86),(2026,'professional','admitted_total','总成绩',87.96,72.04,79,78),
    (2025,'combined','initial_subject','政治',69,38,56,57),(2025,'combined','initial_subject','英语',78,34,54,55),(2025,'combined','initial_subject','数学',138,51,99,98),(2025,'combined','initial_subject','专业课',125,62,97,97),
    (2025,'combined','admitted_total','初试成绩',380,262,315,313),(2025,'combined','admitted_total','复试成绩',97.7,73.18,88,89),(2025,'combined','admitted_total','总成绩',85.25,64.19,75,75),
    (2024,'combined','initial_subject','政治',78,50,64,63),(2024,'combined','initial_subject','英语',88,42,68.3,73),(2024,'combined','initial_subject','数学',133,60,86.9,85),(2024,'combined','initial_subject','专业课',134,67,99.1,100),
    (2024,'combined','admitted_total','初试成绩',405,274,331,326),(2024,'combined','admitted_total','复试成绩',98.8,63.8,84.51,86),(2024,'combined','admitted_total','总成绩',86.6,61.8,74.82,75.09)
ON CONFLICT DO NOTHING;

INSERT INTO report_score_bands (year, program, band, band_order, interviewed, admitted, cumulative_interviewed, cumulative_admitted, note) VALUES
    (2026,'academic','380-389',1,5,5,5,5,NULL),(2026,'academic','370-379',2,3,3,8,8,NULL),(2026,'academic','360-369',3,5,4,13,12,NULL),(2026,'academic','350-359',4,11,6,24,18,NULL),(2026,'academic','340-349',5,14,8,38,26,NULL),(2026,'academic','330-339',6,17,11,55,37,NULL),(2026,'academic','320-329',7,18,10,73,47,NULL),(2026,'academic','310-319',8,9,7,82,54,NULL),
    (2026,'professional','400-409',1,3,3,3,3,NULL),(2026,'professional','390-399',2,7,7,10,10,NULL),(2026,'professional','380-389',3,9,6,19,16,NULL),(2026,'professional','370-379',4,9,8,28,24,NULL),(2026,'professional','360-369',5,10,9,38,33,NULL),(2026,'professional','350-359',6,23,18,61,51,NULL),(2026,'professional','340-349',7,21,10,82,61,NULL),(2026,'professional','330-339',8,24,14,106,75,NULL),(2026,'professional','320-329',9,20,12,126,87,NULL),(2026,'professional','310-319',10,5,2,131,89,NULL),
    (2026,'combined','400-409',1,3,3,3,3,NULL),(2026,'combined','390-399',2,7,7,10,10,NULL),(2026,'combined','380-389',3,14,11,24,21,NULL),(2026,'combined','370-379',4,12,11,36,32,NULL),(2026,'combined','360-369',5,15,13,51,45,NULL),(2026,'combined','350-359',6,34,24,85,69,NULL),(2026,'combined','340-349',7,35,18,120,87,NULL),(2026,'combined','330-339',8,41,25,161,112,NULL),(2026,'combined','320-329',9,38,22,199,134,NULL),(2026,'combined','310-319',10,14,9,213,143,NULL),
    (2025,'combined','371-380',1,2,2,2,2,NULL),(2025,'combined','361-370',2,2,2,4,4,NULL),(2025,'combined','351-360',3,12,11,16,15,NULL),(2025,'combined','341-350',4,10,10,26,25,'录取含 1 名特殊计划'),(2025,'combined','331-340',5,9,7,35,32,NULL),(2025,'combined','321-330',6,21,19,56,51,NULL),(2025,'combined','311-320',7,29,24,85,75,NULL),(2025,'combined','301-310',8,27,18,112,93,NULL),(2025,'combined','291-300',9,33,18,145,111,NULL),(2025,'combined','281-290',10,31,14,176,125,NULL),(2025,'combined','271-280',11,22,8,198,133,'录取含 2 名特殊计划'),(2025,'combined','261-270',12,13,4,211,137,'录取含 2 名特殊计划'),
    (2024,'combined','400-410',1,1,1,1,1,NULL),(2024,'combined','390-399',2,2,2,3,3,NULL),(2024,'combined','380-389',3,1,1,4,4,NULL),(2024,'combined','370-379',4,13,12,17,16,NULL),(2024,'combined','360-369',5,10,6,27,22,NULL),(2024,'combined','350-359',6,20,16,47,38,NULL),(2024,'combined','340-349',7,12,6,59,44,NULL),(2024,'combined','330-339',8,34,25,93,69,NULL),(2024,'combined','320-329',9,37,25,130,94,NULL),(2024,'combined','310-319',10,41,29,171,123,NULL),(2024,'combined','300-309',11,30,12,201,135,NULL),(2024,'combined','290-299',12,37,8,238,143,NULL),(2024,'combined','280-289',13,28,3,266,146,NULL),(2024,'combined','273-279',14,23,3,289,149,NULL)
ON CONFLICT DO NOTHING;

INSERT INTO report_lab_stats (year, program, lab, admitted, rejected, first_choice, highest, lowest, average, median, note) VALUES
    (2026,'professional',1,5,4,9,368,326,350,353,NULL),(2026,'professional',2,10,4,14,390,323,356,358,NULL),(2026,'professional',3,8,2,10,395,338,366,371,NULL),(2026,'professional',4,4,8,13,404,318,357,360,'调出 1 人'),(2026,'professional',5,5,3,8,405,353,383,383,NULL),(2026,'professional',6,17,5,22,398,320,357,356,NULL),(2026,'professional',7,11,5,16,395,320,349,341,NULL),(2026,'professional',8,12,0,11,359,328,347,350,'调入 1 人'),(2026,'professional',9,5,7,12,372,330,346,339,NULL),(2026,'professional',10,10,2,12,382,318,342,334,NULL),(2026,'professional',11,3,2,5,405,336,367,360,NULL),
    (2026,'academic',1,4,4,8,379,320,352,355,NULL),(2026,'academic',2,8,1,9,388,316,344,338,NULL),(2026,'academic',3,4,5,9,354,320,342,347,NULL),(2026,'academic',4,2,1,5,343,317,333,336,'调出 2 人'),(2026,'academic',5,3,5,8,335,315,326,329,NULL),(2026,'academic',6,8,3,11,386,340,365,363,NULL),(2026,'academic',7,5,2,7,350,317,331,325,NULL),(2026,'academic',8,7,0,5,383,324,343,337,'调入 2 人'),(2026,'academic',9,5,6,12,372,319,344,343,'调出 1 人'),(2026,'academic',10,6,1,6,338,315,327,327,'调入 1 人'),(2026,'academic',11,3,1,4,355,321,334,325,NULL),
    (2025,'combined',1,14,4,18,372,291,318,307,NULL),(2025,'combined',2,14,8,22,361,281,314,306,'未录取含自愿放弃 1 人'),(2025,'combined',3,10,7,17,359,306,329,325,NULL),(2025,'combined',4,8,3,11,350,295,320,321,NULL),(2025,'combined',5,13,8,21,359,289,322,317,NULL),(2025,'combined',6,19,11,30,363,267,312,311,NULL),(2025,'combined',7,14,3,17,356,261,301,297,NULL),(2025,'combined',8,11,5,16,380,262,310,313,NULL),(2025,'combined',9,14,6,20,356,273,307,313,NULL),(2025,'combined',10,14,11,25,349,272,307,305,NULL),(2025,'combined',11,6,5,11,355,281,320,324,NULL),
    (2024,'combined',1,16,6,22,399,283,326,325,NULL),(2024,'combined',2,13,11,24,375,279,332,330,'拟录取中含放弃 1 人'),(2024,'combined',3,14,15,29,356,299,324,322,NULL),(2024,'combined',4,8,24,32,390,290,343,349,NULL),(2024,'combined',5,16,12,28,376,301,339,335,NULL),(2024,'combined',6,22,20,42,374,284,321,322,'拟录取中含递补 1 人'),(2024,'combined',7,14,10,24,365,288,324,323,NULL),(2024,'combined',8,14,12,26,352,276,324,327,NULL),(2024,'combined',9,10,8,18,372,292,338,339,NULL),(2024,'combined',10,12,4,16,379,274,329,324,NULL),(2024,'combined',11,13,11,24,405,311,337,321,'拟录取中含少干计划 1 人')
ON CONFLICT DO NOTHING;
