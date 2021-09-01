insert into card (front, back)
values ($1, $2)
returning (id, front, back, date);