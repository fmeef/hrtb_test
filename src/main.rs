use futures::{future::BoxFuture, Future};
struct BoxThing(Box<dyn FnOnce(&i64) -> BoxFuture<()>>);

// a trait for our function to implement
trait TestFn<'a>: Send + Sync {
    type Fut: Future<Output = ()>;
    fn call(self, v: &'a i64) -> Self::Fut;
}

// Implement for FnOnce returning future
impl<'a, F, Fut> TestFn<'a> for F
where
    F: FnOnce(&'a i64) -> Fut + Send + Sync,
    Fut: Future<Output = ()> + 'a,
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

async fn working<'a>(v: &'a i64) {}

async fn main_async() {
    // working: passing async fn
    test(working).await;

    // fails: passing closure with async move block
    test(|v: &_| async move {
        println!("works! {} ", v);
    })
    .await;
}

fn main() {
    futures::executor::block_on(main_async());
}
