use std::sync::Arc;

use wayle_notification::NotificationService;
use zbus::{Connection, connection::Builder};

const BUS_NAME: &str = "dev.sboynton.wayward.Control";
const OBJECT_PATH: &str = "/dev/sboynton/wayward/Control";
const INTERFACE_NAME: &str = "dev.sboynton.wayward.Control";

pub(crate) async fn start(
    notification: Option<Arc<NotificationService>>,
) -> zbus::Result<Connection> {
    Builder::session()?
        .name(BUS_NAME)?
        .serve_at(OBJECT_PATH, Control { notification })?
        .build()
        .await
}

pub(crate) async fn dismiss_notifications() -> zbus::Result<()> {
    let connection = Connection::session().await?;
    let proxy = zbus::Proxy::new(&connection, BUS_NAME, OBJECT_PATH, INTERFACE_NAME).await?;
    proxy.call::<_, _, ()>("DismissNotifications", &()).await
}

struct Control {
    notification: Option<Arc<NotificationService>>,
}

#[zbus::interface(name = "dev.sboynton.wayward.Control")]
impl Control {
    fn dismiss_notifications(&self) {
        let Some(service) = self.notification.as_ref() else {
            tracing::info!(
                "Cannot dismiss notification popups because notification service is unavailable"
            );
            return;
        };

        let popup_ids: Vec<_> = service
            .popups
            .get()
            .iter()
            .map(|notification| notification.id)
            .collect();

        for id in popup_ids {
            service.dismiss_popup(id);
        }
    }
}
