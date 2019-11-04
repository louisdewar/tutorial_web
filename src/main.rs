mod build;
mod common;
mod parse;
mod templates;
mod web;

use clap::{crate_authors, crate_version, load_yaml, App};

fn main() -> std::io::Result<()> {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from(yaml)
        .version(crate_version!())
        .author(crate_authors!())
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("build") {
        let input = matches.value_of("input-dir").unwrap();
        let static_files = matches.value_of("static-dir").unwrap();
        let output = matches.value_of("output-dir").unwrap();
        let base_url = matches.value_of("base-url").unwrap_or("");

        // Used for checking the validity of the base_url string
        let mut url_chars = base_url.chars();

        // This check only matters if the length is greater than 1
        if base_url.chars().count() > 1 {
            // Get the first char
            assert_eq!(
                url_chars.next().unwrap(),
                '/',
                "The base url should start in a /"
            );
        }
        // Get the last char
        assert_ne!(
            url_chars.last(),
            Some('/'),
            "The base url should not end in a /"
        );

        build::build_html(input, static_files, output, base_url.to_string())?;
    } else if let Some(matches) = matches.subcommand_matches("start-test-server") {
        let input = matches.value_of("input-dir").unwrap();
        let static_files = matches.value_of("static-dir").unwrap();

        web::start_server(static_files.to_string(), input)?;
    }

    Ok(())
}
