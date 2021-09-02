create role fc with login password 'EAqfhBBlK3w=';

grant connect on database fc to fc;

grant select on card to fc;
grant insert on card to fc;

grant usage on card_id_seq to fc;