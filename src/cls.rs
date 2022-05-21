#![allow(dead_code)]
//! Справочники
use yaserde_derive::YaSerialize;


#[derive(Debug, PartialEq, YaSerialize)]
pub struct EducationForm {
    #[yaserde(rename = "IdEducationForm")]
    pub id: u64,
}

#[derive(Debug, PartialEq, YaSerialize)]
pub struct EducationLevel {
    #[yaserde(rename = "IdEducationLevel")]
    pub id: u64,
}
