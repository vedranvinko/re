extern crate clap;
use clap::{Arg, App};

use serde::{Deserialize};

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::io::prelude::*;

#[derive(Debug, Deserialize)]
struct Config {
    delimiter: Option<String>,
    url: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            delimiter: Some(String::from(",")),
            url: Some(String::from("https://example.org")),
        }
    }
}

fn create_map(lines: Vec<String>, config: Config) -> HashMap<String, String> {
    let delimiter = config.delimiter.unwrap();
    let url = config.url.unwrap();
    let mut urls: HashMap<String, String> = HashMap::new();

    for line in lines {
        let pair: Vec<&str> = line.split(&delimiter).collect();
        urls.insert(String::from(pair[0].replace(&url, "")), String::from(pair[1].replace(&url, "")));
    }

    urls
}

fn get_lines(data: String) -> Vec<String> {
    data.lines().map(|e| String::from(e)).collect::<Vec<String>>()
}

fn read_config(file_path: String) -> Result<Config, ::std::io::Error> {
    let data = match read_file(file_path) {
        Ok(data) => data,
        Err(_)   => panic!("Couldn't read config file"),
    };

    let config: Config = match toml::from_str(&data) {
        Ok(data) => data,
        Err(_)   => panic!("Cannot deserialize config"),
    };

    Ok(config)
}

fn read_file(file_path: String) -> Result<String, ::std::io::Error> {
    let mut file = File::open(file_path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

fn write_file(map: HashMap<String, String>, output: String) {
    let fh = File::create(output).unwrap();
    let mut fs = BufWriter::new(&fh);

    writeln!(&mut fs, "<httpRedirect enabled=\"true\" exactDestination=\"true\" httpResponseStatus=\"Permanent\">").unwrap();
    for (key, value) in map {
        writeln!(&mut fs, "\t<add wildcard=\"{}\" destination=\"{}\" />", key, value).unwrap();
    }
    writeln!(&mut fs, "</httpRedirect>").unwrap();
}

fn main() {
    let opts = App::new("r(edirection) e(ngine)")
                    .version("0.0.1")
                    .author("vedranvinko")
                    .arg(Arg::with_name("config")
                        .long("config")
                        .required(false)
                        .short("c")
                        .takes_value(true)
                        .help("Specify config file to overwrite defaults"))
                    .arg(Arg::with_name("input")
                        .long("input")
                        .required(true)
                        .short("i")
                        .takes_value(true)
                        .help("Specify an input file"))
                    .arg(Arg::with_name("output")
                        .default_value("httpRedirects.config")
                        .long("output")
                        .required(false)
                        .short("o")
                        .takes_value(true)
                    .help("Specify an output file"))
                    .get_matches();
    
    let config = match opts.is_present("config") {
        true  => {
            let config = match read_config(String::from(opts.value_of("config").unwrap())) {
                Ok(data) => data,
                Err(_)   => return,
            };
            config
        },
        false => Config::default(),
    };

    let raw = read_file(String::from(opts.value_of("input").unwrap()));

    let pairs = get_lines(raw.unwrap());

    let m = create_map(pairs, config);

    write_file(m, String::from(opts.value_of("output").unwrap()));
}
