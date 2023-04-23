use fe2o3_amqp::session::SessionHandle;
use fe2o3_amqp_management::MgmtClient;

#[derive(Debug)]
pub(crate) struct AmqpManagementLink {
    pub(crate) session_handle: SessionHandle<()>,
    pub(crate) client: MgmtClient,
}
