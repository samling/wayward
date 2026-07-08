use futures::channel::mpsc;
use futures::future::FutureExt;
use futures::stream::{self, StreamExt};

use super::merged_update_stream;

#[test]
fn merged_update_stream_waits_when_no_items_are_present() {
    let mut updates = merged_update_stream(Vec::new());

    assert!(updates.next().now_or_never().is_none());
}

#[test]
fn merged_update_stream_wakes_for_any_item_update() {
    let (sender, receiver) = mpsc::unbounded();
    let stream = receiver.map(|_| ()).boxed();
    let mut updates = merged_update_stream(vec![stream]);

    sender.unbounded_send(()).unwrap();

    assert_eq!(futures::executor::block_on(updates.next()), Some(()));
}

#[test]
fn merged_update_stream_waits_when_streams_are_pending() {
    let stream = stream::pending().boxed();
    let mut updates = merged_update_stream(vec![stream]);

    assert!(updates.next().now_or_never().is_none());
}
