--: Novel()

--! get_novels : Novel
SELECT 
    id, 
    novel_id,
    author_id,
    title
FROM novels;

--! insert_novel(novel_id, author_id, title, created_at?, updated_at?) : Novel
INSERT INTO novels (
    novel_id,
    author_id,
    title,
    created_at,
    updated_at
) VALUES (
    :novel_id,
    :author_id,
    :title,
    created_at,
    updated_at
) ON CONFLICT (novel_id) 
DO
    UPDATE SET author_id = :author_id, title = :title 
RETURNING *;
