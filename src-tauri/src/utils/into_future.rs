use async_trait::async_trait;

#[async_trait]
pub trait IntoFuture
where Self: Sized + 'static
{
    async fn into_future(self) -> Self {
        self
    }
}

impl<T, E> IntoFuture for Result<T, E>
where
    T: Sized + 'static,
    E: Sized + 'static
{}