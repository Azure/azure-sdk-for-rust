// cspell:disable
//
// The MIT License (MIT)
//
// Copyright (c) 2019 Yoshua Wuyts
// Copyright (c) 2017 http-rs authors
// Copyright (c) 2020 Jacob Brown
// Copyright (c) 2016-2018 Michael Tilli (Pyfisch) & `httpdate` contributors
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//
// Modifications:
//
// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

#[cfg(any(feature = "json", feature = "xml"))]
use std::fmt::{self, Display};
use std::str::FromStr;

/// HTTP request methods.
///
/// See also [Mozilla's documentation][Mozilla docs], the [RFC7231, Section 4][] and
/// [IANA's Hypertext Transfer Protocol (HTTP) Method Registry][HTTP Method Registry].
///
/// [Mozilla docs]: https://developer.mozilla.org/en-US/docs/Web/HTTP/Methods
/// [RFC7231, Section 4]: https://tools.ietf.org/html/rfc7231#section-4
/// [HTTP Method Registry]: https://www.iana.org/assignments/http-methods/http-methods.xhtml
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum Method {
    /// The DELETE method requests that the origin server remove the association between the target
    /// resource and its current functionality.
    ///
    /// See [RFC7231, Section 4.3.5][].
    ///
    /// [RFC7231, Section 4.3.5]: https://tools.ietf.org/html/rfc7231#section-4.3.5
    Delete,

    /// The GET method requests transfer of a current selected representation for the target
    /// resource.
    ///
    /// See [RFC7231, Section 4.3.1][].
    ///
    /// [RFC7231, Section 4.3.1]: https://tools.ietf.org/html/rfc7231#section-4.3.1
    Get,

    /// The HEAD method is identical to GET except that the server MUST NOT send a message body in
    /// the response.
    ///
    /// See [RFC7231, Section 4.3.2][].
    ///
    /// [RFC7231, Section 4.3.2]: https://tools.ietf.org/html/rfc7231#section-4.3.2
    Head,

    /// The PATCH method requests that a set of changes described in the request entity be applied
    /// to the resource identified by the Request- URI.
    ///
    /// See [RFC5789, Section 2][].
    ///
    /// [RFC5789, Section 2]: https://tools.ietf.org/html/rfc5789#section-2
    Patch,

    /// The POST method requests that the target resource process the representation enclosed in
    /// the request according to the resource's own specific semantics.
    ///
    /// For example, POST is used for the following functions (among others):
    ///
    ///   - Providing a block of data, such as the fields entered into an HTML form, to a
    ///     data-handling process;
    ///   - Posting a message to a bulletin board, newsgroup, mailing list, blog, or similar group
    ///     of articles;
    ///   - Creating a new resource that has yet to be identified by the origin server; and
    ///   - Appending data to a resource's existing representation(s).
    ///
    /// See [RFC7231, Section 4.3.3][].
    ///
    /// [RFC7231, Section 4.3.3]: https://tools.ietf.org/html/rfc7231#section-4.3.3
    Post,

    /// The PUT method requests that the state of the target resource be created or replaced with
    /// the state defined by the representation enclosed in the request message payload.
    ///
    /// See [RFC7231, Section 4.3.4][].
    ///
    /// [RFC7231, Section 4.3.4]: https://tools.ietf.org/html/rfc7231#section-4.3.4
    Put,
}

impl Method {
    /// Whether a method is considered "safe", meaning the request is essentially read-only.
    ///
    /// See [the spec](https://tools.ietf.org/html/rfc7231#section-4.2.1) for more details.
    pub fn is_safe(&self) -> bool {
        matches!(self, Method::Get | Method::Head)
    }
}

#[cfg(any(feature = "json", feature = "xml"))]
mod serde {
    use super::Method;
    use ::serde::{
        de::{Error as DeError, Unexpected, Visitor},
        Deserialize, Deserializer, Serialize, Serializer,
    };
    use std::{fmt, str::FromStr as _};

    struct MethodVisitor;

    impl Visitor<'_> for MethodVisitor {
        type Value = Method;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            write!(formatter, "a HTTP method &str")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: DeError,
        {
            match Method::from_str(v) {
                Ok(method) => Ok(method),
                Err(_) => Err(DeError::invalid_value(Unexpected::Str(v), &self)),
            }
        }
    }

    impl<'de> Deserialize<'de> for Method {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            deserializer.deserialize_str(MethodVisitor)
        }
    }

    impl Serialize for Method {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            serializer.serialize_str(self.as_ref())
        }
    }
}

impl Display for Method {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(AsRef::<str>::as_ref(self))
    }
}

impl FromStr for Method {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_uppercase().as_ref() {
            "DELETE" => Ok(Self::Delete),
            "GET" => Ok(Self::Get),
            "HEAD" => Ok(Self::Head),
            "PATCH" => Ok(Self::Patch),
            "POST" => Ok(Self::Post),
            "PUT" => Ok(Self::Put),
            _ => Err(crate::Error::new(
                crate::error::ErrorKind::DataConversion,
                format!("Invalid HTTP method: {s}"),
            )),
        }
    }
}

impl<'a> std::convert::TryFrom<&'a str> for Method {
    type Error = crate::Error;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}

impl AsRef<str> for Method {
    fn as_ref(&self) -> &'static str {
        match self {
            Self::Delete => "DELETE",
            Self::Get => "GET",
            Self::Head => "HEAD",
            Self::Patch => "PATCH",
            Self::Post => "POST",
            Self::Put => "PUT",
        }
    }
}

#[cfg(test)]
mod test {
    use super::Method;
    use std::collections::HashSet;

    #[cfg(any(feature = "json", feature = "xml"))]
    #[test]
    fn serde() -> Result<(), serde_json::Error> {
        assert_eq!(Method::Get, serde_json::from_str("\"GET\"")?);
        assert_eq!(Some("PATCH"), serde_json::to_value(Method::Patch)?.as_str());
        Ok(())
    }

    #[test]
    fn serde_fail() {
        serde_json::from_str::<Method>("\"ABC\"").expect_err("Did deserialize from invalid string");
    }

    #[test]
    fn names() -> Result<(), crate::Error> {
        let method_names = ["DELETE", "GET", "HEAD", "PATCH", "POST", "PUT"];

        let methods = method_names
            .iter()
            .map(|s| s.parse::<Method>())
            .collect::<Result<HashSet<_>, _>>()?;

        // check that we didn't accidentally map two methods to the same variant
        assert_eq!(methods.len(), method_names.len());

        // check that a method's name and the name it is parsed from match
        for method in methods {
            assert_eq!(method.as_ref().parse::<Method>()?, method);
        }

        Ok(())
    }
}
