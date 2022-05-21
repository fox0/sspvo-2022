#[cfg(test)]
#[path = "../src/models.rs"]
mod models;


#[cfg(test)]
mod tests {
    use std::fs;
    use yaserde::de::from_str;

    use validator::Validate;

    use crate::models::{OrgDirection, TargetOrganization, XmlOrgDirection, XmlTargetOrganization};

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
                Uid: "32342".to_string(),
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
        <Uid>32342</Uid>
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
                Uid: "32342".to_string(),
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
        <Uid>32342</Uid>
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
                Uid: "32342".to_string(),
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
        <Uid>32342</Uid>
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
}
