#![allow(non_snake_case)]

use yaserde_derive::{YaSerialize, YaDeserialize};
use validator::Validate;

use crate::cls::{EducationForm, EducationLevel};


#[allow(dead_code)]
pub enum Payload {
    /// Направления подготовки ООВО
    OrgDirection,
    /// Целевые организации
    TargetOrganization,
    /// Приемная кампания
    Campaign,
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

#[derive(Debug, PartialEq, YaSerialize, Validate)]
#[yaserde(rename = "PackageData")]
pub struct XmlCampaign {
    pub Campaign: Campaign,
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

/// Приемная кампания
#[derive(Debug, PartialEq, Default, YaSerialize, Validate)]
pub struct Campaign {
    #[validate(length(max = 36))]
    pub Uid: String,
    #[validate(length(max = 500))]
    pub Name: String,
    // #[validate(range(min = 2022, max = 2024))]
    pub YearStart: u64,
    // #[validate(range(min = 2022, max = 2024))]
    pub YearEnd: u64,
    pub IdCampaignType: u64,
    pub IdCampaignStatus: u64,
    /// Максимально возможное количество баллов по всем ИД
    // #[validate(range(min = 1, max = 10))]
    pub MaxCountAchievements: u64,
    /// Максимально возможное количество согласий по одному заявлению
    // #[validate(range(min = 2))]
    pub NumberAgree: u64,
    /// Предельное количество специальностей и (или) направлений подготовки,
    /// по которым поступающий вправе одновременно участвовать в конкурсе
    /// по программам бакалавриата и программам специалитета
    // #[validate(range(min = 1, max = 10))]
    pub CountDirections: u64,
    // todo <EndDate>2022-02-02Т09:09:09+03:00</EndDate>
    pub EducationFormList: EducationFormList,
    pub EducationLevelList: EducationLevelList,
}

#[derive(Debug, PartialEq, Default, YaSerialize, Validate)]
pub struct EducationFormList {
    #[yaserde(flatten)]
    pub data: Vec<EducationForm>,
}

#[derive(Debug, PartialEq, Default, YaSerialize, Validate)]
pub struct EducationLevelList {
    #[yaserde(flatten)]
    pub data: Vec<EducationLevel>,
}
