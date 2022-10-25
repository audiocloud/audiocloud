select id, media_id, download, state, created_at
from media_jobs
where in_progress = 0
  and download is not null
order by created_at
limit ?1