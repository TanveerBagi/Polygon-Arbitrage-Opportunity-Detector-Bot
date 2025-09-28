use rusqlite::{params, Connection, Result};
use chrono::Local;

pub fn init_db() -> Result<Connection> {
    let conn = Connection::open("arb_bot.db")?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS opportunities (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            timestamp TEXT NOT NULL,
            token_pair TEXT NOT NULL,
            buy_dex TEXT NOT NULL,
            sell_dex TEXT NOT NULL,
            profit REAL NOT NULL
        )",
        [],
    )?;

    Ok(conn)
}

pub fn save_opportunity(
    conn: &Connection,
    token_pair: &str,
    buy_dex: &str,
    sell_dex: &str,
    profit: f64,
) -> Result<()> {
    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    conn.execute(
        "INSERT INTO opportunities (timestamp, token_pair, buy_dex, sell_dex, profit)
         VALUES (?1, ?2, ?3, ?4, ?5)",
        params![timestamp, token_pair, buy_dex, sell_dex, profit],
    )?;

    Ok(())
}

pub fn show_opportunities(conn: &Connection) -> Result<()> {
    let mut stmt = conn.prepare(
        "SELECT id, timestamp, token_pair, buy_dex, sell_dex, profit FROM opportunities",
    )?;

    let rows = stmt.query_map([], |row| {
        Ok((
            row.get::<_, i32>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, String>(2)?,
            row.get::<_, String>(3)?,
            row.get::<_, String>(4)?,
            row.get::<_, f64>(5)?,
        ))
    })?;

    for row in rows {
        println!("Row: {:?}", row?);
    }

    Ok(())
}
