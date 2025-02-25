use std::time::Duration;

use tracing::Level;
use tracing_mock::{expect, subscriber};

#[tokio::test]
#[should_panic]
async fn repro_should_fail() {
    let (subscriber, handle) = subscriber::mock()
        .event(
            expect::event()
                .at_level(Level::ERROR)
                .with_fields(expect::field("message").with_value(&"not real message")),
        )
        .run_with_handle();

    const SLEEP_TIME: Duration = Duration::from_millis(40);

    {
        let _guard = tracing::subscriber::set_default(subscriber);

        tokio::spawn(async {
            tokio::time::sleep(SLEEP_TIME).await;
            tracing::error!("real message");
        });

        tokio::time::sleep(SLEEP_TIME + Duration::from_millis(10)).await;
    }

    handle.assert_finished();
}

#[tokio::test]
#[should_panic = "[fails_as_expected]"]
async fn fails_as_expected() {
    let (subscriber, handle) = subscriber::mock()
        .event(
            expect::event()
                .at_level(Level::ERROR)
                .with_fields(expect::field("message").with_value(&"not real message")),
        )
        .run_with_handle();

    {
        let _guard = tracing::subscriber::set_default(subscriber);
        tracing::error!("real message");
    }

    handle.assert_finished();
}
