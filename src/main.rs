mod build;
mod common;
mod templates;
mod web;

use clap::{crate_version, App, AppSettings, Arg, SubCommand};

fn main() -> std::io::Result<()> {
    let matches = App::new("Tutorial Web Builder")
        .author("Author: Louis de Wardt")
        .about("Converts yaml files into html websites")
        .version(crate_version!())
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommand(
            SubCommand::with_name("start-test-server")
                .about("Runs a local server hosting the files (purely for testing)")
                .arg(
                    Arg::with_name("input-dir")
                        .short("i")
                        .required(true)
                        .takes_value(true)
                        .help("The directory of the courses"),
                )
                .arg(
                    Arg::with_name("static-dir")
                        .short("s")
                        .required(true)
                        .takes_value(true)
                        .help("The directory of the static files to be bundled under /static/"),
                )
        )
        .subcommand(
            SubCommand::with_name("build")
                .about("Builds the tutorial files into static HTML files ready for production")
                .arg(
                    Arg::with_name("input-dir")
                        .short("i")
                        .required(true)
                        .takes_value(true)
                        .help("The directory of the courses"),
                )
                .arg(
                    Arg::with_name("static-dir")
                        .short("s")
                        .required(true)
                        .takes_value(true)
                        .help("The directory of the static files to be bundled under /static/"),
                )
                .arg(
                    Arg::with_name("output-dir")
                        .short("o")
                        .required(true)
                        .takes_value(true)
                        .help("The name of the output directory (it will be created if it doesn't exist)"),
                )
                .arg(
                    Arg::with_name("base-url")
                        .short("b")
                        .takes_value(true)
                        .help(
                            "The base url for root links (i.e. the url under which the output folder will live).
Defaults to ''. Also it should NOT end in '/', although it should contain a starting '/' (unless it is empty)."
                        ),
                ),
        )
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
