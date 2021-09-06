insert into card (id, front, back)
  values (
    coalesce(nullif($1, 0), nextval('card_id_seq')),
    $2,
    $3
  )
on conflict (id) do update
  set
    front = $2,
    back = $3
returning id, front, back, date;