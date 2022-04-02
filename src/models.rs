// use std::fmt::Debug;

use serde::{Deserialize, Serialize, Serializer};
use serde::ser::SerializeStruct;


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


#[derive(Serialize)]
pub enum Action {
    Add,
    Remove,
    Edit,
    GetMessage,
    MessageConfirm,
    Get,
}

#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Header {
    ogrn: String,
    kpp: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    entity_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    action: Option<Action>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id_jwt: Option<u64>,
}


pub struct Token<T>
    where T: Serialize {
    header: Header,
    payload: Option<T>,
}

impl<T> Token<T>
    where T: Serialize {
    // /// Проверка действительности регистрации сертификата к организации
    // pub fn new_check_certificate(ogrn: String, kpp: String) -> Self {
    //     let header = Header {
    //         ogrn,
    //         kpp,
    //         entity_type: None,
    //         action: None,
    //         id_jwt: None,
    //     };
    //     Self { header, payload: None }
    // }

    pub fn new_add(ogrn: String, kpp: String, payload: T) -> Self {
        let entity_type = unsafe { std::intrinsics::type_name::<T>() }.to_string();
        // "spvo_rs::models::OrgDirection"
        let entity_type = entity_type.rsplit_once("::").unwrap().1.to_string();
        let header = Header {
            ogrn,
            kpp,
            entity_type: Some(entity_type),
            action: Some(Action::Add),
            id_jwt: None,
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
            id_jwt: Some(id_jwt.unwrap_or(0)),
        };
        Self { header, payload: None }
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
    use crate::models::{Header, OrgDirection, Token};

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
        };
        let json = serde_json::to_string(&header).unwrap();
        assert_eq!(json, r#"{"Ogrn":"123","Kpp":"456"}"#);
    }

    #[test]
    fn token_new_add() {
        let payload = OrgDirection { uid: "123".to_string(), id_direction: 42 };
        let token = Token::new_add("123".to_string(), "456".to_string(), payload);
        assert_eq!(token.header.entity_type, Some("OrgDirection".to_string()));
    }

    #[test]
    fn token_new_add_to_json() {
        let payload = OrgDirection { uid: "123".to_string(), id_direction: 42 };
        let token = Token::new_add("123".to_string(), "456".to_string(), payload);
        let json = serde_json::to_string(&token).unwrap();
        assert_eq!(json, r#"{"Token":"eyJPZ3JuIjoiMTIzIiwiS3BwIjoiNDU2IiwiRW50aXR5VHlwZSI6Ik9yZ0RpcmVjdGlvbiIsIkFjdGlvbiI6IkFkZCJ9.PFBhY2thZ2VEYXRhPjxPcmdEaXJlY3Rpb24+PFVpZD4xMjM8L1VpZD48SWREaXJlY3Rpb24+NDI8L0lkRGlyZWN0aW9uPjwvT3JnRGlyZWN0aW9uPjwvUGFja2FnZURhdGE+.AQID"}"#);
    }
}
