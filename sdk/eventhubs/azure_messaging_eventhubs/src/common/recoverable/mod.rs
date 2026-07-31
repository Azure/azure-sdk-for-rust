// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

mod claims_based_security;
mod connection;
mod management;
pub(crate) mod receiver;
mod sender;

pub(crate) use connection::RecoverableConnection;
pub(crate) use sender::RecoverableSender;

/// How many times a generation-guarded cache fill retries when a recovery races
/// it (#4454).
///
/// A resource that is attached while a recovery runs is bound to the connection
/// that the recovery tore down, so the cache discards it and attaches again
/// against the new generation. This bounds that loop: a storm of back-to-back
/// recoveries surfaces an error instead of spinning forever.
///
/// The bound is generous because recovery is rare, and each pass makes forward
/// progress against a newer connection. To reach it, the connection must go down
/// faster than it can come up.
///
/// One recovery costs one pass in the usual case. It costs two for a task that
/// captures its generation inside the recovery, because such a capture is odd and
/// is rejected on parity, so the task attaches once more before it reaches a
/// settled generation. The bound therefore covers at least four back-to-back
/// recoveries in the worst case.
///
/// Every generation-guarded cache in this crate uses this one value, so the
/// policy stays the same for connections, senders, receivers, and tokens.
pub(crate) const MAX_GENERATION_RETRIES: usize = 8;
