use futures::{self, Future, Stream};
use clickhouse_rs::Pool;

fn main() -> Result<(), Box<std::error::Error>> {
    let query = std::env::args().nth(1)
        .ok_or("please enter a query")?;

    let pool = Pool::new("tcp://127.0.0.1:9000");

    let fut = pool
        .get_handle()
        .and_then(move |c| {
            c.query(query)
                .stream_blocks()
                .for_each(|block| {
                    if let Ok(b) = block {
                        println!("{:?}\nblock row count: {}", b, b.row_count());
                    }
                    Ok(())
                })
        })
        .map_err(|err| eprintln!("database error: {}", err));

    tokio::run(fut);

    Ok(())
}

