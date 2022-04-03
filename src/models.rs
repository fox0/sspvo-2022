use serde::{Deserialize, Serialize, Serializer};
use serde::ser::SerializeStruct;


#[derive(Serialize)]
pub struct NoPayload;

#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct OrgDirection {
    uid: String,
    id_direction: u64,
}

// #[derive(Serialize, Deserialize)]
// pub enum EntityType {
//     OrgDirection,
//
//     CampaignAchievement,
//     AdmissionVolume,
//     ServiceApplication,
//     Campaign,
//     CompetitiveGroup,
//     CompetitiveGroupApplicationsList,
//     TargetContract,
//     DirectionParamsValue,
//     DistributedAdmissionVolume,
//     Document,
//     EditApplicationStatusList,
//     EntranceTest,
//     EntranceTestAgreedList,
//     EntranceTestLocation,
//     EntranceTestSheet,
//     ServiceEntrant,
//     ServiceEntrantPhotoFile,
//     EntranceTestResultList,
//     Identification,
//     OrderAdmissionPackage,
//     ServiceApplicationAchievement,
//     ServiceApplicationBenefit,
//     ServiceApplicationNoticeList,
//     SendRefreshedEntranceTestDataToEpguList,
//     TargetOrganization,
//     TermsAdmission,
//     RestartDocumentCheckList,
// }


// #[allow(dead_code)]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Action {
    Add,
    Remove,
    Edit,
    GetMessage,
    MessageConfirm,
    Get,
}

#[derive(Debug, PartialEq, Deserialize)]
pub enum PayloadType {
    Success,
    Error,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Header {
    #[serde(skip_deserializing)]
    ogrn: String,
    #[serde(skip_deserializing)]
    kpp: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    entity_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    action: Option<Action>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id_jwt: Option<u64>,
    #[serde(skip_serializing)]
    payload_type: Option<PayloadType>,
}


pub struct Token<T>
    where T: Serialize {
    header: Header,
    payload: Option<T>,
}

impl<T> Token<T>
    where T: Serialize {
    /// Проверка действительности регистрации сертификата к организации
    ///
    /// ```
    /// let token: Token<NoPayload> = Token::new_check_certificate(ogrn, kpp);
    /// ```
    pub fn new_check_certificate(ogrn: String, kpp: String) -> Self {
        let header = Header {
            ogrn,
            kpp,
            entity_type: None,
            action: None,
            id_jwt: None,
            payload_type: None,
        };
        Self { header, payload: None }
    }

    pub fn new_add(ogrn: String, kpp: String, payload: T) -> Self {
        Self::new_add_or_edit(ogrn, kpp, payload, Action::Add)
    }

    pub fn new_edit(ogrn: String, kpp: String, payload: T) -> Self {
        Self::new_add_or_edit(ogrn, kpp, payload, Action::Edit)
    }

    fn new_add_or_edit(ogrn: String, kpp: String, payload: T, action: Action) -> Self {
        let header = Header {
            ogrn,
            kpp,
            entity_type: Some(Self::get_entity_type(&payload)),
            action: Some(action),
            id_jwt: None,
            payload_type: None,
        };
        Self { header, payload: Some(payload) }
    }

    /// Запрос на получение сообщений из очереди
    pub fn new_get_message(ogrn: String, kpp: String, id_jwt: Option<u64>) -> Self {
        let header = Header {
            ogrn,
            kpp,
            entity_type: None,
            action: Some(Action::GetMessage),
            id_jwt: Some(id_jwt.unwrap_or(0u64)),
            payload_type: None,
        };
        Self { header, payload: None }
    }

    pub fn new_message_confirm(ogrn: String, kpp: String, id_jwt: u64) -> Self {
        let header = Header {
            ogrn,
            kpp,
            entity_type: None,
            action: Some(Action::MessageConfirm),
            id_jwt: Some(id_jwt),
            payload_type: None,
        };
        Self { header, payload: None }
    }

    fn get_entity_type(payload: &T) -> String {
        serde_type_name::type_name(payload).unwrap().to_string()
    }

    // todo
    fn sign(body: &str) -> Vec<u8> {
        vec![1, 2, 3]
    }
}

impl<T> Serialize for Token<T>
    where T: Serialize {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer {
        let header = serde_json::to_string(&self.header).unwrap();
        let header = base64::encode(header);

        let payload = match &self.payload {
            Some(payload) => {
                let payload = serde_xml_rs::to_string(payload).unwrap();
                let payload = format!("<PackageData>{}</PackageData>", payload);
                base64::encode(payload)
            }
            None => "".to_string()
        };

        let body = format!("{}.{}", header, payload);
        let signature = Self::sign(&body);
        let signature = base64::encode(signature);
        let token = format!("{}.{}", body, signature);

        // the number of fields in the struct.
        let mut s = serializer.serialize_struct("Token", 1)?;
        s.serialize_field("Token", &token)?;
        s.end()
    }
}


#[cfg(test)]
mod tests {
    use crate::models::{NoPayload, Header, OrgDirection, Token, Action, PayloadType};

    #[test]
    fn org_direction_to_xml() {
        let payload = OrgDirection { uid: "123".to_string(), id_direction: 42 };
        let xml = serde_xml_rs::to_string(&payload).unwrap();
        assert_eq!(xml, "<OrgDirection><Uid>123</Uid><IdDirection>42</IdDirection></OrgDirection>");
    }

    #[test]
    fn header_to_json() {
        let header = Header {
            ogrn: "123".to_string(),
            kpp: "456".to_string(),
            entity_type: None,
            action: None,
            id_jwt: None,
            payload_type: None,
        };
        let json = serde_json::to_string(&header).unwrap();
        assert_eq!(json, r#"{"Ogrn":"123","Kpp":"456"}"#);
    }

    #[test]
    fn header_from_json() {
        let json = r#"
{
  "IdJwt": 42,
  "PayloadType": "Success",
  "EntityType": "OrgDirection",
  "Action": "Edit"
}
"#;
        let header: Header = serde_json::from_str(json).unwrap();
        assert_eq!(header.id_jwt, Some(42));
        assert_eq!(header.entity_type, Some("OrgDirection".to_string()));
        assert_eq!(header.action, Some(Action::Edit));
        assert_eq!(header.payload_type, Some(PayloadType::Success));
    }

    #[test]
    fn token_new_check_certificate() {
        let token: Token<NoPayload> = Token::new_check_certificate("123".to_string(), "456".to_string());
        let json = serde_json::to_string(&token.header).unwrap();
        assert_eq!(json, r#"{"Ogrn":"123","Kpp":"456"}"#);
        let json = serde_json::to_string(&token).unwrap();
        assert_eq!(json, r#"{"Token":"eyJPZ3JuIjoiMTIzIiwiS3BwIjoiNDU2In0=..AQID"}"#);
    }

    #[test]
    fn token_new_add() {
        let payload = OrgDirection { uid: "123".to_string(), id_direction: 42 };
        let token = Token::new_add("123".to_string(), "456".to_string(), payload);
        let json = serde_json::to_string(&token.header).unwrap();
        assert_eq!(json, r#"{"Ogrn":"123","Kpp":"456","EntityType":"OrgDirection","Action":"Add"}"#);
        let xml = serde_xml_rs::to_string(&token.payload).unwrap();
        assert_eq!(xml, r"<OrgDirection><Uid>123</Uid><IdDirection>42</IdDirection></OrgDirection>");
        let json = serde_json::to_string(&token).unwrap();
        assert_eq!(json, r#"{"Token":"eyJPZ3JuIjoiMTIzIiwiS3BwIjoiNDU2IiwiRW50aXR5VHlwZSI6Ik9yZ0RpcmVjdGlvbiIsIkFjdGlvbiI6IkFkZCJ9.PFBhY2thZ2VEYXRhPjxPcmdEaXJlY3Rpb24+PFVpZD4xMjM8L1VpZD48SWREaXJlY3Rpb24+NDI8L0lkRGlyZWN0aW9uPjwvT3JnRGlyZWN0aW9uPjwvUGFja2FnZURhdGE+.AQID"}"#);
    }

    #[test]
    fn token_new_get_message() {
        let token: Token<NoPayload> = Token::new_get_message("123".to_string(), "456".to_string(), None);
        let json = serde_json::to_string(&token.header).unwrap();
        assert_eq!(json, r#"{"Ogrn":"123","Kpp":"456","Action":"GetMessage","IdJwt":0}"#);
    }

    #[test]
    fn token_new_message_confirm() {
        let token: Token<NoPayload> = Token::new_message_confirm("123".to_string(), "456".to_string(), 42);
        let json = serde_json::to_string(&token.header).unwrap();
        assert_eq!(json, r#"{"Ogrn":"123","Kpp":"456","Action":"MessageConfirm","IdJwt":42}"#);
    }
}
