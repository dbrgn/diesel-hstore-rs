extern crate diesel;

use std::collections::HashMap;
use std::error::Error;
use std::io;

use diesel::pg;
use diesel::types::{FromSql, ToSql, IsNull};

mod types;

impl diesel::types::HasSqlType<types::Hstore> for pg::Pg {
    fn metadata() -> pg::PgTypeMetadata {
        pg::PgTypeMetadata {
            oid: 1,
            array_oid: 1,
        }
    }
}

impl FromSql<types::Hstore, pg::Pg> for HashMap<String, String> {
    fn from_sql(bytes: Option<&[u8]>) -> Result<Self, Box<Error + Send + Sync>> {
        panic!(format!("FromSql Bytes: {:?}", bytes));
    }
}

impl ToSql<types::Hstore, pg::Pg> for HashMap<String, String> {
    fn to_sql<W: io::Write>(&self, out: &mut W) -> Result<IsNull, Box<Error + Send + Sync>> {
        let mut first = true;
        for (k, v) in self.iter() {
            if !first {
                out.write(b",")?;
            }
            out.write(k.as_bytes())?;
            out.write(b"=>")?;
            out.write(v.as_bytes())?;
            first = false;
        }
        Ok(IsNull::No)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn json_to_sql() {
        let mut bytes = vec![];

        let mut map: HashMap<String, String> = HashMap::new();
        map.insert("name".into(), "ferris".into());
        map.insert("species".into(), "crab".into());

        ToSql::<types::Hstore, pg::Pg>::to_sql(&map, &mut bytes).unwrap();
        assert_eq!(bytes, b"name=>ferris,species=>crab");
    }

}
