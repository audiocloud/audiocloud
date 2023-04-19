use std::pin::Pin;

use async_stream::stream;
use futures::channel::oneshot;
use futures::{pin_mut, Stream};
use tokio::spawn;

pub type FlumeStream<T> = Pin<Box<dyn Stream<Item = T> + Send>>;

pub fn flume_stream<T>(receiver: flume::Receiver<T>) -> FlumeStream<T>
  where T: Send + 'static
{
  Box::pin(stream! {

    pin_mut!(receiver);

    loop {
      match receiver.recv_async().await {
        | Ok(item) => yield item,
        | Err(_) => break,
      }
    }
  })
}

pub fn from_oneshot<T>(oneshot_tx: oneshot::Sender<T>) -> flume::Sender<T>
  where T: Send + 'static
{
  let (tx, rx) = flume::bounded(1);

  spawn(async move {
    if let Ok(item) = rx.recv_async().await {
      let _ = oneshot_tx.send(item);
    }
  });

  tx
}
