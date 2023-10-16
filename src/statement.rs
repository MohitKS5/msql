use crate::InputBuffer;

pub struct Statement {
    kind: Kind,
    sql: String,
}

enum Kind {
    SELECT,
    INSERT,
}

pub fn prepare(buf: InputBuffer) -> Result<Statement, &'static str> {
    if buf.buffer.len() < 6 {
        return Err("Unrecognized statement");
    }
    let kind = if buf.buffer[0..6].to_ascii_lowercase() == "insert" {
        Kind::INSERT
    } else if buf.buffer[0..6].to_ascii_lowercase() == "select" {
        Kind::SELECT
    } else {
        return Err("Unrecognized statement");
    };
    Ok(Statement {
        kind,
        sql: buf.buffer,
    })
}

pub fn execute(statement: Statement) -> Result<(), &'static str> {
    match statement.kind {
        Kind::INSERT => println!("insert"),
        Kind::SELECT => println!("select")
    };
    Ok(())
}
