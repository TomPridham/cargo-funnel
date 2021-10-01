use toml_edit::{table, Item, Table};

struct OrderedField {
    name: &'static str,
    required: bool,
}

const PACKAGE_FIELDS_ORDER: [OrderedField; 19] = [
    OrderedField {
        name: "name",
        required: true,
    },
    OrderedField {
        name: "version",
        required: true,
    },
    OrderedField {
        name: "edition",
        required: false,
    },
    OrderedField {
        name: "description",
        required: false,
    },
    OrderedField {
        name: "documentation",
        required: false,
    },
    OrderedField {
        name: "homepage",
        required: false,
    },
    OrderedField {
        name: "repository",
        required: false,
    },
    OrderedField {
        name: "readme",
        required: false,
    },
    OrderedField {
        name: "keywords",
        required: false,
    },
    OrderedField {
        name: "categories",
        required: false,
    },
    OrderedField {
        name: "license",
        required: false,
    },
    OrderedField {
        name: "license-file",
        required: false,
    },
    OrderedField {
        name: "authors",
        required: false,
    },
    OrderedField {
        name: "build",
        required: false,
    },
    OrderedField {
        name: "links",
        required: false,
    },
    OrderedField {
        name: "exclude",
        required: false,
    },
    OrderedField {
        name: "include",
        required: false,
    },
    OrderedField {
        name: "publish",
        required: false,
    },
    OrderedField {
        name: "workspace",
        required: false,
    },
];

pub fn sort_package_fields(package: &mut Table) -> Item {
    let sorted_package = PACKAGE_FIELDS_ORDER
        .iter()
        .fold(table(), |mut acc, field| match package.remove(field.name) {
            Some(x) => {
                acc[field.name] = x;
                acc
            }
            None => {
                assert!(field.required, "missing required field: {}", field.name);
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

        #[test]
        fn orders_all_fields_correctly() {
            let fields = r#"
            [package]
            workspace = "i don't like you"
            publish = false
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
    }
}
