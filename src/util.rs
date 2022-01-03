use toml_edit::{table, Item, Table};

struct OrderedField {
    name: &'static str,
    required_always: bool,
    required_publish: bool,
}

const PACKAGE_FIELDS_ORDER: [OrderedField; 27] = [
    OrderedField {
        name: "name",
        required_always: true,
        required_publish: true,
    },
    OrderedField {
        name: "version",
        required_always: true,
        required_publish: true,
    },
    OrderedField {
        name: "authors",
        required_always: false,
        required_publish: false,
    },
    OrderedField {
        name: "edition",
        required_always: false,
        required_publish: false,
    },
    OrderedField {
        name: "rust-version",
        required_always: false,
        required_publish: false,
    },
    OrderedField {
        name: "description",
        required_always: false,
        required_publish: true,
    },
    OrderedField {
        name: "documentation",
        required_always: false,
        required_publish: true,
    },
    OrderedField {
        name: "readme",
        required_always: false,
        required_publish: true,
    },
    OrderedField {
        name: "homepage",
        required_always: false,
        required_publish: true,
    },
    OrderedField {
        name: "repository",
        required_always: false,
        required_publish: true,
    },
    OrderedField {
        name: "license",
        required_always: false,
        required_publish: true,
    },
    OrderedField {
        name: "license-file",
        required_always: false,
        required_publish: true,
    },
    OrderedField {
        name: "keywords",
        required_always: false,
        required_publish: false,
    },
    OrderedField {
        name: "categories",
        required_always: false,
        required_publish: false,
    },
    OrderedField {
        name: "workspace",
        required_always: false,
        required_publish: false,
    },
    OrderedField {
        name: "build",
        required_always: false,
        required_publish: false,
    },
    OrderedField {
        name: "links",
        required_always: false,
        required_publish: false,
    },
    OrderedField {
        name: "exclude",
        required_always: false,
        required_publish: false,
    },
    OrderedField {
        name: "include",
        required_always: false,
        required_publish: false,
    },
    OrderedField {
        name: "publish",
        required_always: false,
        required_publish: false,
    },
    OrderedField {
        name: "metadata",
        required_always: false,
        required_publish: false,
    },
    OrderedField {
        name: "default-run",
        required_always: false,
        required_publish: false,
    },
    OrderedField {
        name: "autobins",
        required_always: false,
        required_publish: false,
    },
    OrderedField {
        name: "autoexamples",
        required_always: false,
        required_publish: false,
    },
    OrderedField {
        name: "autotests",
        required_always: false,
        required_publish: false,
    },
    OrderedField {
        name: "autobenches",
        required_always: false,
        required_publish: false,
    },
    OrderedField {
        name: "resolver",
        required_always: false,
        required_publish: false,
    },
];

pub fn sort_package_fields(package: &mut Table) -> Item {
    let is_publish = if let Some(publish) = package.get("publish") {
        publish.as_bool().unwrap()
    } else {
        true
    };

    let has_license_or_license_file =
        package.get("license").is_some() || package.get("license-file").is_some();
    let sorted_package =
        PACKAGE_FIELDS_ORDER
            .iter()
            .fold(table(), |mut acc, field| match package.remove(field.name) {
                Some(x)=>{
                    acc[field.name] = x;
                    acc
                }
                None => {
                    if is_publish {
                        if field.name == "license" || field.name == "license-file"  {
                            assert!(has_license_or_license_file, "one of [`license`, `license-file`] is required when publishing to crates.io");
                        } else {
                            assert!(!(field.required_publish || field.required_always), "missing required field: {}", field.name);
                        }
                    } else {
                        assert!(!field.required_always, "missing required field: {}", field.name);
                    }

                    acc
                }
            });
    let sorted_package = package.iter().fold(sorted_package, |mut acc, x| {
        acc[x.0] = x.1.clone();
        acc
    });
    sorted_package
}

#[cfg(test)]
mod util_test {
    use super::*;
    use toml_edit::Document;

    mod sort_package_fields_test {
        use super::*;
        use test_case::test_case;

        #[test]
        fn orders_all_fields_correctly() {
            let fields = r#"
            [package]
            workspace = "i don't like you"
            publish = false
            rust-version = 1.40
            metadata = []
            default-run = "oh my god.exe"
            autobins = false
            autoexamples = false
            autotests = false
            autobenches = false
            resolver = 2
            include = "out of control"
            exclude = "i don't like youuuuuuu"
            links = ["i don't the things you say"]
            build = "say it right to my face"
            authors = ["out of control", "don't wanna see me lose my mind"]
            license-file = "things u do.txt"
            license = "MIT"
            categories = ["can't", "get", "in", "my", "head"]
            keywords = []
            readme = "README.md"
            repository = "dont like u .git"
            homepage = "overwhelmed.com"
            documentation = "docs.rs/but im not"
            description = "words come over me, feels like im somebody else"
            edition = "2018"
            version = "1.0.0"
            name = "overwhelmed"
            "#;

            let mut toml = fields.parse::<Document>().expect("heck");

            let package_fields = toml["package"].as_table_mut().unwrap();
            toml["package"] = sort_package_fields(package_fields);
            insta::assert_snapshot!(format!("{}", toml.to_string()));
        }

        #[test]
        #[should_panic]
        fn panics_if_name_missing() {
            std::panic::set_hook(Box::new(|_| {}));
            let fields = r#"
            [package]
            version = "1.0.0"
            "#;

            let mut toml = fields.parse::<Document>().expect("heck");

            let package_fields = toml["package"].as_table_mut().unwrap();
            toml["package"] = sort_package_fields(package_fields);
        }

        #[test]
        #[should_panic]
        fn panics_if_version_missing() {
            std::panic::set_hook(Box::new(|_| {}));
            let fields = r#"
            [package]
            name = "back down"
            "#;

            let mut toml = fields.parse::<Document>().expect("heck");

            let package_fields = toml["package"].as_table_mut().unwrap();
            toml["package"] = sort_package_fields(package_fields);
        }

        #[test]
        fn only_requires_name_and_version_if_publish_false() {
            let fields = r#"
            [package]
            version = "1.0.0"
            name = "overwhelmed"
            publish = false
            "#;

            let mut toml = fields.parse::<Document>().expect("heck");

            let package_fields = toml["package"].as_table_mut().unwrap();
            toml["package"] = sort_package_fields(package_fields);
            insta::assert_snapshot!(format!("{}", toml.to_string()));
        }

        #[test_case("" => panics; "publish missing")]
        #[test_case("publish = true" => panics; "publish true")]
        #[test_case(r#"description= "check in""# => panics; "documentation missing")]
        #[test_case(r#"
            description= "shoulda been a dentist"
            documentation = "mom said it"
        "# => panics; "homepage missing")]
        #[test_case(r#"
            description= "shoulda been a dentist"
            documentation = "mom said it"
            homepage = "lie cheat steal"
        "# => panics; "repository missing")]
        #[test_case(r#"
            description= "shoulda been a dentist"
            documentation = "mom said it"
            homepage = "lie cheat steal"
            repository = "kill win"
        "# => panics; "readme missing")]
        #[test_case(r#"
            description= "shoulda been a dentist"
            documentation = "mom said it"
            homepage = "lie cheat steal"
            repository = "kill win"
            readme = "banging on my adversaries"
        "# => panics; "license")]
        #[test_case(r#"
            description= "shoulda been a dentist"
            documentation = "mom said it"
            homepage = "lie cheat steal"
            repository = "kill win"
            readme = "banging on my adversaries"
            license = "MIT"
        "# ; "all present")]
        #[test_case(r#"
            description= "shoulda been a dentist"
            documentation = "mom said it"
            homepage = "lie cheat steal"
            repository = "kill win"
            readme = "banging on my adversaries"
            license-file = "license.txt"
        "# ; "all present - license file")]
        fn requires_more_fields_if_publish_true_or_missing(extra_fields: &str) {
            let fields = r#"
            [package]
            version = "1.0.0"
            name = "overwhelmed"
            "#;
            let fields = format!("{}{}", fields, extra_fields);

            let mut toml = fields.parse::<Document>().expect("heck");

            let package_fields = toml["package"].as_table_mut().unwrap();
            toml["package"] = sort_package_fields(package_fields);
        }
    }
}
