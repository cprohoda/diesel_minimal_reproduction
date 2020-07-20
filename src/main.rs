#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

use diesel::*;
use diesel::serialize::{self, Output, ToSql, WriteTuple};
use diesel::deserialize::{self, FromSql};
use diesel::pg::Pg;
use diesel::sql_types::{Record, Integer};
use std::io::Write;

/* _____ up.sql copied here for convenience _____
CREATE TYPE Thing AS (
    first INT4,
    second INT4
);

CREATE TABLE things (
    id SERIAL PRIMARY KEY,
    my_things Thing ARRAY,
    a_thing Thing NOT NULL
);
*/

/* _____ down.sql copied here for convenience _____
DROP TABLE things;
DROP TYPE Thing;
*/

mod schema;

use crate::schema::things;

pub fn connect() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("Cannot find DATABASE_URL in .env file");
    PgConnection::establish(&database_url).expect("Error connecting to database")
}

#[derive(Queryable,Debug)]
pub struct Things {
    pub id: i32,
    pub my_things: Option<Vec<Thing>>,
    pub a_thing: Thing,
}

#[derive(Insertable)]
#[table_name="things"]
pub struct NewThings {
	pub my_things: Option<Vec<Thing>>,
	pub a_thing: Thing,
}

#[derive(SqlType)]
#[postgres(type_name = "Thing")]
pub struct ThingType;


#[derive(Queryable,Debug,SqlType,AsExpression,PartialEq)]
#[sql_type="ThingType"]
pub struct Thing {
	pub first: i32,
	pub second: i32,
}

impl ToSql<ThingType, Pg> for Thing {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
        WriteTuple::<(Integer, Integer)>::write_tuple(
            &(self.first, self.second),
            out,
        )
    }
}

impl FromSql<ThingType, Pg> for Thing {
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
        let (first,second): (i32, i32) = FromSql::<Record<(Integer, Integer)>, Pg>::from_sql(bytes)?;
        Ok(Thing {first: first, second: second})
    }
}

pub mod types {
    pub use super::ThingType as Thing;
}

pub fn create_things(connection: &PgConnection, things_to_create: &NewThings) -> Result<Things, diesel::result::Error> {
    use crate::schema::things::dsl::*;

    diesel::insert_into(things)
        .values(things_to_create)
        .get_result::<Things>(connection)
}

fn main() {
	unimplemented!();
}

mod tests {
    #[test]
    fn create_things_test() {
        use diesel::prelude::*;

        use super::connect;
        use super::{Things, NewThings, Thing};
        use super::create_things;

        let connection = connect();

        let test_things = NewThings {
        	my_things: Some(vec![Thing {first: 3i32, second: 4i32}]),
        	a_thing: Thing {first: 0i32, second: 1i32},
        };

        let expected_things = Things {
            id: 1,
            my_things: Some(vec![Thing {first: 3i32, second: 4i32}]),
            a_thing: Thing {first: 0i32, second: 1i32},
        };

        connection.test_transaction::<_, diesel::result::Error, _>(|| {
            let created_things = create_things(&connection, &test_things).expect("Test user creation failed");

            if expected_things.my_things != created_things.my_things
               || expected_things.a_thing != created_things.a_thing
            {
                Err(diesel::result::Error::__Nonexhaustive)
            } else {
                Ok(())
            }
        })
    }
}
