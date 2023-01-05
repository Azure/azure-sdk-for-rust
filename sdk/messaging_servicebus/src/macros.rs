macro_rules! cfg_not_wasm32 {
    ($($item:item)*) => {
        $(
            #[cfg(not(target_arch = "wasm32"))]
            $item
        )*
    }
}

#[allow(unused_macros)] // This macro will not be invoked if the target is wasm32
macro_rules! cfg_either_rustls_or_native_tls {
    ($($item:item)*) => {
        $(
            #[cfg(any(
                all(feature = "rustls", not(feature = "native-tls")),
                all(feature = "native-tls", not(feature = "rustls"))
            ))]
            #[cfg_attr(docsrs, doc(cfg(any(
                all(feature = "rustls", not(feature = "native-tls")),
                all(feature = "native-tls", not(feature = "rustls"))
            ))))]
            $item
        )*
    }
}
