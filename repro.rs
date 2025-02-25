use std::fmt::Debug;

use tracing::{Level, Subscriber};
use tracing_mock::{expect, subscriber};

#[tokio::test]
#[should_panic]
async fn repro_should_fail() {
    let (subscriber, handle) = subscriber_mock();

    {
        let _guard = tracing::subscriber::set_default(subscriber);
        tokio::spawn(async {
            tracing::error!("real message");
        });
        tokio::task::yield_now().await;
    }

    handle.assert_finished();
}

#[tokio::test]
#[should_panic = "[fails_as_expected]"]
async fn fails_as_expected() {
    let (subscriber, handle) = subscriber_mock();

    {
        let _guard = tracing::subscriber::set_default(subscriber);
        tracing::error!("real message");
    }

    handle.assert_finished();
}

fn subscriber_mock() -> (impl Subscriber, subscriber::MockHandle) {
    subscriber::mock()
        .event(
            expect::event()
                .at_level(Level::ERROR)
                .with_fields(expect::field("message").with_value(&debug_value("not real message"))),
        )
        .run_with_handle()
}

fn debug_value(message: impl Into<String>) -> tracing::field::DebugValue<Box<dyn Debug>> {
    struct Message(String);

    impl Debug for Message {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(&self.0)
        }
    }

    tracing::field::debug(Box::new(Message(message.into())))
}
