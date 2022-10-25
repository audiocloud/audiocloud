select id, media_id, upload, state, created_at
from media_jobs
where in_progress = 0
  and upload is not null
order by created_at
limit ?1