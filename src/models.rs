use serde::Serialize;
use ss_derive::PackageData;


/// ```
/// #[derive(Serialize, PackageData)]
/// #[serde(rename_all = "PascalCase")]
/// pub struct OrgDirection {
///     pub uid: String,
///     //...
/// }
/// ```
pub trait PackageData: Serialize {
    fn get_name(&self) -> &str;
    fn get_uid(&self) -> &String;

    fn get_xml_add(&self) -> String {
        let xml = serde_xml_rs::to_string(&self).unwrap();
        format!(r#"<?xml version="1.0" encoding="utf-8"?><PackageData>{}</PackageData>"#, xml)
    }

    fn get_xml_remove(&self) -> String {
        let name = self.get_name();
        let uid = self.get_uid();
        format!(r#"<?xml version="1.0" encoding="utf-8"?><PackageData><{}><Uid>{}</Uid></{}></PackageData>"#, name, uid, name)
    }
}

#[derive(Serialize)]
pub struct NoPayload;

impl PackageData for NoPayload {
    fn get_name(&self) -> &str {
        unreachable!()
    }
    fn get_uid(&self) -> &String {
        unreachable!()
    }
}

/// Направления подготовки ООВО
#[derive(Serialize, PackageData)]
#[serde(rename_all = "PascalCase")]
pub struct OrgDirection {
    pub uid: String,
    pub id_direction: u64,
}


#[cfg(test)]
mod tests {
    use crate::models::{OrgDirection, PackageData};

    #[test]
    fn org_direction_add() {
        let payload = OrgDirection { uid: "123".to_string(), id_direction: 42 };
        let xml = payload.get_xml_add();
        assert_eq!(xml, r#"<?xml version="1.0" encoding="utf-8"?><PackageData><OrgDirection><Uid>123</Uid><IdDirection>42</IdDirection></OrgDirection></PackageData>"#);
    }

    #[test]
    fn org_direction_remove() {
        let payload = OrgDirection { uid: "123".to_string(), id_direction: 42 };
        let xml = payload.get_xml_remove();
        assert_eq!(xml, r#"<?xml version="1.0" encoding="utf-8"?><PackageData><OrgDirection><Uid>123</Uid></OrgDirection></PackageData>"#);
    }
}
