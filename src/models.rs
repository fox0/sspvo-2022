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

#[derive(PartialEq, Serialize)]
pub enum Action {
    /// Проверка действительности регистрации сертификата к организации
    CheckCertificate,
    /// Запрос на получение сообщений из очереди
    GetMessage,
    MessageConfirm,
    Add,
    Edit,
    Remove,
    Get,
}

#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
struct Header {
    #[serde(skip_serializing_if = "header_action_skip_serializing")]
    action: Action,
    ogrn: String,
    kpp: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    entity_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id_jwt: Option<u64>,
}

fn header_action_skip_serializing(action: &Action) -> bool {
    action == &Action::CheckCertificate
}

// #[derive(Debug, PartialEq, Deserialize)]
// pub enum PayloadType {
//     Success,
//     Error,
// }

// ResponseHeader { id_jwt: u64, entity_type: String, action: Action, payload_type: PayloadType },

pub struct Token<T> where T: Serialize {
    header: Header,
    payload: Option<T>,
}

impl<T> Token<T> where T: Serialize {
    /// Проверка действительности регистрации сертификата к организации
    ///
    /// ```
    /// let token: Token<NoPayload> = Token::new_check_certificate(ogrn, kpp);
    /// ```
    pub fn new_check_certificate(ogrn: impl Into<String>, kpp: impl Into<String>) -> Self {
        Self {
            header: Header {
                action: Action::CheckCertificate,
                ogrn: ogrn.into(),
                kpp: kpp.into(),
                entity_type: None,
                id_jwt: None,
            },
            payload: None,
        }
    }

    /// Запрос на получение сообщений из очереди
    pub fn new_get_message(ogrn: impl Into<String>, kpp: impl Into<String>, id_jwt: Option<u64>) -> Self {
        let id_jwt = id_jwt.unwrap_or(0u64);
        Self {
            header: Header {
                action: Action::GetMessage,
                ogrn: ogrn.into(),
                kpp: kpp.into(),
                entity_type: None,
                id_jwt: Some(id_jwt),
            },
            payload: None,
        }
    }

    pub fn new_message_confirm(ogrn: impl Into<String>, kpp: impl Into<String>, id_jwt: u64) -> Self {
        Self {
            header: Header {
                action: Action::MessageConfirm,
                ogrn: ogrn.into(),
                kpp: kpp.into(),
                entity_type: None,
                id_jwt: Some(id_jwt),
            },
            payload: None,
        }
    }

    pub fn new_add(ogrn: impl Into<String>, kpp: impl Into<String>, payload: T) -> Self {
        let entity_type = Self::get_entity_type(&payload);
        Self {
            header: Header {
                action: Action::Add,
                ogrn: ogrn.into(),
                kpp: kpp.into(),
                entity_type: Some(entity_type),
                id_jwt: None,
            },
            payload: Some(payload),
        }
    }

    fn get_entity_type(payload: &T) -> String {
        serde_type_name::type_name(payload).unwrap().to_string()
    }

    // todo
    fn sign(body: &str) -> Vec<u8> {
        vec![1, 2, 3]
    }
}

impl<T> Serialize for Token<T> where T: Serialize {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let header = serde_json::to_string(&self.header).unwrap();

        //todo action in add edit
        let payload = match &self.payload {
            Some(payload) => {
                let payload = serde_xml_rs::to_string(&payload).unwrap();
                format!(r#"r#"<?xml version="1.0" encoding="utf-8"?><PackageData>{}</PackageData>"#, payload)
            }
            None => "".to_string()
        };

        let body = format!("{}.{}", base64::encode(header), base64::encode(payload));
        let signature = Self::sign(&body);
        let token = format!("{}.{}", body, base64::encode(signature));

        // the number of fields in the struct.
        let mut s = serializer.serialize_struct("Token", 1)?;
        s.serialize_field("Token", &token)?;
        s.end()
    }
}

// ResponseToken


#[cfg(test)]
mod tests {
    use crate::models::{Action, Header, NoPayload, OrgDirection, Token};

    #[test]
    fn org_direction_to_xml() {
        let payload = OrgDirection { uid: "123".to_string(), id_direction: 42 };
        let xml = serde_xml_rs::to_string(&payload).unwrap();
        assert_eq!(xml, "<OrgDirection><Uid>123</Uid><IdDirection>42</IdDirection></OrgDirection>");
    }

    #[test]
    fn header_check_certificate() {
        let header = Header {
            ogrn: "123".to_string(),
            kpp: "456".to_string(),
            action: Action::CheckCertificate,
            entity_type: None,
            id_jwt: None,
        };
        let json = serde_json::to_string(&header).unwrap();
        assert_eq!(json, r#"{"Ogrn":"123","Kpp":"456"}"#);
    }

    #[test]
    fn token_check_certificate() {
        let token: Token<NoPayload> = Token::new_check_certificate("123", "456");
        let json = serde_json::to_string(&token).unwrap();
        assert_eq!(json, r#"{"Token":"eyJPZ3JuIjoiMTIzIiwiS3BwIjoiNDU2In0=..AQID"}"#);
    }

    #[test]
    fn token_new_get_message() {
        let token: Token<NoPayload> = Token::new_get_message("123", "456", None);
        let json = serde_json::to_string(&token.header).unwrap();
        assert_eq!(json, r#"{"Action":"GetMessage","Ogrn":"123","Kpp":"456","IdJwt":0}"#);
    }

    #[test]
    fn token_new_message_confirm() {
        let token: Token<NoPayload> = Token::new_message_confirm("123", "456", 42);
        let json = serde_json::to_string(&token.header).unwrap();
        assert_eq!(json, r#"{"Action":"MessageConfirm","Ogrn":"123","Kpp":"456","IdJwt":42}"#);
    }

    #[test]
    fn token_new_add() {
        let payload = OrgDirection { uid: "123".to_string(), id_direction: 42 };
        let token = Token::new_add("123", "456", payload);
        let json = serde_json::to_string(&token.header).unwrap();
        assert_eq!(json, r#"{"Action":"Add","Ogrn":"123","Kpp":"456","EntityType":"OrgDirection"}"#);
        let xml = serde_xml_rs::to_string(&token.payload).unwrap();
        assert_eq!(xml, r"<OrgDirection><Uid>123</Uid><IdDirection>42</IdDirection></OrgDirection>");
        let json = serde_json::to_string(&token).unwrap();
        assert_eq!(json, r#"{"Token":"eyJBY3Rpb24iOiJBZGQiLCJPZ3JuIjoiMTIzIiwiS3BwIjoiNDU2IiwiRW50aXR5VHlwZSI6Ik9yZ0RpcmVjdGlvbiJ9.ciMiPD94bWwgdmVyc2lvbj0iMS4wIiBlbmNvZGluZz0idXRmLTgiPz48UGFja2FnZURhdGE+PE9yZ0RpcmVjdGlvbj48VWlkPjEyMzwvVWlkPjxJZERpcmVjdGlvbj40MjwvSWREaXJlY3Rpb24+PC9PcmdEaXJlY3Rpb24+PC9QYWNrYWdlRGF0YT4=.AQID"}"#);
    }

//     #[test]
//     fn header_from_json() {
//         let json = r#"
// {
//   "IdJwt": 42,
//   "PayloadType": "Success",
//   "EntityType": "OrgDirection",
//   "Action": "Edit"
// }
// "#;
//         let header: Header = serde_json::from_str(json).unwrap();
//         assert_eq!(header.id_jwt, Some(42));
//         assert_eq!(header.entity_type, Some("OrgDirection".to_string()));
//         assert_eq!(header.action, Some(Action::Edit));
//         assert_eq!(header.payload_type, Some(PayloadType::Success));
//     }
}
