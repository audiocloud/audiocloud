select id, metadata, path, created_at, last_used, revision
from media_objects
where id = ?1