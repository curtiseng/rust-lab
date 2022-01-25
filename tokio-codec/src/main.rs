use std::fmt::{Display, Formatter};
use std::pin::Pin;
use std::task::{Context, Poll};
use futures::{FutureExt, Stream};
use pin_project_lite::pin_project;
use tokio_stream::{self as stream, StreamExt};
use tokio::io::{AsyncRead, AsyncWrite};
use tokio_util::codec::{Decoder, Framed};

mod frame;

type BoxError = Box<dyn std::error::Error + Send + Sync>;

#[tokio::main]
async fn main() {
    let mut stream = stream::iter(vec![0, 1, 2]);

    while let Some(value) = stream.map(|x| x * 2).next().await {
        println!("Got {}", value);
    }
}

pub struct Server<I, S, P> {
    channels: Vec<Channel<I, S, P>>
}

pub struct Channel<I, S, P> {
    conn: I,
    proto: P,
    // 持有的用户最终handler，async_fn_handler()
    inner: S,
}

#[derive(Debug)]
pub struct Message {}

impl Display for Message {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl <I, S, P> Channel<I, S, P>
where
    S: Handle<S>
{
    // 改造为builder模式
    fn handle(conn:I, proto: P, handle : S) -> Channel<I, S, P> {
        Channel {
            conn,
            proto,
            inner: handle,
        }
    }
}
//       根据proto和transport             包装桥接framed
//                         framed -> framed_inbound_handler  -> other_inbound_handler  ->
// incoming <=> channel <=>                                            | error |      async_fn_handler() -> ? 无返回值怎么处理
//                         framed <- framed_outbound_handler <- other_outbound_handler <-
pub trait Handle<Message> {

    type Item;

    type Error;

    type Stream : Stream<Item = Option<Result<Self::Item, Self::Error>>>;

    // 控制处理的速度,是否需要?
    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>>;

    // 获取上一个stream，用来创建下一个stream
    // 每次连接创建一个channel，每个channel调用一次handle链来创建stream
    fn process(&mut self, message: Message) -> Self::Stream;
}

#[derive(Debug, Clone)]
pub struct FramedHandler<S> {
    inner: S,
}

pub fn make_handler<T, U, S>(inner: T, codec: U) -> FramedHandler<S>
where
    T: AsyncRead + AsyncWrite,
    U: Decoder,
{
    FramedHandler {
        inner: Framed::new(inner, codec),
    }
}

// call的是request，handler的是什么, Message还是Channel
impl<S, U, E> Handle<Message> for FramedHandler<S>
where
    U: Decoder,
    E: Into<BoxError>,
    S: Stream<Item = Result<U::Item, E>>,
{
    type Item = U::Item;
    type Error = E;
    type Stream = S;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn process(&mut self, message: Message) -> Self::Stream {
        println!("message: {}", message);
        // 包装handler无法访问process，获取framed
        FramedStream {
            inner : self.inner
        }
    }
}

pin_project! {
    pub struct FramedStream<S> {
        #[pin]
        inner: S,
    }
}


impl<S, U, E> Stream for FramedStream<S>
where
    U: Decoder,
    E: Into<BoxError>,
    S: Stream<Item = Result<U::Item, E>>,
{
    type Item = Result<U::Item, E>;

    fn poll_next(self: &mut Self, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        // 包装stream默认同被包装状态相同
        match self.project().inner.poll_next(cx) {
            Poll::Ready(t) => Poll::Ready(t),
            Poll::Pending => Poll::Pending,
        }
        // timeout是在pending时计时
    }
}

