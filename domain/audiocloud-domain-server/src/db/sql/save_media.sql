update media_objects
set metadata  = ?2,
    path      = ?3,
    last_used = ?4,
    revision  = revision + 1
where id = ?1