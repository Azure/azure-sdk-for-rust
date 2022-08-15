/// Creates setter methods
///
/// The methods created are of the form `$name` that takes an argument of type `$typ`
/// and sets the field $name to result of calling `$transform` with the value of the argument.
///
/// In other words. The following macro call:
/// ```
/// # #[macro_use] extern crate azure_core;
/// struct MyStruct<'a> { foo: Option<&'a str> };
/// impl <'a> MyStruct<'a> {
///     setters! { foo: &'a str => Some(foo), }
/// }
/// ```
/// Roughly expands to:
/// ```
/// struct MyStruct<'a> { foo: Option<&'a str> };
/// impl <'a> MyStruct<'a> {
///     fn foo(self, foo: &'a str) -> Self {
///         Self {
///             foo: Some(foo),
///             ..self
///         }
///     }
/// }
/// ```
#[macro_export]
macro_rules! setters {
    (@single $name:ident : $typ:ty => $transform:expr) => {
        #[allow(clippy::redundant_field_names)]
        #[allow(clippy::needless_update)]
        #[allow(missing_docs)]
        pub fn $name<P: ::std::convert::Into<$typ>>(self, $name: P) -> Self {
            let $name: $typ = $name.into();
            Self  {
                $name: $transform,
                ..self
            }
        }
    };
    // Terminal condition
    (@recurse) => {};
    // Recurse without transform
    (@recurse $name:ident : $typ:ty, $($tokens:tt)*) => {
        $crate::setters! { @recurse $name: $typ => $name, $($tokens)* }
    };
    // Recurse with transform
    (@recurse $name:ident : $typ:ty => $transform:expr, $($tokens:tt)*) => {
        $crate::setters! { @single $name : $typ => $transform }
        $crate::setters! { @recurse $($tokens)* }
    };
    ($($tokens:tt)*) => {
        $crate::setters! { @recurse $($tokens)* }
    }
}

/// Helper for constructing operations
///
/// For the following code:
/// ```
/// # #[derive(Clone, Debug)]
/// # pub struct DatabaseClient;
/// # pub struct CreateCollectionResponse;
/// azure_core::operation! {
///    CreateCollection,
///    client: DatabaseClient,
///    collection_name: String,
///    ?consistency_level: u32
/// }
/// ```
///
/// The following code will be generated
///
/// ```
/// # use azure_core::setters;
/// # use azure_core::Context;
/// # #[derive(Clone, Debug)]
/// # pub struct DatabaseClient;
/// # pub struct CreateCollectionResponse;
/// #[derive(Debug, Clone)]
/// pub struct CreateCollectionBuilder {
///     client: DatabaseClient,
///     collection_name: String,
///     consistency_level: Option<u32>,
///     context: Context,
/// }
///
/// impl CreateCollectionBuilder {
///     pub(crate) fn new(
///         client: DatabaseClient,
///         collection_name: String,
///     ) -> Self {
///         Self {
///             client,
///             collection_name,
///             consistency_level: None,
///             context: Context::new(),
///         }
///     }
///
///     setters! {
///         consistency_level: u32 => Some(consistency_level),
///         context: Context => context,
///     }
/// }
///
/// #[cfg(feature = "into_future")]
/// impl std::future::IntoFuture for CreateCollectionBuilder {
///     type IntoFuture = CreateCollection;
///     type Output = <CreateCollection as std::future::Future>::Output;
///     fn into_future(self) -> Self::IntoFuture {
///         Self::into_future(self)
///     }
/// }
///
/// /// The future returned by calling `into_future` on the builder.
/// pub type CreateCollection =
///     futures::future::BoxFuture<'static, azure_core::Result<CreateCollectionResponse>>;
/// ```
///
/// Additionally, `#[stream]` can be used before the operation name to generate code appropriate for list operations
/// and `#[skip]` can be used at the end of the list of options for options where we should not generate a setter.
#[macro_export]
macro_rules! operation {
    // Construct the builder.
    (@builder
        $(#[$outer:meta])*
        // The name of the operation and any generic params along with their constraints
        $name:ident<$($generic:ident: $first_constraint:ident $(+ $constraint:ident)* ),* $(+ $lt:lifetime)?>,
        // The client
        client: $client:ty,
        // The required fields that will be used in the constructor
        @required
        $($required:ident: $rtype:ty,)*
        // The optional fields that will have generated setters
        @optional
        $($optional:ident: $otype:ty,)*
        // The optional fields which won't have generated setters
        @nosetter
        $($nosetter:ident: $nstype:ty),*
        ) => {
        azure_core::__private::paste! {
        #[derive(Debug, Clone)]
        $(#[$outer])*
        pub struct [<$name Builder>]<$($generic)*> {
            client: $client,
            $($required: $rtype,)*
            $($optional: Option<$otype>,)*
            $($nosetter: Option<$nstype>,)*
            context: azure_core::Context,
        }

        /// Setters for the various options for this builder
        impl <$($generic: $first_constraint $(+ $constraint)* )*>[<$name Builder>]<$($generic),*> {
            pub(crate) fn new(
                client: $client,
                $($required: $rtype,)*
            ) -> Self {
                Self {
                    client,
                    $($required,)*
                    $($optional: None,)*
                    $($nosetter: None,)*
                    context: azure_core::Context::new(),
                }
            }

            $crate::setters! {
                $($optional: $otype => Some($optional),)*
                context: azure_core::Context => context,
            }
        }
        }
    };
    // `operation! { #[stream] ListUsers, client: UserClient, ?consistency_level: ConsistencyLevel }`
    (#[stream] $(#[$outer:meta])* $name:ident,
        client: $client:ty,
        $($required:ident: $rtype:ty,)*
        $(?$optional:ident: $otype:ty),*) => {
            $crate::operation!{
                @builder
                $(#[$outer])*
                $name<>,
                client: $client,
                @required
                $($required: $rtype,)*
                @optional
                $($optional: $otype,)*
                @nosetter
            }
    };
    (#[stream] $(#[$outer:meta])*
        $name:ident,
        client: $client:ty,
        $($required:ident: $rtype:ty,)*
        $(?$optional:ident: $otype:ty,)*
        $(#[skip]$nosetter:ident: $nstype:ty),*
    ) => {
            $crate::operation!{
                @builder
                $(#[$outer])*
                $name<>,
                client: $client,
                @required
                $($required: $rtype,)*
                @optional
                $($optional: $otype,)*
                @nosetter
                $($nosetter: $nstype),*
            }
    };
    // Construct a builder and the `Future` related code
    ($(#[$outer:meta])* $name:ident<$($generic:ident: $first_constraint:ident $(+ $constraint:ident)* ),* $(+ $lt:lifetime)?>,
        client: $client:ty,
        @required
        $($required:ident: $rtype:ty,)*
        @optional
        $($optional:ident: $otype:ty,)*
        @nosetter
        $($nosetter:ident: $nstype:ty),*
        ) => {
        $crate::operation! {
            @builder
            $(#[$outer])*
            $name<$($generic: $first_constraint $(+ $constraint)*),* $(+ $lt)*>,
            client: $client,
            @required
            $($required: $rtype,)*
            @optional
            $($optional: $otype,)*
            @nosetter
            $($nosetter: $nstype),*
        }
        $crate::future!($name);
        azure_core::__private::paste! {
        #[cfg(feature = "into_future")]
        impl <$($generic: $first_constraint $(+ $constraint)*)* $(+ $lt)*> std::future::IntoFuture for [<$name Builder>]<$($generic),*> {
            type IntoFuture = $name;
            type Output = <$name as std::future::Future>::Output;
            fn into_future(self) -> Self::IntoFuture {
                Self::into_future(self)
            }
        }
        }
    };
    // `operation! { CreateUser, client: UserClient, ?consistency_level: ConsistencyLevel }`
    ($(#[$outer:meta])* $name:ident,
        client: $client:ty,
        $($required:ident: $rtype:ty,)*
        $(?$optional:ident: $otype:ty),*) => {
            $crate::operation!{
                $(#[$outer])*
                $name<>,
                client: $client,
                @required
                $($required: $rtype,)*
                @optional
                $($optional: $otype,)*
                @nosetter
            }
    };
    // `operation! { CreateDocument<D: Serialize>, client: UserClient, ?consistency_level: ConsistencyLevel, ??other_field: bool }`
    ($(#[$outer:meta])* $name:ident<$($generic:ident: $first_constraint:ident $(+ $constraint:ident)*),* $(+ $lt:lifetime)?>,
        client: $client:ty,
        $($required:ident: $rtype:ty,)*
        $(?$optional:ident: $otype:ty,)*
        $(#[skip] $nosetter:ident: $nstype:ty),*) => {
            $crate::operation!{
                $(#[$outer])*
                $name<$($generic: $first_constraint $(+ $constraint)*),* $(+ $lt)*>,
                client: $client,
                @required
                $($required: $rtype,)*
                @optional
                $($optional: $otype,)*
                @nosetter
                $($nosetter: $nstype),*
            }
    }
}

/// Declare a `Future` with the given name
///
/// `Future::Output` will be set to `azure_core::Result<$NAMEResponse>.
/// The `Future` will be `Send` for all targets but `wasm32`.
#[macro_export]
macro_rules! future {
    ($name:ident) => {
        $crate::future!($name<>);
    };
    ($name:ident<$($generic:ident)?>) => {
        azure_core::__private::paste! {
        #[cfg(target_arch = "wasm32")]
        pub type $name<$($generic)*> =
            std::pin::Pin<std::boxed::Box<dyn std::future::Future<Output = azure_core::Result<[<$name Response>]<$($generic)*>>> + 'static>>;
        #[cfg(not(target_arch = "wasm32"))]
        pub type $name<$($generic)*> =
            futures::future::BoxFuture<'static, azure_core::Result<[<$name Response>]<$($generic)*>>>;
        }
    };
}

/// The following macro invocation:
/// ```
/// # #[macro_use] extern crate azure_core;
/// request_header!(
///     /// Builds a client request id header
///     ClientRequestId, CLIENT_REQUEST_ID,
/// );
/// ```
/// Turns into a Header value used to construct requests.
#[macro_export]
macro_rules! request_header {
    ($(#[$outer:meta])* $name:ident, $header:ident) => {
        $crate::request_header!($name, $header,);
    };
    ($(#[$outer:meta])* $name:ident, $header:ident, $(($variant:ident, $value:expr)), *) => {
        $crate::request_option!($(#[$outer])* $name);
        impl $name {
            $(
                pub const $variant: $name = $name::from_static($value);
            )*
        }
        impl $crate::headers::Header for $name {
            fn name(&self) -> $crate::headers::HeaderName {
                $crate::headers::$header
            }

            fn value(&self) -> $crate::headers::HeaderValue {
                $crate::headers::HeaderValue::from_cow(self.0.clone())
            }
        }
    };
}

/// The following macro invocation:
/// ```
/// # #[macro_use] extern crate azure_core;
/// request_query!(Prefix, "prefix");
/// ```
/// Turns into a request query option used to construct requests
#[macro_export]
macro_rules! request_query {
    ($(#[$outer:meta])* $name:ident, $option:expr) => {
        $crate::request_option!($(#[$outer])* $name);
        impl $crate::AppendToUrlQuery for $name {
            fn append_to_url_query(&self, url: &mut url::Url) {
                url.query_pairs_mut().append_pair($option, &self.0);
            }
        }
    };
}

/// The following macro invocation:
/// ```
/// # #[macro_use] extern crate azure_core;
/// request_option!(Prefix);
/// ```
/// Turns into a request option useable either as a header or as a query string.
#[macro_export]
macro_rules! request_option {
    ($(#[$outer:meta])* $name:ident) => {
        #[derive(Debug, Clone)]
        $(#[$outer])*
        pub struct $name(std::borrow::Cow<'static, str>);

        impl $name {
            pub fn new<S>(s: S) -> Self
            where
                S: Into<std::borrow::Cow<'static, str>>,
            {
                Self(s.into())
            }

            pub const fn from_static(s: &'static str) -> Self {
                Self(std::borrow::Cow::Borrowed(s))
            }
        }

        impl<S> From<S> for $name
        where
            S: Into<std::borrow::Cow<'static, str>>,
        {
            fn from(s: S) -> Self {
                Self::new(s)
            }
        }
    };
}

/// The following macro invocation:
/// ```
/// # #[macro_use] extern crate azure_core;
/// create_enum!(Words, (Pollo, "Pollo"), (Bianco, "Bianco"), (Giallo, "Giallo"));
/// ```
/// Turns into a struct where each variant can be turned into and construct from the corresponding string.
#[macro_export]
macro_rules! create_enum {
    ($name:ident, $(($variant:ident, $value:expr)), *) => (
        #[derive(Debug, PartialEq, Eq, PartialOrd, Clone, Copy)]
        pub enum $name {
            $(
                $variant,
            )*
        }

        impl ::std::convert::From<$name> for &'static str {
            fn from(e: $name) -> Self {
                match e {
                    $(
                        $name::$variant => $value,
                    )*
                }
            }
        }

        impl $crate::parsing::FromStringOptional<$name> for $name {
            fn from_str_optional(s : &str) -> $crate::error::Result<$name> {
                s.parse::<$name>()
            }
        }

        impl ::std::str::FromStr for $name {
            type Err = $crate::error::Error;

            fn from_str(s: &str) -> $crate::error::Result<$name> {
                match s {
                    $(
                        $value => Ok($name::$variant),
                    )*
                    _ => Err($crate::error::Error::with_message($crate::error::ErrorKind::DataConversion, || format!("unknown variant of {} found: \"{}\"",
                        stringify!($name),
                         s
                    )))
                }
            }
        }

        impl<'de> serde::Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> ::core::result::Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                let s = String::deserialize(deserializer)?;

                match s.as_ref() {
                    $(
                        $value => Ok(Self::$variant),
                    )*
                    _ => Err(serde::de::Error::custom("unsupported value")),
                }
            }
        }

        impl serde::Serialize for $name {
            fn serialize<S>(&self, s: S) -> ::core::result::Result<S::Ok, S::Error>
            where S: serde::Serializer {
                return s.serialize_str(&self.to_string())
            }
        }

        impl ::std::convert::AsRef<str> for $name {
            fn as_ref(&self) -> &str {
                 match *self {
                    $(
                        $name::$variant => $value,
                    )*
                }
            }
        }

        impl ::std::fmt::Display for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                match *self {
                    $(
                        $name::$variant => write!(f, "{}", $value),
                    )*
                }
            }
        }
    )
}

#[cfg(test)]
mod test {
    create_enum!(Colors, (Black, "Black"), (White, "White"), (Red, "Red"));
    create_enum!(ColorsMonochrome, (Black, "Black"), (White, "White"));

    struct Options {
        a: Option<String>,
        b: u32,
    }

    #[allow(dead_code)]
    impl Options {
        setters! {
            a: String => Some(a),
            b: u32 => b,
        }
    }

    impl Default for Options {
        fn default() -> Self {
            Options { a: None, b: 1 }
        }
    }

    #[test]
    fn test_color_parse_1() {
        let color = "Black".parse::<Colors>().unwrap();
        assert_eq!(Colors::Black, color);
    }

    #[test]
    fn test_color_parse_2() {
        let color = "White".parse::<ColorsMonochrome>().unwrap();
        assert_eq!(ColorsMonochrome::White, color);
    }

    #[test]
    fn test_color_parse_err_1() {
        "Red".parse::<ColorsMonochrome>().unwrap_err();
    }

    #[test]
    fn test_setters() {
        let options = Options::default().a("test".to_owned());

        assert_eq!(Some("test".to_owned()), options.a);
        assert_eq!(1, options.b);
    }
}
