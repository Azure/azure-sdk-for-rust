use fe2o3_amqp::session::SessionHandle;
use fe2o3_amqp_management::{MgmtClient, error::Error as ManagementError, Request, Response};
use fe2o3_amqp_types::messaging::FromBody;

use crate::util::sharable::Sharable;

#[derive(Debug)]
pub(crate) struct AmqpManagementLink {
    pub(crate) session_handle: SessionHandle<()>,
    pub(crate) client: MgmtClient,
}

impl Sharable<AmqpManagementLink> {
    pub(crate) async fn call<Req, Res>(&mut self, request: Req) -> Result<Res, ManagementError>
    where
        Req: Request<Response = Res>,
        Res: Response,
        Res::Error: Into<ManagementError>,
        for<'de> Res::Body: FromBody<'de> + std::fmt::Debug + Send,
    {
        match self {
            Sharable::Owned(l) => l.client.call(request).await,
            Sharable::Shared(lock) => {
                let mut lock = lock.write().await;
                lock.client.call(request).await
            },
            Sharable::None => todo!(),
        }
    }
}
