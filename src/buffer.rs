use ::rusqlite;
use std::error;

type Error = Box<error::Error>;

pub struct Buffer {
    conn: rusqlite::Connection,
}

impl Buffer {
    pub fn new() -> Self {
        let conn = rusqlite::Connection::open_in_memory().unwrap();
        conn.execute("create table data(value text not null)", &[])
            .unwrap();
        Buffer { conn: conn }
    }

    pub fn dump(&self) -> Result<(), Error> {
        let mut stmt = self.conn
            .prepare("select rowid, value from data")?;
        let mut rows = stmt.query(&[])?;
        while let Some(result_row) = rows.next() {
            let row = result_row?;
            let n: i64 = row.get(0);
            let s: String = row.get(1);
            println!("{}|{:?}", n, s);
        }
        Ok(())
    }

    fn check(&self) -> Result<(), Error> {
        let mut stmt = self.conn.prepare("select rowid from data")?;
        let mut rows = stmt.query(&[])?;
        let mut i = 1;
        while let Some(result_row) = rows.next() {
            let row = result_row?;
            let n = row.get(0);
            if i != n {
                return Err(From::from(format!("hit row {}, expecting row {}", n, i)));
            }
            i += 1;
        }
        Ok(())
    }

    pub fn line_count(&self) -> Result<i64, Error> {
        let mut stmt = self.conn.prepare("select max(rowid) from data")?;
        let result = stmt.query_row(&[], |row| row.get(0))?;
        Ok(result)
    }

    pub fn lines(&self) -> Result<Vec<String>, Error> {
        let mut stmt = self.conn
            .prepare("select rowid, value from data")?;
        let mut rows = stmt.query(&[])?;
        let mut result = Vec::new();
        while let Some(result_row) = rows.next() {
            let row = result_row?;
            result.push(row.get(1));
        }
        self.check()?;
        Ok(result)
    }

    pub fn line(&self, addr: i64) -> Result<String, Error> {
        let mut stmt = self.conn
            .prepare("select value from data where rowid = ?")?;
        let result = stmt.query_row(&[&addr], |row| row.get(0))?;
        Ok(result)
    }



    // editing
    pub fn append(&mut self, s: &str) -> Result<(), Error> {
        let mut stmt = self.conn
            .prepare("insert into data(value) values (?)")?;
        for line in s.lines() {
            stmt.execute(&[&String::from(line)])?;
        }
        Ok(())
        // self.check()
    }

    pub fn extend<L, E>(&mut self, lines: L) -> Result<i64, Error>
        where L: Iterator<Item = Result<String, E>>,
              E: error::Error + 'static
    {
        let mut r = 0i64;
        let mut stmt = self.conn
            .prepare("insert into data(value) values (?)")?;
        for l in lines {
            let s: String = l?;
            r += stmt.execute(&[&s])? as i64;
        }
        Ok(r)
    }

    pub fn delete(&mut self, addr: i64) -> Result<(), Error> {
        {
            let tx = self.conn.transaction()?;
            {
                let mut stmt = tx.prepare("delete from data where rowid = ?")?;
                let c = stmt.execute(&[&addr])?;
                if c != 1 {
                    return Err(From::from(format!("expected 1 row affected, was {}", c)));
                }
                let mut stmt = tx.prepare("update data set rowid = rowid - 1 where rowid > ?")?;
                let c = stmt.execute(&[&addr])?;
                if c == 0 {
                }
            }
            tx.commit()?;
        }
        self.check()
    }
}
