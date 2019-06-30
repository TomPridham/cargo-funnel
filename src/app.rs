use clap::{App, Arg};

pub fn build_app<'a, 'b>() -> App<'a, 'b> {
    App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .version_short("v")
        .arg(
            Arg::with_name("funnel")
                .possible_value("funnel")
                .index(1)
                .required(true)
                .hidden(true),
        )
}
