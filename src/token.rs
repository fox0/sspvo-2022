use serde::{Deserialize, Serialize, Serializer};
use serde::ser::SerializeStruct;

use crate::models::{NoPayload, PackageData};


// #[derive(Serialize, Deserialize)]
// pub enum Payload {
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

#[derive(Debug, PartialEq, Serialize, Deserialize)]
enum Action {
    /// Проверка действительности регистрации сертификата к организации
    CheckCertificate,
    /// Запрос на получение сообщений из очереди
    GetMessage,
    MessageConfirm,
    Add,
    // Edit,
    Remove,
    Get,
}

impl Action {
    fn is_check_certificate(&self) -> bool {
        self == &Action::CheckCertificate
    }
}

#[derive(Debug, PartialEq, Deserialize)]
enum PayloadType {
    Success,
    Error,
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "PascalCase")]
struct Header {
    #[serde(skip_deserializing)]
    ogrn: String,
    #[serde(skip_deserializing)]
    kpp: String,
    #[serde(skip_serializing_if = "Action::is_check_certificate")]
    action: Action,
    #[serde(skip_serializing_if = "Option::is_none")]
    entity_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id_jwt: Option<u64>,
    #[serde(skip_serializing)]
    payload_type: PayloadType,
}

pub struct Token<T> where T: Serialize + PackageData {
    header: Header,
    payload: T,
}

impl<T> Token<T> where T: Serialize + PackageData {
    pub fn new_add(ogrn: impl Into<String>, kpp: impl Into<String>, payload: T) -> Self {
        Self::new(Action::Add, ogrn.into(), kpp.into(), payload)
    }

    // pub fn new_edit(ogrn: impl Into<String>, kpp: impl Into<String>, payload: T) -> Self {
    //     Self::new(Action::Edit, ogrn.into(), kpp.into(), payload)
    // }

    pub fn new_remove(ogrn: impl Into<String>, kpp: impl Into<String>, payload: T) -> Self {
        Self::new(Action::Remove, ogrn.into(), kpp.into(), payload)
    }

    pub fn new_get(ogrn: impl Into<String>, kpp: impl Into<String>, payload: T) -> Self {
        Self::new(Action::Get, ogrn.into(), kpp.into(), payload)
    }

    fn new(action: Action, ogrn: String, kpp: String, payload: T) -> Self {
        let entity_type = Some(payload.get_name().into());
        Self {
            header: Header { action, ogrn, kpp, entity_type, id_jwt: None },
            payload,
        }
    }

    // todo
    fn sign(body: &str) -> Vec<u8> {
        vec![1, 2, 3]
    }
}

impl Token<NoPayload> {
    /// Проверка действительности регистрации сертификата к организации
    pub fn new_check_certificate(ogrn: impl Into<String>, kpp: impl Into<String>) -> Self {
        Self::new_nopayload(Action::CheckCertificate, ogrn.into(), kpp.into(), None)
    }

    /// Запрос на получение сообщений из очереди
    pub fn new_get_message(ogrn: impl Into<String>, kpp: impl Into<String>, id_jwt: Option<u64>) -> Self {
        let id_jwt = id_jwt.unwrap_or(0u64);
        Self::new_nopayload(Action::GetMessage, ogrn.into(), kpp.into(), Some(id_jwt))
    }

    pub fn new_message_confirm(ogrn: impl Into<String>, kpp: impl Into<String>, id_jwt: u64) -> Self {
        Self::new_nopayload(Action::MessageConfirm, ogrn.into(), kpp.into(), Some(id_jwt))
    }

    fn new_nopayload(action: Action, ogrn: String, kpp: String, id_jwt: Option<u64>) -> Self {
        Self {
            header: Header { action, ogrn, kpp, entity_type: None, id_jwt },
            payload: NoPayload,
        }
    }
}

impl<T> Serialize for Token<T> where T: Serialize + PackageData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let header = serde_json::to_string(&self.header).unwrap();
        let payload = match self.header.action {
            Action::CheckCertificate | Action::GetMessage | Action::MessageConfirm => "".to_string(),
            Action::Add /*| Action::Edit*/ => self.payload.get_xml_add(),
            Action::Remove | Action::Get => self.payload.get_xml_remove(),
        };
        if cfg!(debug_assertions) {
            println!("{}", &header);
            println!("{}", &payload);
        }
        let body = format!("{}.{}", base64::encode(header), base64::encode(payload));
        let signature = Self::sign(&body);
        let token = format!("{}.{}", body, base64::encode(signature));

        let mut s = serializer.serialize_struct("Token", 1)?;
        s.serialize_field("Token", &token)?;
        s.end()
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
enum ResponseToken {
    ResponseToken(Token<>)
}
//


#[cfg(test)]
mod tests {
    use crate::models::{PackageData, OrgDirection};
    use crate::token::{Token, Action, PayloadType, ResponseHeader};

    #[test]
    fn token_check_certificate() {
        let token = Token::new_check_certificate("123", "456");
        let json = serde_json::to_string(&token.header).unwrap();
        assert_eq!(json, r#"{"Ogrn":"123","Kpp":"456"}"#);
        let json = serde_json::to_string(&token).unwrap();
        assert_eq!(json, r#"{"Token":"eyJPZ3JuIjoiMTIzIiwiS3BwIjoiNDU2In0=..AQID"}"#);
    }

    #[test]
    fn token_new_get_message() {
        let token = Token::new_get_message("123", "456", None);
        let json = serde_json::to_string(&token.header).unwrap();
        assert_eq!(json, r#"{"Action":"GetMessage","Ogrn":"123","Kpp":"456","IdJwt":0}"#);
        // let json = serde_json::to_string(&token).unwrap();
        // assert_eq!(json, r#"{"Token":""}"#);
    }

    #[test]
    fn token_new_message_confirm() {
        let token = Token::new_message_confirm("123", "456", 42);
        let json = serde_json::to_string(&token.header).unwrap();
        assert_eq!(json, r#"{"Action":"MessageConfirm","Ogrn":"123","Kpp":"456","IdJwt":42}"#);
        // let json = serde_json::to_string(&token).unwrap();
        // assert_eq!(json, r#"{"Token":""}"#);
    }

    #[test]
    fn token_new_add() {
        let payload = OrgDirection { uid: "123".to_string(), id_direction: 42 };
        let token = Token::new_add("123", "456", payload);
        let json = serde_json::to_string(&token.header).unwrap();
        assert_eq!(json, r#"{"Action":"Add","Ogrn":"123","Kpp":"456","EntityType":"OrgDirection"}"#);
        let xml = token.payload.get_xml_add();
        assert_eq!(xml, r#"<?xml version="1.0" encoding="utf-8"?><PackageData><OrgDirection><Uid>123</Uid><IdDirection>42</IdDirection></OrgDirection></PackageData>"#);
        let json = serde_json::to_string(&token).unwrap();
        assert_eq!(json, r#"{"Token":"eyJBY3Rpb24iOiJBZGQiLCJPZ3JuIjoiMTIzIiwiS3BwIjoiNDU2IiwiRW50aXR5VHlwZSI6Ik9yZ0RpcmVjdGlvbiJ9.PD94bWwgdmVyc2lvbj0iMS4wIiBlbmNvZGluZz0idXRmLTgiPz48UGFja2FnZURhdGE+PE9yZ0RpcmVjdGlvbj48VWlkPjEyMzwvVWlkPjxJZERpcmVjdGlvbj40MjwvSWREaXJlY3Rpb24+PC9PcmdEaXJlY3Rpb24+PC9QYWNrYWdlRGF0YT4=.AQID"}"#);
    }

    #[test]
    fn token_new_remove() {
        let payload = OrgDirection { uid: "123".to_string(), id_direction: 42 };
        let token = Token::new_remove("123", "456", payload);
        let json = serde_json::to_string(&token.header).unwrap();
        assert_eq!(json, r#"{"Action":"Remove","Ogrn":"123","Kpp":"456","EntityType":"OrgDirection"}"#);
        let xml = token.payload.get_xml_remove();
        assert_eq!(xml, r#"<?xml version="1.0" encoding="utf-8"?><PackageData><OrgDirection><Uid>123</Uid></OrgDirection></PackageData>"#);
        // let json = serde_json::to_string(&token).unwrap();
        // assert_eq!(json, r#"{"Token":""}"#);
    }

    #[test]
    fn token_new_get() {
        let payload = OrgDirection { uid: "123".to_string(), id_direction: 42 };
        let token = Token::new_get("123", "456", payload);
        let json = serde_json::to_string(&token.header).unwrap();
        assert_eq!(json, r#"{"Action":"Get","Ogrn":"123","Kpp":"456","EntityType":"OrgDirection"}"#);
        let xml = token.payload.get_xml_remove();
        assert_eq!(xml, r#"<?xml version="1.0" encoding="utf-8"?><PackageData><OrgDirection><Uid>123</Uid></OrgDirection></PackageData>"#);
        // let json = serde_json::to_string(&token).unwrap();
        // assert_eq!(json, r#"{"Token":""}"#);
    }

    #[test]
    fn response_header_from_json() {
        let json = r#"
{
  "IdJwt": 42,
  "PayloadType": "Success",
  "EntityType": "OrgDirection",
  "Action": "Add"
}
"#;
        let header: ResponseHeader = serde_json::from_str(json).unwrap();
        assert_eq!(header.id_jwt, 42);
        assert_eq!(header.entity_type, "OrgDirection".to_string());
        assert_eq!(header.action, Action::Add);
        assert_eq!(header.payload_type, PayloadType::Success);
    }
}
