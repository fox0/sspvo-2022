#[cfg(test)]
#[path = "../src/cls.rs"]
mod cls;
#[cfg(test)]
#[path = "../src/models.rs"]
mod models;


#[cfg(test)]
mod tests {
    use std::fs;
    use yaserde::de::from_str;

    use validator::Validate;

    use crate::cls::{EducationForm, EducationLevel};
    use crate::models::{Campaign, EducationFormList, EducationLevelList, OrgDirection, TargetOrganization, XmlCampaign, XmlOrgDirection, XmlTargetOrganization};

    fn get_config() -> yaserde::ser::Config {
        yaserde::ser::Config {
            perform_indent: true,
            write_document_declaration: false,
            indent_string: Some("    ".to_string()),
            // ..Default::default()
        }
    }

    #[test]
    fn org_direction_ser() {
        let model = XmlOrgDirection {
            OrgDirection: OrgDirection {
                Uid: "f983c809-b827-4812-b9b3-3a75697af2f2".to_string(),
                IdDirection: 42,
            }
        };
        model.validate().unwrap();

        let config = get_config();
        let xml = yaserde::ser::to_string_with_config(&model, &config).ok().unwrap();
        // println!("{}", &xml);
        assert_eq!(xml, "\
<PackageData>
    <OrgDirection>
        <Uid>f983c809-b827-4812-b9b3-3a75697af2f2</Uid>
        <IdDirection>42</IdDirection>
    </OrgDirection>
</PackageData>");
    }

    #[test]
    fn org_direction_de() {
        let xml = fs::read_to_string("tests/Payload/OrgDirection/Add.xml").unwrap();
        // println!("{:?}", xml);
        let obj: XmlOrgDirection = from_str(&xml).unwrap();
        let reference = XmlOrgDirection {
            OrgDirection: OrgDirection {
                Uid: "string".to_string(),
                IdDirection: 200,
            }
        };
        assert_eq!(obj, reference);
    }

    #[test]
    fn target_organization_ser() {
        let model = XmlTargetOrganization {
            TargetOrganization: TargetOrganization {
                Uid: "f983c809-b827-4812-b9b3-3a75697af2f2".to_string(),
                Ogrn: "1234567800013".to_string(),
                Kpp: "123456789".to_string(),
                Inn: "1234567810".to_string(),
                ShortTitle: "short".to_string(),
                FullTitle: "full".to_string(),
                Address: None,
                Phone: None,
                Email: None,
                ChiefNames: None,
            }
        };
        model.validate().unwrap();

        let config = get_config();
        let xml = yaserde::ser::to_string_with_config(&model, &config).ok().unwrap();
        // println!("{}", &xml);
        assert_eq!(xml, "\
<PackageData>
    <TargetOrganization>
        <Uid>f983c809-b827-4812-b9b3-3a75697af2f2</Uid>
        <Ogrn>1234567800013</Ogrn>
        <Kpp>123456789</Kpp>
        <Inn>1234567810</Inn>
        <ShortTitle>short</ShortTitle>
        <FullTitle>full</FullTitle>
    </TargetOrganization>
</PackageData>");
    }

    #[test]
    fn target_organization_ser2() {
        let model = XmlTargetOrganization {
            TargetOrganization: TargetOrganization {
                Uid: "f983c809-b827-4812-b9b3-3a75697af2f2".to_string(),
                Ogrn: "1234567800013".to_string(),
                Kpp: "123456789".to_string(),
                Inn: "1234567810".to_string(),
                ShortTitle: "short".to_string(),
                FullTitle: "full".to_string(),
                Address: Some("11".to_string()),
                Phone: Some("22".to_string()),
                Email: Some("33".to_string()),
                ChiefNames: Some("44".to_string()),
            }
        };
        model.validate().unwrap();

        let config = get_config();
        let xml = yaserde::ser::to_string_with_config(&model, &config).ok().unwrap();
        // println!("{}", &xml);
        assert_eq!(xml, "\
<PackageData>
    <TargetOrganization>
        <Uid>f983c809-b827-4812-b9b3-3a75697af2f2</Uid>
        <Ogrn>1234567800013</Ogrn>
        <Kpp>123456789</Kpp>
        <Inn>1234567810</Inn>
        <ShortTitle>short</ShortTitle>
        <FullTitle>full</FullTitle>
        <Address>11</Address>
        <Phone>22</Phone>
        <Email>33</Email>
        <ChiefNames>44</ChiefNames>
    </TargetOrganization>
</PackageData>");
    }

    #[test]
    fn target_organization_de() {
        let xml = fs::read_to_string("tests/Payload/TargetOrganization/Add.xml").unwrap();
        // println!("{:?}", xml);
        let obj: XmlTargetOrganization = from_str(&xml).unwrap();
        let reference = XmlTargetOrganization {
            TargetOrganization: TargetOrganization {
                Uid: "string".to_string(),
                Ogrn: "string".to_string(),
                Kpp: "string".to_string(),
                Inn: "string".to_string(),
                ShortTitle: "string".to_string(),
                FullTitle: "string".to_string(),
                Address: Some("string".to_string()),
                Phone: Some("string".to_string()),
                Email: Some("string".to_string()),
                ChiefNames: None,
            }
        };
        assert_eq!(obj, reference);
    }

    #[test]
    fn campaign_ser() {
        let model = XmlCampaign {
            Campaign: Campaign {
                Uid: "f983c809-b827-4812-b9b3-3a75697af2f2".to_string(),
                Name: "name".to_string(),
                YearStart: 2022,
                YearEnd: 2022,
                IdCampaignType: 1,
                IdCampaignStatus: 1,
                MaxCountAchievements: 10,
                NumberAgree: 2,
                CountDirections: 3,
                EducationFormList: EducationFormList {
                    data: vec![EducationForm { id: 1 }, EducationForm { id: 2 }]
                },
                EducationLevelList: EducationLevelList {
                    data: vec![EducationLevel { id: 3 }, EducationLevel { id: 4 }, EducationLevel { id: 5 }]
                },
            }
        };
        model.validate().unwrap();

        let config = get_config();
        let xml = yaserde::ser::to_string_with_config(&model, &config).ok().unwrap();
        // println!("{}", &xml);
        assert_eq!(xml, "\
<PackageData>
    <Campaign>
        <Uid>f983c809-b827-4812-b9b3-3a75697af2f2</Uid>
        <Name>name</Name>
        <YearStart>2022</YearStart>
        <YearEnd>2022</YearEnd>
        <IdCampaignType>1</IdCampaignType>
        <IdCampaignStatus>1</IdCampaignStatus>
        <MaxCountAchievements>10</MaxCountAchievements>
        <NumberAgree>2</NumberAgree>
        <CountDirections>3</CountDirections>
        <EducationFormList>
            <IdEducationForm>1</IdEducationForm>
            <IdEducationForm>2</IdEducationForm>
        </EducationFormList>
        <EducationLevelList>
            <IdEducationLevel>3</IdEducationLevel>
            <IdEducationLevel>4</IdEducationLevel>
            <IdEducationLevel>5</IdEducationLevel>
        </EducationLevelList>
    </Campaign>
</PackageData>");
    }
}
