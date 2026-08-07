pub trait TryFromAsync<T>: Sized {
    type Error;

    fn try_from_async(input: T) -> impl Future<Output = Result<Self, Self::Error>> + Send;
}
