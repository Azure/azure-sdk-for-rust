// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! URL manipulation utilities.

use url::Url;

/// Extension trait for `Url` to provide additional URL manipulation methods.
pub trait UrlOperations {
    /// Appends a path segment to the URL's path, handling slashes appropriately and preserving query parameters.
    fn append_path(&mut self, p: &str);
}

impl UrlOperations for Url {
    fn append_path(&mut self, p: &str) {
        if self.path().len() == 1 {
            self.set_path(p);
        }
        else {
            match if self.path().ends_with('/') { 1 } else { 0 } + if p.starts_with('/') { 1 } else { 0 } {
                0 => self.set_path(&format!("{}/{}", self.path(), p).to_string()),
                1 => self.set_path(&(self.path().to_owned() + p)),
                _ => self.set_path(&(self.path()[..self.path().len() - 2].to_owned() + p)),
            }
        }
    }
}

#[test]
fn test_url_append_path() {
    {
        let mut url = Url::parse("https://www.microsoft.com?q=q").unwrap();
        url.append_path("alpha");
        assert_eq!(url.as_str(), "https://www.microsoft.com/alpha?q=q");
    }
    {
        let mut url = Url::parse("https://www.microsoft.com/?q=q").unwrap();
        url.append_path("alpha");
        assert_eq!(url.as_str(), "https://www.microsoft.com/alpha?q=q");
    }
    {
        let mut url = Url::parse("https://www.microsoft.com?q=q").unwrap();
        url.append_path("/alpha");
        assert_eq!(url.as_str(), "https://www.microsoft.com/alpha?q=q");
    }
    {
        let mut url = Url::parse("https://www.microsoft.com/?q=q").unwrap();
        url.append_path("/alpha");
        assert_eq!(url.as_str(), "https://www.microsoft.com/alpha?q=q");
    }
    {
        let mut url = Url::parse("https://www.microsoft.com?q=q").unwrap();
        url.append_path("alpha/");
        assert_eq!(url.as_str(), "https://www.microsoft.com/alpha/?q=q");
    }
    {
        let mut url = Url::parse("https://www.microsoft.com/?q=q").unwrap();
        url.append_path("alpha/");
        assert_eq!(url.as_str(), "https://www.microsoft.com/alpha/?q=q");
    }
    {
        let mut url = Url::parse("https://www.microsoft.com?q=q").unwrap();
        url.append_path("/alpha/");
        assert_eq!(url.as_str(), "https://www.microsoft.com/alpha/?q=q");
    }
    {
        let mut url = Url::parse("https://www.microsoft.com/?q=q").unwrap();
        url.append_path("/alpha/");
        assert_eq!(url.as_str(), "https://www.microsoft.com/alpha/?q=q");
    }
    {
        let mut url = Url::parse("https://www.microsoft.com/alpha?q=q").unwrap();
        url.append_path("beta");
        assert_eq!(url.as_str(), "https://www.microsoft.com/alpha/beta?q=q");
    }
    {
        let mut url = Url::parse("https://www.microsoft.com/alpha/?q=q").unwrap();
        url.append_path("beta");
        assert_eq!(url.as_str(), "https://www.microsoft.com/alpha/beta?q=q");
    }
    {
        let mut url = Url::parse("https://www.microsoft.com/alpha?q=q").unwrap();
        url.append_path("/beta");
        assert_eq!(url.as_str(), "https://www.microsoft.com/alpha/beta?q=q");
    }
    {
        let mut url = Url::parse("https://www.microsoft.com/alpha/?q=q").unwrap();
        url.append_path("/beta");
        assert_eq!(url.as_str(), "https://www.microsoft.com/alpha/beta?q=q");
    }
    {
        let mut url = Url::parse("https://www.microsoft.com/alpha?q=q").unwrap();
        url.append_path("beta/");
        assert_eq!(url.as_str(), "https://www.microsoft.com/alpha/beta/?q=q");
    }
    {
        let mut url = Url::parse("https://www.microsoft.com/alpha/?q=q").unwrap();
        url.append_path("beta/");
        assert_eq!(url.as_str(), "https://www.microsoft.com/alpha/beta/?q=q");
    }
    {
        let mut url = Url::parse("https://www.microsoft.com/alpha?q=q").unwrap();
        url.append_path("/beta/");
        assert_eq!(url.as_str(), "https://www.microsoft.com/alpha/beta/?q=q");
    }
    {
        let mut url = Url::parse("https://www.microsoft.com/alpha/?q=q").unwrap();
        url.append_path("/beta/");
        assert_eq!(url.as_str(), "https://www.microsoft.com/alpha/beta/?q=q");
    }
    {
        let mut url = Url::parse("https://www.microsoft.com?q=q").unwrap();
        url.append_path("/");
        assert_eq!(url.as_str(), "https://www.microsoft.com/?q=q");
    }
    {
        let mut url = Url::parse("https://www.microsoft.com/?q=q").unwrap();
        url.append_path("/");
        assert_eq!(url.as_str(), "https://www.microsoft.com/?q=q");
    }
    {
        let mut url = Url::parse("https://www.microsoft.com?q=q").unwrap();
        url.append_path("");
        assert_eq!(url.as_str(), "https://www.microsoft.com/?q=q");
    }
    {
        let mut url = Url::parse("https://www.microsoft.com?q=q").unwrap();
        url.append_path("");
        assert_eq!(url.as_str(), "https://www.microsoft.com/?q=q");
    }
    {
        let mut url = Url::parse("https://www.microsoft.com/alpha?q=q").unwrap();
        url.append_path("/");
        assert_eq!(url.as_str(), "https://www.microsoft.com/alpha/?q=q");
    }
    {
        let mut url = Url::parse("https://www.microsoft.com/alpha/?q=q").unwrap();
        url.append_path("/");
        assert_eq!(url.as_str(), "https://www.microsoft.com/alpha/?q=q");
    }
    {
        let mut url = Url::parse("https://www.microsoft.com/alpha?q=q").unwrap();
        url.append_path("");
        assert_eq!(url.as_str(), "https://www.microsoft.com/alpha?q=q");
    }
    {
        let mut url = Url::parse("https://www.microsoft.com/alpha/?q=q").unwrap();
        url.append_path("");
        assert_eq!(url.as_str(), "https://www.microsoft.com/alpha/?q=q");
    }
}
