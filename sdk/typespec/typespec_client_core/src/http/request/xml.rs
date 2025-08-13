// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

macro_rules! impl_try_from {
        ($t:ty) => {
            impl<T> ::core::convert::TryFrom<::std::vec::Vec<$t>> for $crate::http::RequestContent<T, $crate::http::XmlFormat> {
                type Error = $crate::Error;
                fn try_from(value: ::std::vec::Vec<$t>) -> $crate::Result<Self> {
                    Ok(Self {
                        body: $crate::xml::to_xml(&value)?.into(),
                        phantom: ::core::marker::PhantomData,
                    })
                }
            }

            impl<T> ::core::convert::TryFrom<::std::collections::HashMap<String, $t>> for $crate::http::RequestContent<T, $crate::http::XmlFormat> {
                type Error = $crate::Error;
                fn try_from(value: ::std::collections::HashMap<String, $t>) -> $crate::Result<Self> {
                    Ok(Self {
                        body: $crate::xml::to_xml(&value)?.into(),
                        phantom: ::core::marker::PhantomData,
                    })
                }
            }

            #[cfg(test)]
            impl<T> ::core::convert::TryFrom<::std::collections::BTreeMap<String, $t>> for $crate::http::RequestContent<T, $crate::http::XmlFormat> {
                type Error = $crate::Error;
                fn try_from(value: ::std::collections::BTreeMap<String, $t>) -> $crate::Result<Self> {
                    Ok(Self {
                        body: $crate::xml::to_xml(&value)?.into(),
                        phantom: ::core::marker::PhantomData,
                    })
                }
            }
        };

        ($($t:ty),*) => {
            $(impl_try_from!($t);)*
        };
    }

// We can't add a blanket implementation of TryFrom<T> for RequestContent<T, XmlFormat>,
// so we explicit support those scenarios needed for unbranded TypeSpec mimicking the Spector test suite.
impl_try_from!(bool);
impl_try_from!(&str, String);
impl_try_from!(i32, i64);
impl_try_from!(f32, f64);

#[cfg(test)]
mod tests {
    use crate::{
        http::{Body, RequestContent, XmlFormat},
        time::OffsetDateTime,
    };
    use std::collections::{BTreeMap, HashMap};
    use time::macros::datetime;

    #[test]
    fn spector_vec_bool() {
        let actual: RequestContent<Vec<bool>, XmlFormat> = vec![true, false].try_into().unwrap();
        assert_eq!(actual.body(), &Body::from_static(br#"[true,false]"#));
    }

    #[test]
    fn spector_vec_offset_date_time() {
        let actual: RequestContent<Vec<OffsetDateTime>, XmlFormat> =
            vec![datetime!(2022-08-26 18:38:00 UTC)].try_into().unwrap();
        assert_eq!(
            actual.body(),
            &Body::from_static(br#"["2022-08-26T18:38:00Z"]"#)
        );
    }

    #[test]
    fn spector_vec_duration() {
        let actual: RequestContent<Vec<String>, XmlFormat> =
            vec!["P123DT22H14M12.011S".to_string()].try_into().unwrap();
        assert_eq!(
            actual.body(),
            &Body::from_static(br#"["P123DT22H14M12.011S"]"#)
        );
    }

    #[test]
    fn spector_vec_f32() {
        let actual: RequestContent<Vec<f32>, XmlFormat> = vec![43.125f32].try_into().unwrap();
        assert_eq!(actual.body(), &Body::from_static(br#"[43.125]"#));
    }

    #[test]
    fn spector_vec_i64() {
        let actual: RequestContent<Vec<i64>, XmlFormat> =
            vec![9007199254740991i64, -9007199254740991i64]
                .try_into()
                .unwrap();
        assert_eq!(
            actual.body(),
            &Body::from_static(br#"[9007199254740991,-9007199254740991]"#)
        );
    }

    #[test]
    fn spector_vec_string() {
        let actual: RequestContent<Vec<String>, XmlFormat> =
            vec!["hello".to_string(), "".to_string()]
                .try_into()
                .unwrap();
        assert_eq!(actual.body(), &Body::from_static(br#"["hello",""]"#));
    }

    #[test]
    fn spector_dictionary_bool() {
        let actual: RequestContent<BTreeMap<String, bool>, XmlFormat> =
            BTreeMap::from_iter(vec![("k1".into(), true), ("k2".into(), false)])
                .try_into()
                .unwrap();
        assert_eq!(
            actual.body(),
            &Body::from_static(br#"{"k1":true,"k2":false}"#)
        );
    }

    #[test]
    fn spector_dictionary_offset_date_time() {
        let actual: RequestContent<HashMap<String, OffsetDateTime>, XmlFormat> =
            HashMap::from_iter(vec![("k1".into(), datetime!(2022-08-26 18:38:00 UTC))])
                .try_into()
                .unwrap();
        assert_eq!(
            actual.body(),
            &Body::from_static(br#"{"k1":"2022-08-26T18:38:00Z"}"#)
        );
    }

    #[test]
    fn spector_dictionary_duration() {
        let actual: RequestContent<HashMap<String, String>, XmlFormat> =
            HashMap::from_iter(vec![("k1".to_string(), "P123DT22H14M12.011S".to_string())])
                .try_into()
                .unwrap();
        assert_eq!(
            actual.body(),
            &Body::from_static(br#"{"k1":"P123DT22H14M12.011S"}"#)
        );
    }

    #[test]
    fn spector_dictionary_f32() {
        let actual: RequestContent<HashMap<String, f32>, XmlFormat> =
            HashMap::from_iter(vec![("k1".into(), 43.125f32)])
                .try_into()
                .unwrap();
        assert_eq!(actual.body(), &Body::from_static(br#"{"k1":43.125}"#));
    }

    #[test]
    fn spector_dictionary_i64() {
        let actual: RequestContent<BTreeMap<String, i64>, XmlFormat> = BTreeMap::from_iter(vec![
            ("k1".into(), 9007199254740991i64),
            ("k2".into(), -9007199254740991i64),
        ])
        .try_into()
        .unwrap();
        assert_eq!(
            actual.body(),
            &Body::from_static(br#"{"k1":9007199254740991,"k2":-9007199254740991}"#)
        );
    }

    #[test]
    fn spector_dictionary_string() {
        let actual: RequestContent<BTreeMap<String, String>, XmlFormat> =
            BTreeMap::from_iter(vec![
                ("k1".to_string(), "hello".to_string()),
                ("k2".to_string(), "".to_string()),
            ])
            .try_into()
            .unwrap();
        assert_eq!(
            actual.body(),
            &Body::from_static(br#"{"k1":"hello","k2":""}"#)
        );
    }
}
