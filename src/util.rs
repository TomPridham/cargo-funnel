use toml_edit::Table;

pub fn sort_package_fields(package: &mut Table) -> Table {
    let package_fields_order = vec![
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
        .fold(Table::new(), |mut acc, &field| {
            match package.remove(field.0) {
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
            }
        });
    let sorted_package = package.iter().fold(sorted_package, |mut acc, x| {
        acc[x.0] = x.1.clone();
        acc
    });
    sorted_package
}
