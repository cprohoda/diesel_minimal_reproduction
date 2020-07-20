CREATE TYPE Thing AS (
    first INT4,
    second INT4
);

CREATE TABLE things (
    id SERIAL PRIMARY KEY,
    my_things Thing ARRAY,
    a_thing Thing NOT NULL
);
