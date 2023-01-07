#![cfg(all(test, feature = "test_e2e"))]

#[allow(unused_macros)]
macro_rules! cfg_not_wasm32 {
    ($($item:item)*) => {
        $(
            #[cfg(not(target_arch = "wasm32"))]
            $item
        )*
    }
}
