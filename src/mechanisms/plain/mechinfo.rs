use crate::gsasl::gsasl::{CMechanismStateKeeper, MechanismVTable};
use crate::{Mechanism, Mechname};
use crate::mechanisms::plain::{client, server};

#[cfg(feature = "registry_static")]
use crate::registry::{distributed_slice, MECHANISMS};
#[cfg_attr(feature = "registry_static", distributed_slice(MECHANISMS))]
pub static PLAIN: Mechanism = Mechanism {
    mechanism: &Mechname::const_new_unchecked(b"PLAIN"),
    priority: 300,
    client: Some(|_sasl| Ok(Box::new(client::Plain))),
    server: Some(|_sasl| Ok(Box::new(server::Plain))),
};