update media_jobs
set in_progress = 0,
    state       = json_replace(state, '$.in_progress', json('false'))