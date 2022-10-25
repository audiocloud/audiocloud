insert into media_jobs (id, media_id, state, download, created_at, updated_at)
values (?1, ?2, ?3, ?4, ?5, ?5)
on conflict (id) do update
    set media_id   = ?2,
        state      = ?3,
        download   = ?4,
        updated_at = ?5
where media_jobs.id = ?1