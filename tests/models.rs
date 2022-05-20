#[cfg(test)]
#[path = "../src/models.rs"]
mod models;


#[cfg(test)]
mod tests {
    use validator::Validate;
    use crate::models::{OrgDirection, PackageData, TargetOrganization};

    fn get_config() -> yaserde::ser::Config {
        yaserde::ser::Config {
            perform_indent: true,
            write_document_declaration: false,
            indent_string: Some("    ".to_string()),
            // ..Default::default()
        }
    }

    #[test]
    fn org_direction() {
        let obj = OrgDirection {
            Uid: "32342".to_string(),
            IdDirection: 42,
        };
        obj.validate().unwrap();
        let model = PackageData::OrgDirection(obj);

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
    fn target_organization() {
        let obj = TargetOrganization {
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
        };
        obj.validate().unwrap();
        let model = PackageData::TargetOrganization(obj);

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
    fn target_organization2() {
        let obj = TargetOrganization {
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
        };
        obj.validate().unwrap();
        let model = PackageData::TargetOrganization(obj);

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
