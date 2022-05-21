#![allow(non_snake_case)]

use yaserde_derive::{YaSerialize, YaDeserialize};
use validator::Validate;


#[allow(dead_code)]
pub enum Payload {
    OrgDirection,
    TargetOrganization,
}


#[derive(Debug, PartialEq, YaSerialize, YaDeserialize, Validate)]
#[yaserde(rename = "PackageData")]
pub struct XmlOrgDirection {
    pub OrgDirection: OrgDirection,
}

#[derive(Debug, PartialEq, YaSerialize, YaDeserialize, Validate)]
#[yaserde(rename = "PackageData")]
pub struct XmlTargetOrganization {
    pub TargetOrganization: TargetOrganization,
}


//DirectionParamsValue

/// Направления подготовки ООВО
#[derive(Debug, PartialEq, Default, YaSerialize, YaDeserialize, Validate)]
pub struct OrgDirection {
    #[validate(length(max = 36))]
    pub Uid: String,
    pub IdDirection: u64,
    //DirectionParamsValueList
}

/// Целевые организации
#[derive(Debug, PartialEq, Default, YaSerialize, YaDeserialize, Validate)]
pub struct TargetOrganization {
    #[validate(length(max = 36))]
    pub Uid: String,
    #[validate(length(equal = 13))]
    pub Ogrn: String,
    #[validate(length(equal = 9))]
    pub Kpp: String,
    #[validate(length(equal = 10))]
    pub Inn: String,
    #[validate(length(max = 255))]
    pub ShortTitle: String,
    #[validate(length(max = 500))]
    pub FullTitle: String,
    #[validate(length(max = 2048))]
    pub Address: Option<String>,
    #[validate(length(max = 255))]
    pub Phone: Option<String>,
    #[validate(length(max = 255))]
    pub Email: Option<String>,
    /// ФИО руководителя организации
    #[validate(length(max = 500))]
    pub ChiefNames: Option<String>,
}
