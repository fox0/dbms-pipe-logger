const USERNAME: &str = "sys";

fn main() -> Result<(), oracle::Error> {
    let conn = oracle::Connection::connect(USERNAME, "tiger", "//localhost/XE")?;

    let sql = "select ename, sal, comm from emp where deptno = :1";

// Select a table with a bind variable.
    println!("---------------|---------------|---------------|");
    let rows = conn.query(sql, &[&30])?;
    for row_result in rows {
        let row = row_result?;
        // get a column value by position (0-based)
        let ename: String = row.get(0)?;
        // get a column by name (case-insensitive)
        let sal: i32 = row.get("sal")?;
        // Use `Option<...>` to get a nullable column.
        // Otherwise, `Err(Error::NullValue)` is returned
        // for null values.
        let comm: Option<i32> = row.get(2)?;

        println!(" {:14}| {:>10}    | {:>10}    |",
                 ename,
                 sal,
                 comm.map_or("".to_string(), |v| v.to_string()));
    }

// Another way to fetch rows.
// The rows iterator returns Result<(String, i32, Option<i32>)>.
    println!("---------------|---------------|---------------|");
    let rows = conn.query_as::<(String, i32, Option<i32>)>(sql, &[&10])?;
    for row_result in rows {
        let (ename, sal, comm) = row_result?;
        println!(" {:14}| {:>10}    | {:>10}    |",
                 ename,
                 sal,
                 comm.map_or("".to_string(), |v| v.to_string()));
    }
    Ok(())
}
