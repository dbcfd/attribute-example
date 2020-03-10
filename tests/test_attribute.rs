use attribute_example::{async_handler, sync_handler};

#[derive(Default)]
struct MyAsyncHandler {}

#[async_handler]
impl MyAsyncHandler {
    #[sync_handler]
    fn run(&self) -> Result<(), std::io::Error> {
        Ok(())
    }
}

#[tokio::test]
async fn test_attribute() {
    let v = MyAsyncHandler::default();

    v.run_async().await.unwrap();
}
