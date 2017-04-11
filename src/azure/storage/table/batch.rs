use rustc_serialize::{Encodable, json};
use super::TABLE_SUFFIX;

const BATCH_BEGIN: &'static str = r#"--batch_a1e9d677-b28b-435e-a89e-87e6a768a431
Content-Type: multipart/mixed; boundary=changeset_8a28b620-b4bb-458c-a177-0959fb14c977

"#;
const BATCH_END: &'static str = "--batch_a1e9d677-b28b-435e-a89e-87e6a768a431\n";
const CHANGESET_BEGIN: &'static str = r#"--changeset_8a28b620-b4bb-458c-a177-0959fb14c977
Content-Type: application/http
Content-Transfer-Encoding: binary

"#;
const CHANGESET_END: &'static str = "--changeset_8a28b620-b4bb-458c-a177-0959fb14c977--\n";
const UPDATE_HEADER: &'static str = "Content-Type: application/json\n\n";

pub struct BatchItem<T: Encodable>(String, String, Option<T>);

pub fn generate_batch_payload<T: Encodable>(account: &str,
                                            table: &str,
                                            items: &[BatchItem<T>],
                                            use_https: bool)
                                            -> String {
    let mut payload: String = BATCH_BEGIN.to_owned();
    for item in items {
        payload.push_str(CHANGESET_BEGIN);
        payload.push_str(if item.2.is_some() { "PUT" } else { "DELETE" });
        payload.push_str(" ");
        payload.push_str(if use_https { "https" } else { "http" });
        payload.push_str("://");
        payload.push_str(account);
        payload.push_str(TABLE_SUFFIX);
        payload.push_str("/");
        payload.push_str(table);
        payload.push_str("(PartitionKey='");
        payload.push_str(item.0.as_str());
        payload.push_str("',RowKey='");
        payload.push_str(item.1.as_str());
        payload.push_str("') HTTP/1.1\n");
        if let Some(ref v) = item.2 {
            payload.push_str(UPDATE_HEADER);
            payload.push_str(json::encode(v).unwrap().as_str());
        }
        payload.push_str("\n");
    }
    payload + CHANGESET_END + BATCH_END
}

#[cfg(test)]
mod test {
    use super::*;

    #[allow(non_snake_case)]
    #[derive(RustcEncodable)]
    struct Entity {
        PartitionKey: String,
        RowKey: String,
        Rating: i32,
        Text: String,
    }

    #[test]
    fn verify_batch_payload() {
        let expected = r#"--batch_a1e9d677-b28b-435e-a89e-87e6a768a431
Content-Type: multipart/mixed; boundary=changeset_8a28b620-b4bb-458c-a177-0959fb14c977

--changeset_8a28b620-b4bb-458c-a177-0959fb14c977
Content-Type: application/http
Content-Transfer-Encoding: binary

PUT https://myaccount.table.core.windows.net/Blogs(PartitionKey='Channel_17',RowKey='3') HTTP/1.1
Content-Type: application/json

{"PartitionKey":"Channel_17","RowKey":"3","Rating":9,"Text":".NET..."}
--changeset_8a28b620-b4bb-458c-a177-0959fb14c977
Content-Type: application/http
Content-Transfer-Encoding: binary

PUT https://myaccount.table.core.windows.net/Blogs(PartitionKey='Channel_17',RowKey='3') HTTP/1.1
Content-Type: application/json

{"PartitionKey":"Channel_17","RowKey":"3","Rating":9,"Text":"PDC 2008..."}
--changeset_8a28b620-b4bb-458c-a177-0959fb14c977
Content-Type: application/http
Content-Transfer-Encoding: binary

DELETE https://myaccount.table.core.windows.net/Blogs(PartitionKey='Channel_17',RowKey='3') HTTP/1.1

--changeset_8a28b620-b4bb-458c-a177-0959fb14c977--
--batch_a1e9d677-b28b-435e-a89e-87e6a768a431
"#;

        let items = vec![bupdate("Channel_17", "3", 9, ".NET..."),
                         bupdate("Channel_17", "3", 9, "PDC 2008..."),
                         bdelete("Channel_17", "3")];
        let actual = generate_batch_payload("myaccount", "Blogs", items.as_slice(), true);
        // println!("{}\n{}\n", expected, actual);
        assert_eq!(expected, actual);
    }

    fn bupdate(pk: &str, rk: &str, rating: i32, text: &str) -> BatchItem<Entity> {
        BatchItem(pk.to_owned(),
                  rk.to_owned(),
                  Some(Entity {
                      PartitionKey: pk.to_owned(),
                      RowKey: rk.to_owned(),
                      Rating: rating,
                      Text: text.to_owned(),
                  }))
    }

    fn bdelete(pk: &str, rk: &str) -> BatchItem<Entity> {
        BatchItem(pk.to_owned(), rk.to_owned(), None)
    }
}