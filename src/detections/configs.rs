use crate::toml;
use clap::{App, AppSettings, Arg, ArgMatches};
use std::fs::File;
use std::io::prelude::*;
use std::sync::Once;

#[derive(Clone)]
pub struct SingletonReader {
    pub regex: Vec<Vec<String>>,
    pub whitelist: Vec<Vec<String>>,
    pub args: ArgMatches<'static>,
    pub rules: toml::ParseToml,
}

pub enum Lang {
    Ja,
    En,
}

pub fn singleton() -> Box<SingletonReader> {
    static mut SINGLETON: Option<Box<SingletonReader>> = Option::None;
    static ONCE: Once = Once::new();

    let mut toml = toml::ParseToml::new();
    &toml.read_dir("rules".to_string());

    unsafe {
        ONCE.call_once(|| {
            let singleton = SingletonReader {
                regex: read_csv("regexes.txt"),
                whitelist: read_csv("whitelist.txt"),
                args: build_app().get_matches(),
                rules: toml,
            };

            SINGLETON = Some(Box::new(singleton));
        });

        return SINGLETON.clone().unwrap();
    }
}

fn build_app() -> clap::App<'static, 'static> {
    let program = std::env::args()
        .nth(0)
        .and_then(|s| {
            std::path::PathBuf::from(s)
                .file_stem()
                .map(|s| s.to_string_lossy().into_owned())
        })
        .unwrap();

    App::new(program)
        .about("Yea! (Yamato Event Analyzer). Aiming to be the world's greatest Windows event log analysis tool!")
        .version("0.0.1")
        .author("Author name <author@example.com>")
        .setting(AppSettings::VersionlessSubcommands)
        .arg(Arg::from_usage("-f --filepath=[FILEPATH] 'event file path'"))
        .arg(Arg::from_usage("--attackhunt=[ATTACK_HUNT] 'Attack Hunt'"))
        .arg(Arg::from_usage("--csv-timeline=[CSV_TIMELINE] 'csv output timeline'"))
        .arg(Arg::from_usage("--human-readable-timeline=[HUMAN_READABLE_TIMELINE] 'human readable timeline'"))
        .arg(Arg::from_usage("-l --lang=[LANG] 'output language'"))
        .arg(Arg::from_usage("-t --timezone=[TIMEZONE] 'timezone setting'"))
        .arg(Arg::from_usage("-d --directory 'event log files directory'"))
        .arg(Arg::from_usage("-s --statistics 'event statistics'"))
        .arg(Arg::from_usage("-u --update 'signature update'"))
        .arg(Arg::from_usage("--credits 'Zachary Mathis, Akira Nishikawa'"))
}

fn read_csv(filename: &str) -> Vec<Vec<String>> {
    let mut f = File::open(filename).expect("file not found!!!");
    let mut contents: String = String::new();
    let mut ret = vec![];
    if f.read_to_string(&mut contents).is_err() {
        return ret;
    }

    let mut rdr = csv::Reader::from_reader(contents.as_bytes());
    rdr.records().for_each(|r| {
        if r.is_err() {
            return;
        }

        let line = r.unwrap();
        let mut v = vec![];
        line.iter().for_each(|s| v.push(s.to_string()));
        ret.push(v);
    });

    return ret;
}
