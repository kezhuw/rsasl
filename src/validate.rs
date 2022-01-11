use std::any::{Any, TypeId};
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug)]
pub struct ValidationDefinition {
    pub name: &'static str,
    pub display: &'static str,
    source: &'static str,
}
impl ValidationDefinition {
    pub const fn new(name: &'static str, display: &'static str) -> Self {
        let source = env!("CARGO_CRATE_NAME");
        Self { name, display, source }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Validation {
    witness: *const ValidationDefinition,
}
impl Debug for Validation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let def = unsafe { &*self.witness };
        f.debug_tuple("Validation")
         .field(&def.name)
         .finish()
    }
}
impl Display for Validation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let def = unsafe { &*self.witness };
        f.write_str(def.display)
    }
}
impl Validation {
    pub const fn new(definition: &'static ValidationDefinition) -> Self {
        let witness = definition as *const _;
        Self { witness }
    }
}

/// Validation using Username/Password combination
///
/// An application MUST in this case check if the given [`Password`] matches the given
/// [`AuthId`] and SHOULD check if the [`AuthzId`] is empty (if authorization id handling is not
/// implemented) or if the given user is allowed to authorize as the given authorization id (if
/// handling is implemented).
pub const SIMPLE: Validation = Validation::new(&ValidationDefinition::new(
    "simple", "username/password based authentication"
));

pub const OPENID20: Validation = Validation::new(&ValidationDefinition::new(
    "openid20", "validate the users oidc token"
));

pub const SAML20: Validation = Validation::new(&ValidationDefinition::new(
    "saml20", "validate the users saml token"
));

pub const SECURID: Validation = Validation::new(&ValidationDefinition::new(
    "securid", "validate the user using SecurID"
));

/// GSSAPI validation
///
/// This validation is called at the end of a GSSAPI validation. The properties available depend
/// on the exact GSSAPI mechanism but with Kerberos V5 (the ubiquitous default) [`Authzid`] and
/// [`GssapiDisplayName`] should be checked containing the authZid and principal name respectively.
pub const GSSAPI: Validation = Validation::new(&ValidationDefinition::new(
    "gssapi", "validate the users gssapi authentication"
));

/// Anonymous validation
///
/// The anonymous authentication allows clients to specify a "token" of 0-255 utf-8 code points
/// to be provided to the server. This token can be accessed using the [`AnonymousToken`] property.
pub const ANONYMOUS: Validation = Validation::new(&ValidationDefinition::new(
    "anonymous", "validate the provided anonymous token"
));

/// External validation
///
/// This validation relies on external information outside the protocol connection itself, e.g.
/// TLS client certificates, originating UID/GID of an UNIX socket connection, or source IP. No
/// properties are provided.
pub const EXTERNAL: Validation = Validation::new(&ValidationDefinition::new(
    "external", "validate the connection using External information"
));

#[cfg(test)]
mod tests {
    use std::any::TypeId;
    use std::collections::HashMap;
    use std::ptr::null_mut;
    use crate::{Callback, eq_type, Mechname, SASLError};
    use crate::SASLError::NoValidate;
    use crate::session::SessionData;
    use super::*;

    #[test]
    fn test_validation_callback() {
        struct TestCallback;
        impl Callback for TestCallback {
            fn validate(&self, session: &mut SessionData, validation: Validation, mechanism: &Mechname)
                -> Result<(), SASLError>
            {
                match validation {
                    SIMPLE => {
                        println!("Hey I know how to validate simple!");
                        Ok(())
                    }
                    _ => {
                        println!("Huh, I don't know how to validate {} ({:?}) for mech {}",
                                 validation,
                                 validation,
                                 mechanism);
                        Err(NoValidate { validation })
                    }
                }
            }
        }

        let cb = TestCallback;
        let s = unsafe { &mut *null_mut() as &mut SessionData };
        let mech = Mechname::try_parse(b"LOGIN").unwrap();
        cb.validate(s, SIMPLE, mech).unwrap();
        cb.validate(s, OPENID20, mech).unwrap_err();
    }

    #[test]
    fn test_matchable() {
        // This is an alternative idea for how to do Validation and possibly Property. To be
        // evaluated
        trait Foo {}
        #[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
        struct MatchTest {
            inner: *const dyn Foo,
        }
        struct FooI;
        impl Foo for FooI {}
        struct FooJ;
        impl Foo for FooJ {}
        struct FooK;
        impl Foo for FooK {}
        const FOOC: &'static dyn Foo = &FooI;
        const FOOD: &'static dyn Foo = &FooJ;
        const FOOE: &'static dyn Foo = &FooK;
        const FOOI: MatchTest = MatchTest { inner: FOOC as *const dyn Foo };
        const FOOJ: MatchTest = MatchTest { inner: FOOD as *const dyn Foo };
        const FOOK: MatchTest = MatchTest { inner: FOOE as *const dyn Foo };

        fn t(t: MatchTest) {
            match t {
                FOOI => println!("known, fooi!"),
                FOOJ => println!("known, fooj!"),
                _ => println!("other"),
            }
        }

        t(FOOI);
        t(FOOJ);
        t(FOOK)
    }
}