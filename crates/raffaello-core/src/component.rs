use core::future::Future;
use futures::Stream;
use std::pin::Pin;

use crate::Draw;

pub type BoxFutureMsg<M> = Pin<Box<dyn Future<Output = Option<M>> + Send + 'static>>;
pub type BoxStreamMsg<M> = Pin<Box<dyn Stream<Item = M> + Send + 'static>>;

pub trait Model: Default {}

pub trait Component: 'static {
    type Message: Send + 'static;
    type Properties: 'static;

    fn new(_props: Self::Properties) -> Self
    where
        Self: Sized + Default,
    {
        Self::default()
    }

    /// Чистая функция: Model(self) -> Draw
    fn view(&self) -> Draw;

    /// Редьюсер: применяет сообщение и возвращает команды
    fn update(&mut self, msg: Self::Message) -> Command<Self::Message> {
        let _ = msg;
        Command::none()
    }

    /// Долгоживущие источники сообщений (таймеры, stdin и т.п.)
    fn subscriptions(&self) -> Subscriptions<Self::Message> {
        Subscriptions::none()
    }
}

#[async_trait::async_trait]
pub trait Actor<Action, Output>: Send + Sync + 'static {
    async fn call(&self, action: Action) -> Output;
}

pub enum Command<M> {
    None,
    Batch(Vec<BoxFutureMsg<M>>),
}
impl<M> Command<M> {
    pub fn none() -> Self {
        Self::None
    }

    pub fn single<F>(f: F) -> Self
    where
        F: Future<Output = Option<M>> + Send + 'static,
    {
        Command::Batch(vec![Box::pin(f)])
    }

    pub fn batch<I, F>(it: I) -> Self
    where
        I: IntoIterator<Item = F>,
        F: Future<Output = Option<M>> + Send + 'static,
    {
        Command::Batch(
            it.into_iter()
                .map(|f| Box::pin(f) as BoxFutureMsg<M>)
                .collect(),
        )
    }

    /// Поднять сообщения
    pub fn map<N>(self, f: fn(M) -> N) -> Command<N>
    where
        M: Send + 'static,
        N: Send + 'static,
    {
        match self {
            Command::None => Command::None,
            Command::Batch(list) => Command::Batch(
                list.into_iter()
                    .map(|fut| Box::pin(async move { fut.await.map(f) }) as BoxFutureMsg<N>)
                    .collect(),
            ),
        }
    }
}

pub enum Subscriptions<M> {
    None,
    Many(Vec<BoxStreamMsg<M>>),
}
impl<M> Subscriptions<M> {
    pub fn none() -> Self {
        Subscriptions::None
    }

    pub fn from_stream<S>(s: S) -> Self
    where
        S: Stream<Item = M> + Send + 'static,
    {
        Subscriptions::Many(vec![Box::pin(s)])
    }

    /// Объединить несколько подписок
    pub fn merge(self, other: Subscriptions<M>) -> Self {
        match (self, other) {
            (Subscriptions::None, x) => x,
            (x, Subscriptions::None) => x,
            (Subscriptions::Many(mut a), Subscriptions::Many(mut b)) => {
                a.append(&mut b);
                Subscriptions::Many(a)
            }
        }
    }

    /// Поднять тип сообщений
    pub fn map<N>(self, f: fn(M) -> N) -> Subscriptions<N>
    where
        M: Send + 'static,
        N: Send + 'static,
    {
        use futures::StreamExt;
        match self {
            Subscriptions::None => Subscriptions::None,
            Subscriptions::Many(v) => Subscriptions::Many(
                v.into_iter()
                    .map(|s| Box::pin(s.map(f)) as BoxStreamMsg<N>)
                    .collect(),
            ),
        }
    }
}
