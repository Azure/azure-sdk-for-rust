use std::iter::IntoIterator;

#[derive(Serialize, Debug)]
pub struct PartitionKey<'a> {
    pk: Option<Vec<&'a str>>,
}

impl<'a> PartitionKey<'a> {
    pub fn new() -> PartitionKey<'a> {
        PartitionKey { pk: None }
    }

    pub fn push(&mut self, key: &'a str) {
        match self.pk {
            Some(ref mut p) => p.push(key),
            None => self.pk = Some(vec![key]),
        }
    }
}

impl<'a> IntoIterator for PartitionKey<'a> {
    type Item = &'a str;
    type IntoIter = ::std::vec::IntoIter<&'a str>;

    fn into_iter(self) -> Self::IntoIter {
        match self.pk {
            Some(p) => p.into_iter(),
            None => ::std::vec::Vec::<&str>::new().into_iter(),
        }
    }
}
