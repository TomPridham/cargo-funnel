use toml_edit::{table, Item, Table};

pub fn sort_badges_fields(badges: &mut Table) -> Item {
    let badges_fields_order = [
        ("appveyor", vec!["repository"]),
        ("circle-ci", vec!["repository"]),
        ("gitlab", vec!["repository"]),
        ("azure-devops", vec!["project", "pipeline"]),
        ("travis-ci", vec!["repository"]),
        ("codecov", vec!["repository"]),
        ("coveralls", vec!["repository"]),
        ("is-it-maintained-issue-resolution", vec!["repository"]),
        ("is-it-maintained-open-issues", vec!["repository"]),
        ("maintenance", vec!["status"]),
    ];
    let sorted_badges =
        badges_fields_order
            .iter()
            .fold(table(), |mut acc, field| match badges.remove(field.0) {
                Some(x) => {
                    let val = x.clone();
                    field.1.iter().for_each(|f| {
                        if !val.as_inline_table().unwrap().contains_key(f) {
                            panic!("missing required field: {} for {} badge", f, field.0)
                        }
                    });
                    acc[field.0] = val;
                    acc
                }
                None => acc,
            });
    let sorted_badges = badges.iter().fold(sorted_badges, |mut acc, x| {
        acc[x.0] = x.1.clone();
        acc
    });
    sorted_badges
}

pub fn sort_package_fields(package: &mut Table) -> Item {
    let package_fields_order = [
        ("name", true),
        ("version", true),
        ("authors", false),
        ("edition", false),
        ("build", false),
        ("links", false),
        ("exclude", false),
        ("include", false),
        ("publish", false),
        ("workspace", false),
        ("description", false),
        ("documentation", false),
        ("homepage", false),
        ("repository", false),
        ("readme", false),
        ("keywords", false),
        ("categories", false),
        ("license", false),
        ("license-file", false),
    ];
    let sorted_package = package_fields_order
        .iter()
        .fold(table(), |mut acc, &field| match package.remove(field.0) {
            Some(x) => {
                acc[field.0] = x.clone();
                acc
            }
            None => {
                if field.1 {
                    panic!("missing required field: {}", field.0)
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
