INSERT INTO djpt.chara_view (id, cname, permissions) VALUES (1, '管理员', '1111111');
INSERT INTO djpt.chara_view (id, cname, permissions) VALUES (2, '审核员', '0010110');
INSERT INTO djpt.chara_view (id, cname, permissions) VALUES (3, '拟稿员', '0001110');
INSERT INTO djpt.chara_view (id, cname, permissions) VALUES (4, '普通用户', '0000110');

INSERT INTO djpt.column_view (id, parent_id, name, index, type) VALUES (-1, null, '默认栏目（未分类）', 99999, null);
INSERT INTO djpt.column_view (id, parent_id, name, index, type) VALUES (1, null, '党的政策', 0, null);
INSERT INTO djpt.column_view (id, parent_id, name, index, type) VALUES (2, null, '党的理论', 1, null);
INSERT INTO djpt.column_view (id, parent_id, name, index, type) VALUES (3, null, '党建活动', 2, null);

INSERT INTO djpt.sector_view (id, name) VALUES (1, '部门1');
INSERT INTO djpt.sector_view (id, name) VALUES (2, '部门2');

INSERT INTO djpt.department_view (id, seid, name) VALUES (-1, null, '未分类');
INSERT INTO djpt.department_view (id, seid, name) VALUES (1, 1, '科室1');
INSERT INTO djpt.department_view (id, seid, name) VALUES (2, 1, '科室2');
INSERT INTO djpt.department_view (id, seid, name) VALUES (3, 2, '科室3');

INSERT INTO djpt.branch_view (bid, name) VALUES (1, '支部1');

INSERT INTO djpt.group_view (gid, leader_uid, bid, name) VALUES (-1, null, null, '未分类');
INSERT INTO djpt.group_view (gid, leader_uid, bid, name) VALUES (1, null, 1, '小组1');

INSERT INTO djpt.user_view (id, cid, gid, deid, uname, psw, points, username, avatar) VALUES (7, 2, -1, -1, 'auditor', 'Zpx20222111', 0, null, 'http://localhost:8081/file/download/head.jpg');
INSERT INTO djpt.user_view (id, cid, gid, deid, uname, psw, points, username, avatar) VALUES (8, 3, -1, -1, 'editor', 'Zpx20222111', 0, null, 'http://localhost:8081/file/download/head.jpg');
INSERT INTO djpt.user_view (id, cid, gid, deid, uname, psw, points, username, avatar) VALUES (5, 4, 1, -1, 'zpx', 'Zpx20222111', 0, null, 'http://localhost:8081/file/download/head.jpg');
INSERT INTO djpt.user_view (id, cid, gid, deid, uname, psw, points, username, avatar) VALUES (3, 1, 1, 2, 'root', 'root', 0, '海绵狗狗', 'http://localhost:8081/file/download/head.jpg');
INSERT INTO djpt.user_view (id, cid, gid, deid, uname, psw, points, username, avatar) VALUES (9, 4, -1, -1, 'stu-1', 'password123', 0, null, 'http://localhost:8081/file/download/head.jpg');
INSERT INTO djpt.user_view (id, cid, gid, deid, uname, psw, points, username, avatar) VALUES (10, 4, -1, -1, 'stu-2', 'password124', 0, null, 'http://localhost:8081/file/download/head.jpg');
INSERT INTO djpt.user_view (id, cid, gid, deid, uname, psw, points, username, avatar) VALUES (11, 4, -1, -1, 'stu-3', 'password125', 0, null, 'http://localhost:8081/file/download/head.jpg');
INSERT INTO djpt.user_view (id, cid, gid, deid, uname, psw, points, username, avatar) VALUES (12, 4, -1, -1, 'stu-4', 'password126', 0, null, 'http://localhost:8081/file/download/head.jpg');
INSERT INTO djpt.user_view (id, cid, gid, deid, uname, psw, points, username, avatar) VALUES (13, 4, -1, -1, 'stu-5', 'password127', 0, null, 'http://localhost:8081/file/download/head.jpg');
INSERT INTO djpt.user_view (id, cid, gid, deid, uname, psw, points, username, avatar) VALUES (14, 4, -1, -1, 'stu-6', 'password128', 0, null, 'http://localhost:8081/file/download/head.jpg');
INSERT INTO djpt.user_view (id, cid, gid, deid, uname, psw, points, username, avatar) VALUES (15, 4, -1, -1, 'stu-7', 'password129', 0, null, 'http://localhost:8081/file/download/head.jpg');
INSERT INTO djpt.user_view (id, cid, gid, deid, uname, psw, points, username, avatar) VALUES (16, 4, -1, -1, 'stu-8', 'password130', 0, null, 'http://localhost:8081/file/download/head.jpg');
INSERT INTO djpt.user_view (id, cid, gid, deid, uname, psw, points, username, avatar) VALUES (17, 4, -1, -1, 'stu-9', 'password131', 0, null, 'http://localhost:8081/file/download/head.jpg');
INSERT INTO djpt.user_view (id, cid, gid, deid, uname, psw, points, username, avatar) VALUES (18, 4, -1, -1, 'stu-10', 'password132', 0, null, 'http://localhost:8081/file/download/head.jpg');
INSERT INTO djpt.user_view (id, cid, gid, deid, uname, psw, points, username, avatar) VALUES (19, 4, -1, -1, 'stu-11', 'password133', 0, null, 'http://localhost:8081/file/download/head.jpg');
INSERT INTO djpt.user_view (id, cid, gid, deid, uname, psw, points, username, avatar) VALUES (20, 4, -1, -1, 'stu-12', 'password134', 0, null, 'http://localhost:8081/file/download/head.jpg');
INSERT INTO djpt.user_view (id, cid, gid, deid, uname, psw, points, username, avatar) VALUES (21, 4, -1, -1, 'stu-13', 'password135', 0, null, 'http://localhost:8081/file/download/head.jpg');
INSERT INTO djpt.user_view (id, cid, gid, deid, uname, psw, points, username, avatar) VALUES (22, 4, -1, -1, 'stu-14', 'password136', 0, null, 'http://localhost:8081/file/download/head.jpg');
INSERT INTO djpt.user_view (id, cid, gid, deid, uname, psw, points, username, avatar) VALUES (23, 4, -1, -1, 'stu-15', 'password137', 0, null, 'http://localhost:8081/file/download/head.jpg');
INSERT INTO djpt.user_view (id, cid, gid, deid, uname, psw, points, username, avatar) VALUES (6, 1, -1, 3, 'u1', 'Zpx20222111', 0, null, 'http://localhost:8081/file/download/head.jpg');
INSERT INTO djpt.user_view (id, cid, gid, deid, uname, psw, points, username, avatar) VALUES (4, 1, 1, -1, 'u2', '123456', 100, null, 'http://localhost:8081/file/download/head.jpg');

INSERT INTO djpt.branch_manager_view (id, bid, uid, name) VALUES (1, 1, 3, '支部书记');

INSERT INTO djpt.draft_view (id, uid, aid, coid, content, save_time, title, source, status, send_time) VALUES (1, 3, null, 1, '<p>1</p>', '2025-02-04 22:08:00', '测试1', null, 4, '2025-02-04 22:08:00');
INSERT INTO djpt.draft_view (id, uid, aid, coid, content, save_time, title, source, status, send_time) VALUES (4, 3, null, 1, '<p>111</p>', '2025-02-19 19:15:14', '测试1', '1', 4, '2025-02-19 19:15:14');
INSERT INTO djpt.draft_view (id, uid, aid, coid, content, save_time, title, source, status, send_time) VALUES (3, 3, null, 1, '<p>测试</p>', '2025-02-18 20:34:31', '测试', null, 4, '2025-02-18 20:34:31');
INSERT INTO djpt.draft_view (id, uid, aid, coid, content, save_time, title, source, status, send_time) VALUES (5, 3, null, -1, '<p>新华社北京3月6日电（记者戴小河、农冠斌）中国海油3月6日宣布，旗下位于我国北部湾海域的涠洲10-5油气田获得高产油气流，标志着北部湾盆地古生界潜山油气勘探获重大突破。</p><p>涠洲10-5油气田位于南海北部湾海域，距离广西北海市约75公里，平均水深约37米。探井钻遇油气层283米，完钻井深约4840米。经测试，此井日产天然气约37万立方米，日产原油约102吨，成为北部湾盆地首个花岗岩潜山油气勘探发现。</p><p>北部湾盆地涠西南凹陷是中国海域勘探程度最高的凹陷之一。经过40余年的勘探开发，发现整装油气田的难度逐渐增加。近年来，中国海油加强复杂潜山成储理论研究与关键技术攻关，推动潜山成为中国近海油气勘探储量新增长点。</p><p>中国海油湛江分公司南海西部石油研究院院长范彩伟说，涠西南凹陷是北部湾盆地油气最富集的凹陷，开展花岗岩潜山成山、成储、成藏研究攻关，优选成藏条件最有利的近洼断裂带实施钻探，有望实现勘探突破。</p><p>中国海油首席执行官周心怀表示，近年来中国海油在我国多个海域获得多种类型潜山的大中型油气发现，进一步夯实了公司的油气资源基础。此次在北部湾盆地探获勘探发现，将有助于保障油气资源的稳定供应。</p>', '2025-03-09 10:17:12', '我国北部湾海域油气勘探获重大突破', 'http://www.dangjian.cn/djyw/2025/03/07/detail_202503077812711.html', 4, '2025-03-09 10:17:12');
INSERT INTO djpt.draft_view (id, uid, aid, coid, content, save_time, title, source, status, send_time) VALUES (6, 3, null, -1, '<p>测试2</p>', '2025-03-09 10:40:55', '测试2', null, 1, '2025-03-09 10:40:55');
INSERT INTO djpt.draft_view (id, uid, aid, coid, content, save_time, title, source, status, send_time) VALUES (7, 3, null, -1, '<p>测</p>', '2025-03-09 10:52:18', '测试3', null, 1, '2025-03-09 10:52:18');
INSERT INTO djpt.draft_view (id, uid, aid, coid, content, save_time, title, source, status, send_time) VALUES (8, 3, null, 1, '<p>测试4</p>', '2025-03-09 14:08:24', '测试4', null, 1, '2025-03-09 14:08:24');
INSERT INTO djpt.draft_view (id, uid, aid, coid, content, save_time, title, source, status, send_time) VALUES (9, 3, null, 1, '<p>测试6</p>', '2025-03-09 14:37:12', '测试6', null, 1, '2025-03-09 14:37:12');

INSERT INTO djpt.submit_view (id, did, time, status, title, content, source) VALUES (1, 3, null, 0, '测试', '<p>测试</p>', null);
INSERT INTO djpt.submit_view (id, did, time, status, title, content, source) VALUES (2, 4, null, 0, '测试1', '<p>111</p>', '1');
INSERT INTO djpt.submit_view (id, did, time, status, title, content, source) VALUES (3, 5, null, 0, '我国北部湾海域油气勘探获重大突破', '<p>新华社北京3月6日电（记者戴小河、农冠斌）中国海油3月6日宣布，旗下位于我国北部湾海域的涠洲10-5油气田获得高产油气流，标志着北部湾盆地古生界潜山油气勘探获重大突破。</p><p>涠洲10-5油气田位于南海北部湾海域，距离广西北海市约75公里，平均水深约37米。探井钻遇油气层283米，完钻井深约4840米。经测试，此井日产天然气约37万立方米，日产原油约102吨，成为北部湾盆地首个花岗岩潜山油气勘探发现。</p><p>北部湾盆地涠西南凹陷是中国海域勘探程度最高的凹陷之一。经过40余年的勘探开发，发现整装油气田的难度逐渐增加。近年来，中国海油加强复杂潜山成储理论研究与关键技术攻关，推动潜山成为中国近海油气勘探储量新增长点。</p><p>中国海油湛江分公司南海西部石油研究院院长范彩伟说，涠西南凹陷是北部湾盆地油气最富集的凹陷，开展花岗岩潜山成山、成储、成藏研究攻关，优选成藏条件最有利的近洼断裂带实施钻探，有望实现勘探突破。</p><p>中国海油首席执行官周心怀表示，近年来中国海油在我国多个海域获得多种类型潜山的大中型油气发现，进一步夯实了公司的油气资源基础。此次在北部湾盆地探获勘探发现，将有助于保障油气资源的稳定供应。</p>', 'http://www.dangjian.cn/djyw/2025/03/07/detail_202503077812711.html');
INSERT INTO djpt.submit_view (id, did, time, status, title, content, source) VALUES (4, 6, null, 0, '测试2', '<p>测试2</p>', null);
INSERT INTO djpt.submit_view (id, did, time, status, title, content, source) VALUES (5, 7, null, 0, '测试3', '<p>测</p>', null);
INSERT INTO djpt.submit_view (id, did, time, status, title, content, source) VALUES (6, 8, null, 3, '测试4', '<p>测试4</p>', null);
INSERT INTO djpt.submit_view (id, did, time, status, title, content, source) VALUES (7, 9, null, 2, '测试6', '<p>测试6</p>', null);

INSERT INTO djpt.audit_view (id, uid, srid, advice, time, status, next) VALUES (1, 4, 1, null, null, 1, null);
INSERT INTO djpt.audit_view (id, uid, srid, advice, time, status, next) VALUES (2, 3, 1, null, '2025-02-19 19:09:08', 2, 1);
INSERT INTO djpt.audit_view (id, uid, srid, advice, time, status, next) VALUES (3, 3, 2, null, '2025-02-19 19:22:17', 2, null);
INSERT INTO djpt.audit_view (id, uid, srid, advice, time, status, next) VALUES (4, 3, 2, null, '2025-02-19 19:22:03', 2, 3);
INSERT INTO djpt.audit_view (id, uid, srid, advice, time, status, next) VALUES (5, 3, 3, null, '2025-03-09 10:18:56', 3, null);
INSERT INTO djpt.audit_view (id, uid, srid, advice, time, status, next) VALUES (6, 3, 3, null, '2025-03-09 10:18:33', 2, 5);
INSERT INTO djpt.audit_view (id, uid, srid, advice, time, status, next) VALUES (7, 3, 4, null, null, 0, null);
INSERT INTO djpt.audit_view (id, uid, srid, advice, time, status, next) VALUES (8, 3, 4, null, '2025-03-09 14:23:14', 2, 7);
INSERT INTO djpt.audit_view (id, uid, srid, advice, time, status, next) VALUES (9, 3, 5, null, '2025-03-09 11:03:17', 2, null);
INSERT INTO djpt.audit_view (id, uid, srid, advice, time, status, next) VALUES (10, 3, 5, null, '2025-03-09 10:55:28', 2, 9);
INSERT INTO djpt.audit_view (id, uid, srid, advice, time, status, next) VALUES (11, 3, 6, null, '2025-03-09 14:35:53', 3, null);
INSERT INTO djpt.audit_view (id, uid, srid, advice, time, status, next) VALUES (12, 3, 6, null, '2025-03-09 14:34:20', 2, 11);
INSERT INTO djpt.audit_view (id, uid, srid, advice, time, status, next) VALUES (13, 3, 7, null, '2025-03-09 14:37:39', 2, null);
INSERT INTO djpt.audit_view (id, uid, srid, advice, time, status, next) VALUES (14, 3, 7, null, '2025-03-09 14:37:29', 2, 13);

INSERT INTO djpt.process_view (id, next, last) VALUES (26, null, 1);
INSERT INTO djpt.process_view (id, next, last) VALUES (1, 26, 0);
INSERT INTO djpt.process_view (id, next,  last) VALUES (34, null, 1);
INSERT INTO djpt.process_view (id, next, last) VALUES (32, 34, 0);

INSERT INTO djpt.processtype_view (id, pid, name, number, type) VALUES (5, 32, '测试1', 1, 0);
INSERT INTO djpt.processtype_view (id, pid, name, number, type) VALUES (1, 1, '默认', 2, 1);

INSERT INTO djpt.article_view (id, coid, content, publish_time, title, source, show) VALUES (1, 1, '<p>测试</p>', '2025-02-19 19:09:08', '测试', null, 0);
INSERT INTO djpt.article_view (id, coid, content, publish_time, title, source, show) VALUES (2, 1, '<p>111</p>', '2025-02-19 19:22:17', '测试1', '1', 0);
INSERT INTO djpt.article_view (id, coid, content, publish_time, title, source, show) VALUES (3, 3, '<p>新华社北京3月6日电（记者戴小河、农冠斌）中国海油3月6日宣布，旗下位于我国北部湾海域的涠洲10-5油气田获得高产油气流，标志着北部湾盆地古生界潜山油气勘探获重大突破。</p><p>涠洲10-5油气田位于南海北部湾海域，距离广西北海市约75公里，平均水深约37米。探井钻遇油气层283米，完钻井深约4840米。经测试，此井日产天然气约37万立方米，日产原油约102吨，成为北部湾盆地首个花岗岩潜山油气勘探发现。</p><p>北部湾盆地涠西南凹陷是中国海域勘探程度最高的凹陷之一。经过40余年的勘探开发，发现整装油气田的难度逐渐增加。近年来，中国海油加强复杂潜山成储理论研究与关键技术攻关，推动潜山成为中国近海油气勘探储量新增长点。</p><p>中国海油湛江分公司南海西部石油研究院院长范彩伟说，涠西南凹陷是北部湾盆地油气最富集的凹陷，开展花岗岩潜山成山、成储、成藏研究攻关，优选成藏条件最有利的近洼断裂带实施钻探，有望实现勘探突破。</p><p>中国海油首席执行官周心怀表示，近年来中国海油在我国多个海域获得多种类型潜山的大中型油气发现，进一步夯实了公司的油气资源基础。此次在北部湾盆地探获勘探发现，将有助于保障油气资源的稳定供应。</p>', '2025-03-09 10:18:56', '我国北部湾海域油气勘探获重大突破', 'http://www.dangjian.cn/djyw/2025/03/07/detail_202503077812711.html', 0);
INSERT INTO djpt.article_view (id, coid, content, publish_time, title, source, show) VALUES (4, -1, '<p>测</p>', '2025-03-09 11:03:17', '测试3', null, 0);
INSERT INTO djpt.article_view (id, coid, content, publish_time, title, source, show) VALUES (5, 1, '<p>测试6</p>', '2025-03-09 14:37:39', '测试6', null, 0);

INSERT INTO djpt.comments_view (id, uid, audit_uid, aid, parent_id, content, status, send_time) VALUES (1, null, null, 5, null, null, 0, '2025-03-22 01:51:36');

INSERT INTO djpt.activity_view (id, uid, name, time, content, type, cover_image) VALUES (-1, null, '（未分类资源）', null, null, null, 'http://localhost:8081/file/download/dsjtjt.jpg');
INSERT INTO djpt.activity_view (id, uid, name, time, content, type, cover_image) VALUES (5, 3, '活动1', '2025-02-04 14:07:33', '测试', 1, 'http://localhost:8081/file/download/dsjtjt.jpg');
INSERT INTO djpt.activity_view (id, uid, name, time, content, type, cover_image) VALUES (6, 3, '测试', '2025-02-09 11:54:19', e'议程1：……
议程2：……', 1, 'http://localhost:8081/file/download/8-1.png');

INSERT INTO djpt.participate_view (acid, uid, id, type, detail) VALUES (6, 5, 2, 1, '生病');
INSERT INTO djpt.participate_view (acid, uid, id, type, detail) VALUES (6, 4, 3, 0, null);
INSERT INTO djpt.participate_view (acid, uid, id, type, detail) VALUES (6, 3, 1, 0, null);

INSERT INTO djpt.resource_view (id, uid, acid, name, content, savetime, type, description) VALUES (1, 3, 5, 'sql.docx', 'http://localhost:8081/file/download/sql.docx', '2025-02-07 17:39:05', 4, null);
INSERT INTO djpt.resource_view (id, uid, acid, name, content, savetime, type, description) VALUES (2, 3, 5, 'login_bg.png', 'http://localhost:8081/file/download/login_bg.png', '2025-02-07 17:39:38', 1, null);
INSERT INTO djpt.resource_view (id, uid, acid, name, content, savetime, type, description) VALUES (3, 3, 5, 'dj001 (57).png', 'http://localhost:8081/file/download/dj001 (57).png', '2025-02-07 17:39:38', 1, null);
INSERT INTO djpt.resource_view (id, uid, acid, name, content, savetime, type, description) VALUES (4, 3, 5, '背景.jpg', 'http://localhost:8081/file/download/背景.jpg', '2025-02-07 17:39:38', 1, null);
INSERT INTO djpt.resource_view (id, uid, acid, name, content, savetime, type, description) VALUES (5, 3, 5, '校标（高清）无白底.png', 'http://localhost:8081/file/download/校标（高清）无白底.png', '2025-02-10 00:12:13', 1, '校标');

-- 生成重置脚本（需替换表名和序列名）
DO $$
    DECLARE
        reset_sql TEXT;
    BEGIN
        -- 生成所有序列的 setval() 命令
        FOR reset_sql IN
            SELECT
                'SELECT setval(''' || n.nspname || '.' || c.relname || ''', COALESCE(MAX(' || a.attname || '), 0) + 1) FROM ' || t.relname || ';' AS cmd
            FROM
                pg_class c
                    JOIN pg_namespace n ON n.oid = c.relnamespace
                    JOIN pg_depend d ON d.objid = c.oid
                    JOIN pg_class t ON t.oid = d.refobjid
                    JOIN pg_attribute a ON a.attrelid = t.oid AND a.attnum = d.refobjsubid
            WHERE
                c.relkind = 'S'           -- 只处理序列
              AND n.nspname NOT LIKE 'pg_%'  -- 排除系统序列
              AND d.refobjsubid > 0     -- 确保关联到表字段
            LOOP
                EXECUTE reset_sql;  -- 自动执行生成的SQL
                RAISE NOTICE '已执行: %', reset_sql;  -- 打印日志
            END LOOP;
    END $$;
