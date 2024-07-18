use async_stream::stream;

use futures::pin_mut;
use futures::stream::Stream;
use futures::stream::StreamExt;

fn zero_to_three() -> impl Stream<Item = u32> {
    let s = stream! {
        for i in 0..3 {
            yield i;
        }
    };
    s
}

#[tokio::main]
async fn main() {
    let s = zero_to_three();
    pin_mut!(s); // needed for iteration

    while let Some(value) = s.next().await {
        println!("got {}", value);
    }
}
