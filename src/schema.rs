table! {
    use diesel::sql_types::*;
    use crate::types::*;

    things (id) {
        id -> Int4,
        my_things -> Nullable<Array<Thing>>,
        a_thing -> Thing,
    }
}
