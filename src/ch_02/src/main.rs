mod question;

use warp::Filter;

#[tokio::main]
async fn main() {

    question::test();

    let hello = warp::get()
        .map(|| format!("Hello, World!"));

    warp::serve(hello)
        .run(([127, 0, 0, 1], 8080))
        .await;
}
