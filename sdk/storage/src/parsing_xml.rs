use azure_core::error::{Error, ErrorKind};
use azure_core::parsing::FromStringOptional;
use xml::Element;
use xml::Xml::{CharacterNode, ElementNode};

#[inline]
pub fn traverse_single_must<'a>(
    node: &'a Element,
    path: &[&str],
) -> azure_core::Result<&'a Element> {
    let vec = traverse(node, path, false)?;
    if vec.len() > 1 {
        return Err(Error::with_message(ErrorKind::Other, || {
            format!("multiple node: {}", path[path.len() - 1])
        }));
    }

    Ok(vec[0])
}

pub fn traverse_single_optional<'a>(
    node: &'a Element,
    path: &[&str],
) -> azure_core::Result<Option<&'a Element>> {
    let vec = traverse(node, path, true)?;
    if vec.len() > 1 {
        return Err(Error::with_message(ErrorKind::Other, || {
            format!("multiple node: {}", path[path.len() - 1])
        }));
    }

    if vec.is_empty() {
        return Ok(None);
    }

    Ok(Some(vec[0]))
}

#[inline]
pub fn traverse<'a>(
    node: &'a Element,
    path: &[&str],
    ignore_empty_leaf: bool,
) -> azure_core::Result<Vec<&'a Element>> {
    trace!(
        "traverse(node == {:?}, path == {:?}, ignore_empty_leaf == {})",
        node,
        path,
        ignore_empty_leaf
    );

    if path.is_empty() {
        return Ok(vec![node]);
    }

    let mut curnode = node;

    for (x, item) in path.iter().enumerate() {
        let vec = find_subnodes(curnode, item);
        if vec.is_empty() {
            if (x + 1) >= path.len() && ignore_empty_leaf {
                return Ok(vec);
            }
            return Err(Error::with_message(ErrorKind::Other, || {
                format!("path not found: {}", *item)
            }));
        }

        if vec.len() > 1 && (x + 1) < path.len() {
            return Err(Error::with_message(ErrorKind::Other, || {
                format!("multiple node: {}", *item)
            }));
        }

        if (x + 1) >= path.len() {
            return Ok(vec);
        }

        curnode = vec[0];
    }

    unreachable!();
}

#[inline]
pub fn find_subnodes<'a>(node: &'a Element, subnode: &str) -> Vec<&'a Element> {
    node.children
        .iter()
        .filter(|x| match **x {
            ElementNode(ref mynode) => mynode.name == subnode,
            _ => false,
        })
        .map(|x| match *x {
            ElementNode(ref mynode) => mynode,
            _ => unreachable!(),
        })
        .collect::<Vec<_>>()
}

#[inline]
pub fn inner_text(node: &Element) -> azure_core::Result<&str> {
    for child in &node.children {
        match *child {
            CharacterNode(ref txt) => return Ok(txt),
            _ => continue,
        };
    }

    Ok("")

    //debug!("\n!!! node == {}", node);
    //Err(TraversingError::TextNotFound)
}

#[inline]
pub fn cast_optional<'a, T>(node: &'a Element, path: &[&str]) -> azure_core::Result<Option<T>>
where
    T: FromStringOptional<T>,
{
    match traverse_single_optional(node, path)? {
        Some(e) => match inner_text(e) {
            Ok(txt) => Ok(Some(T::from_str_optional(txt)?)),
            Err(_) => Ok(None),
        },
        None => Ok(None),
    }
}

#[inline]
pub fn cast_must<'a, T>(node: &'a Element, path: &[&str]) -> azure_core::Result<T>
where
    T: FromStringOptional<T>,
{
    let node = traverse_single_must(node, path)?;
    let itxt = inner_text(node)?;
    T::from_str_optional(itxt)
}

#[cfg(test)]
mod test {
    use xml::Element;

    const XML: &str = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>
<EnumerationResults \
                               ServiceEndpoint=\"http://mindrust.blob.core.windows.net/\">
  \
                               <Containers>
    <Container>
      <Name>pippo</Name>
      \
                               <Properties>
        <Last-Modified>Mon, 23Nov 2015 21:12:35 \
                               GMT</Last-Modified>
        <Etag>\"0x8D2F44ACF757699\"</Etag>
        \
                               <LeaseStatus>unlocked</LeaseStatus>
        \
                               <LeaseState>available</LeaseState>
                               \
                               <SomeNumber>256</SomeNumber>
      </Properties>
    </Container>
    \
                               <Container>
      <Name>pluto</Name>
      <Properties>
        \
                               <Last-Modified>Mon, 23Nov 2015 21:12:35 GMT</Last-Modified>
        \
                               <Etag>\"0xAA2F44ACF757699\"</Etag>
        \
                               <LeaseStatus>locked</LeaseStatus>
        \
                               <LeaseState>available</LeaseState>
      </Properties>
    \
                               </Container>
  </Containers>
  <NextMarker />
</EnumerationResults>";

    #[test]
    fn test_cast_optional_1() {
        let elem: Element = XML.parse().unwrap();

        let sub1 = super::traverse(&elem, &["Containers", "Container"], false).unwrap();

        {
            let num = super::cast_optional::<u64>(sub1[0], &["Properties", "SomeNumber"]).unwrap();
            assert_eq!(Some(256u64), num);
        }

        {
            let num2 = super::cast_optional::<u64>(sub1[1], &["Properties", "SomeNumber"]).unwrap();
            assert_eq!(None, num2);
        }
    }

    #[test]
    fn test_first_1() {
        let elem: Element = XML.parse().unwrap();

        let sub1 = super::find_subnodes(&elem, "Containers");
        assert_eq!(1, sub1.len());

        let sub2 = super::find_subnodes(sub1[0], "Container");
        assert_eq!(2, sub2.len());
    }

    #[test]
    fn test_inner_2() {
        let elem: Element = XML.parse().unwrap();

        let mut sub = super::find_subnodes(&elem, "Containers");
        sub = super::find_subnodes(sub[0], "Container");
        sub = super::find_subnodes(sub[0], "Properties");
        sub = super::find_subnodes(sub[0], "LeaseStatus");

        if let Ok(inner) = super::inner_text(sub[0]) {
            assert_eq!(inner, "unlocked");
        } else {
            panic!("should have found CharacterNode");
        }
    }

    #[test]
    fn test_traverse_1() {
        let elem: Element = XML.parse().unwrap();

        let mut res = super::traverse(&elem, &["Containers", "Container"], false).unwrap();
        res = super::traverse(res[0], &["Properties", "LeaseStatus"], false).unwrap();

        if let Ok(inner) = super::inner_text(res[0]) {
            assert_eq!(inner, "unlocked");
        } else {
            panic!("should have found CharacterNode");
        }
    }

    #[test]
    fn test_traverse_2() {
        let elem: Element = XML.parse().unwrap();

        let mut res = super::traverse(&elem, &["Containers", "Container"], false).unwrap();
        res = super::traverse(res[1], &["Properties", "LeaseStatus"], false).unwrap();

        if let Ok(inner) = super::inner_text(res[0]) {
            assert_eq!(inner, "locked");
        } else {
            panic!("should have found CharacterNode");
        }
    }

    #[test]
    fn test_traverse_single_must_1() {
        let elem: Element = XML.parse().unwrap();

        let res = super::traverse(&elem, &["Containers", "Container"], false).unwrap();
        let res_final =
            super::traverse_single_must(res[1], &["Properties", "LeaseStatus"]).unwrap();

        if let Ok(inner) = super::inner_text(res_final) {
            assert_eq!(inner, "locked");
        } else {
            panic!("should have found CharacterNode");
        }
    }

    #[test]
    fn test_traverse_single_optional_1() {
        let elem: Element = XML.parse().unwrap();

        let res = super::traverse(&elem, &["Containers", "Container"], false).unwrap();
        let res_final =
            super::traverse_single_optional(res[1], &["Properties", "Pinocchio"]).unwrap();

        assert_eq!(res_final, None);
    }
}
