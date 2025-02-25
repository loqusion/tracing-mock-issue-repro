use tracing::{Level, Subscriber};
use tracing_mock::{expect, subscriber};

#[tokio::test]
#[should_panic]
async fn repro_should_fail() {
    let (subscriber, handle) = subscriber_mock();

    let join_handle = {
        let _guard = tracing::subscriber::set_default(subscriber);
        let join_handle = tokio::spawn(async {
            tracing::error!("real message");
        });
        tokio::task::yield_now().await;
        join_handle
    };

    // If `tokio-mock` assertions fail, this will panic
    join_handle.await.unwrap();

    handle.assert_finished();
}

#[tokio::test]
#[should_panic = "[fails_successfully]"]
async fn fails_successfully() {
    let (subscriber, handle) = subscriber_mock();

    {
        let _guard = tracing::subscriber::set_default(subscriber);
        tracing::error!("real message");
    }

    handle.assert_finished();
}

fn subscriber_mock() -> (impl Subscriber, subscriber::MockHandle) {
    subscriber::mock()
        .event(expect::event().at_level(Level::WARN))
        .run_with_handle()
}
