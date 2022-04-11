use futures::Future;
trait TestFn<'a>: Send + Sync {
    type Fut: Future<Output = ()>;
    fn call(self, v: &'a i64) -> Self::Fut;
}

impl<'a, F, Fut> TestFn<'a> for F
where
    F: FnOnce(&'a i64) -> Fut + Send + Sync,
    Fut: Future<Output = ()>,
{
    type Fut = Fut;

    fn call(self, v: &'a i64) -> Self::Fut {
        self(v)
    }
}

async fn test<F>(func: F)
where
    F: for<'a> TestFn<'a>,
{
    func.call(&3).await
}

async fn main_async() {
    test(|v| async move {
        println!("works! {} ", v);
    })
    .await;
}

fn main() {
    futures::executor::block_on(main_async());
}
