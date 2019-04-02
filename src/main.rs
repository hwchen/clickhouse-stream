use futures::Future;
use clickhouse_rs::{Pool};

fn main() -> Result<(), Box<std::error::Error>> {
    let pool = Pool::new("tcp://127.0.0.1:9000");

    let fut = pool
        .get_handle()
        .and_then(|c| {
            c.query("select * from test")
                .fold(0, |mut acc, row| {
                    let pop: Option<i64> = row.get("population")?;
                    if let Some(pop) = pop {
                        acc += pop
                    }
                    println!("{}", acc);
                    Ok(acc)
                })
        })
        .and_then(move |(_, i)| {
            println!("{}", i);
            Ok(())
        })
        .map_err(|err| eprintln!("database error: {}", err));

    tokio::run(fut);

    Ok(())
}
