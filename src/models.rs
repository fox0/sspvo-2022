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


// #[derive(Debug, PartialEq, Serialize, Deserialize)]
// pub enum Action {
//     Add,
//     Remove,
//     Edit,
//     GetMessage,
//     MessageConfirm,
//     Get,
// }

#[derive(Serialize)]
#[serde(tag = "Action")]
pub enum Header {
    // воспользуемся особенностью, что лишние поля в json игнорируются
    // #[serde(skip_deserializing)]
    #[serde(rename_all = "PascalCase")]
    CheckCertificate { ogrn: String, kpp: String },

    // #[serde(skip_deserializing)]
    #[serde(rename_all = "PascalCase")]
    GetMessage { ogrn: String, kpp: String, id_jwt: u64 },

    // #[serde(skip_deserializing)]
    #[serde(rename_all = "PascalCase")]
    MessageConfirm { ogrn: String, kpp: String, id_jwt: u64 },

    // #[serde(skip_deserializing)]
    #[serde(rename_all = "PascalCase")]
    Add { ogrn: String, kpp: String, entity_type: String },

    //Response { id_jwt: u64, entity_type: String, action: Action, payload_type: PayloadType },
}

#[derive(Debug, PartialEq, Deserialize)]
pub enum PayloadType {
    Success,
    Error,
}

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
    pub fn new_check_certificate(ogrn: String, kpp: String) -> Self {
        Self {
            header: Header::CheckCertificate { ogrn, kpp },
            payload: None,
        }
    }

    /// Запрос на получение сообщений из очереди
    pub fn new_get_message(ogrn: String, kpp: String, id_jwt: Option<u64>) -> Self {
        let id_jwt = id_jwt.unwrap_or(0u64);
        Self {
            header: Header::GetMessage { ogrn, kpp, id_jwt },
            payload: None,
        }
    }

    pub fn new_message_confirm(ogrn: String, kpp: String, id_jwt: u64) -> Self {
        Self {
            header: Header::MessageConfirm { ogrn, kpp, id_jwt },
            payload: None,
        }
    }

    pub fn new_add(ogrn: String, kpp: String, payload: T) -> Self {
        let entity_type = Self::get_entity_type(&payload);
        Self {
            header: Header::Add { ogrn, kpp, entity_type },
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
    use crate::models::{NoPayload, OrgDirection, Token};

    #[test]
    fn org_direction_to_xml() {
        let payload = OrgDirection { uid: "123".to_string(), id_direction: 42 };
        let xml = serde_xml_rs::to_string(&payload).unwrap();
        assert_eq!(xml, "<OrgDirection><Uid>123</Uid><IdDirection>42</IdDirection></OrgDirection>");
    }

    #[test]
    fn token_new_check_certificate() {
        let token: Token<NoPayload> = Token::new_check_certificate("123".to_string(), "456".to_string());
        let json = serde_json::to_string(&token.header).unwrap();
        // assert_eq!(json, r#"{"Ogrn":"123","Kpp":"456"}"#);
        assert_eq!(json, r#"{"Action":"CheckCertificate","Ogrn":"123","Kpp":"456"}"#);
        let json = serde_json::to_string(&token).unwrap();
        assert_eq!(json, r#"{"Token":"eyJBY3Rpb24iOiJDaGVja0NlcnRpZmljYXRlIiwiT2dybiI6IjEyMyIsIktwcCI6IjQ1NiJ9..AQID"}"#);
    }

    #[test]
    fn token_new_get_message() {
        let token: Token<NoPayload> = Token::new_get_message("123".to_string(), "456".to_string(), None);
        let json = serde_json::to_string(&token.header).unwrap();
        assert_eq!(json, r#"{"Action":"GetMessage","Ogrn":"123","Kpp":"456","IdJwt":0}"#);
    }

    #[test]
    fn token_new_message_confirm() {
        let token: Token<NoPayload> = Token::new_message_confirm("123".to_string(), "456".to_string(), 42);
        let json = serde_json::to_string(&token.header).unwrap();
        assert_eq!(json, r#"{"Action":"MessageConfirm","Ogrn":"123","Kpp":"456","IdJwt":42}"#);
    }

    #[test]
    fn token_new_add() {
        let payload = OrgDirection { uid: "123".to_string(), id_direction: 42 };
        let token = Token::new_add("123".to_string(), "456".to_string(), payload);
        let json = serde_json::to_string(&token.header).unwrap();
        assert_eq!(json, r#"{"Action":"Add","Ogrn":"123","Kpp":"456","EntityType":"OrgDirection"}"#);
        let xml = serde_xml_rs::to_string(&token.payload).unwrap();
        assert_eq!(xml, r"<OrgDirection><Uid>123</Uid><IdDirection>42</IdDirection></OrgDirection>");
        let json = serde_json::to_string(&token).unwrap();
        assert_eq!(json, r#"{"Token":"eyJBY3Rpb24iOiJBZGQiLCJPZ3JuIjoiMTIzIiwiS3BwIjoiNDU2IiwiRW50aXR5VHlwZSI6Ik9yZ0RpcmVjdGlvbiJ9.PFBhY2thZ2VEYXRhPjxPcmdEaXJlY3Rpb24+PFVpZD4xMjM8L1VpZD48SWREaXJlY3Rpb24+NDI8L0lkRGlyZWN0aW9uPjwvT3JnRGlyZWN0aW9uPjwvUGFja2FnZURhdGE+.AQID"}"#);
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
