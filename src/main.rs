use clap::{App, Arg};
use csv;
use csv::{Reader, StringRecord};
use std::collections::HashMap;
use std::error::Error;
use std::{fs, io};

/// convert csv::Reader to json object map
/// csv::Reader is an abstract input object, represent for input both from stdin or csv file.
///
fn csv_to_json<R: std::io::Read>(
    rdr: &mut Reader<R>,
    limit: Option<usize>,
    offset: Option<usize>,
) -> Result<Vec<HashMap<String, String>>, Box<dyn Error>> {
    let header: StringRecord = (rdr.headers()?).clone();

    let mut res: Vec<HashMap<String, String>> = Vec::new();

    let n: usize = limit.unwrap_or(usize::max_value());
    let s: usize = offset.unwrap_or(0);

    for record in rdr.records().skip(s).take(n) {
        match record {
            Ok(r) => {
                let map: HashMap<String, String> = header
                    .clone()
                    .into_iter()
                    .map(|s| s.to_owned())
                    .zip(r.into_iter().map(|s| s.to_owned()))
                    .collect();
                res.push(map);
            }
            Err(e) => {
                //throw error, avoid swollow error！
                return Err(Box::new(e));
            }
        }
    }

    Ok(res)
}

/// write json string with abstract writer.
/// if writer is oi::stdout() , then it write to stdout
/// if writer is some io::BufWriter, then it will writer to a file
/// you can alse complement yourself writer to write to specifica place
fn write_json<W: std::io::Write>(writer: &mut W, json: String) -> io::Result<()> {
    writer.write(&json.into_bytes())?;
    writer.flush()?;
    Ok(())
}

fn main() -> io::Result<()> {
    // cli args parser with clap
    let matches = App::new("convert csv to json format")
        .author("heguangfu")
        .about("csv to json tools")
        .version("0.0.1")
        .arg(
            Arg::with_name("verbose")
                .short("-v")
                .long("verbose")
                .help("get some detail infomation od csv to json"),
        )
        .arg(
            Arg::with_name("input_csv")
                .short("i")
                .long("input-file")
                .help("input csv file path, either input_csv or stdin_csv is required!")
                .conflicts_with("stdin_csv")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("output_json")
                .short("o")
                .long("output-file")
                .takes_value(true)
                .help("result json file path"),
        )
        .arg(Arg::from_usage("-p, --pretty 'output json pretty format'"))
        .arg(
            Arg::from_usage("-l, --limit 'output ${limit}json objects with '")
                .takes_value(true)
                .min_values(0)
                .max_values(i32::max_value() as u64),
        )
        .arg(
            Arg::from_usage("-s, --offset 'output json objects start from ${offset}'")
                .takes_value(true)
                .default_value("0")
                .min_values(0)
                .max_values(i32::max_value() as u64),
        )
        .arg(
            Arg::with_name("stdin_csv")
                .help("stdin csv content, either input_csv or stdin_csv is required!")
                .conflicts_with("input_csv")
                .required_unless("input_csv")
                .takes_value(true),
        )
        .get_matches();

    let mut pretty: bool = false;
    if matches.occurrences_of("pretty") > 0 {
        pretty = true;
    }

    let mut verbose: bool = false;
    if matches.occurrences_of("verbose") > 0 {
        verbose = true;
    }

    if verbose {
        println!("user input:\n{:?}", matches);
    }

    let mut limit: Option<usize> = None;
    if let Some(l) = matches.value_of("limit") {
        limit = Some(usize::from_str_radix(l, 10).expect("limit input error!"));
    }

    let mut offset: Option<usize> = None;
    if let Some(s) = matches.value_of("offset") {
        offset = Some(usize::from_str_radix(s, 10).expect("offset input error!"));
    }

    let mut writer: Option<io::BufWriter<fs::File>> = None;
    if let Some(o) = matches.value_of("output_json") {
        writer = Some(io::BufWriter::new(fs::File::create(o)?));
    }

    let list: Result<Vec<HashMap<String, String>>, Box<dyn Error>>;
    if let Some(i) = matches.value_of("input_csv") {
        let msg = if verbose {
            String::from("cannot read input csv file, file = ") + i
        } else {
            String::from("cannot read input csv file")
        };
        let mut rdr: Reader<fs::File> = csv::Reader::from_path(i).expect(msg.as_str());
        list = csv_to_json(&mut rdr, limit, offset);
    } else {
        if verbose {
            println!("get csv from stdin",);
        }
        let mut rdr = csv::Reader::from_reader(io::stdin());
        list = csv_to_json(&mut rdr, limit, offset);
    }

    match list {
        Ok(list_objects) => {
            let json = if pretty {
                serde_json::to_string_pretty(&list_objects)?
            } else {
                serde_json::to_string(&list_objects)?
            };
            if let Some(mut w) = writer {
                write_json(&mut w, json)?;
                if verbose {
                    println!(
                        "output json file (records: {}) : {:?}",
                        list_objects.len(),
                        matches.value_of("output_json")
                    );
                }
            } else {
                write_json(&mut io::stdout(), json)?;
            }
        }
        Err(e) => {
            println!("Error: {:?}", e);
            return Ok(());
        }
    }
    Ok(())
}
