use std::option::Option;
use std::path::PathBuf;

use clap::Parser;

use crate::{Format, Template};

#[derive(Parser, Debug)]
#[clap(
    author = "Brian Reece",
    version = "v0.3-alpha",
    about = "A tool for centralizing scripted commands via a configurable markup file",
    long_about = None
)]
pub struct Args {
    #[clap(
        short,
        long,
        parse(from_os_str),
        value_name = "PATH",
        help = "Path to config file"
    )]
    pub path: Option<PathBuf>,

    #[clap(
        short,
        long,
        arg_enum,
        group = "format_group",
        value_name = "FORMAT",
        help = "Config file format"
    )]
    pub format: Option<Format>,

    #[clap(
        short,
        long,
        arg_enum,
        group = "template_group",
        value_name = "TEMPLATE",
        help = "Config file template"
    )]
    pub template: Option<Template>,

    #[clap(short, long, parse(from_occurrences), help = "Sets verbosity level")]
    pub verbose: usize,

    #[clap(
        short,
        long,
        requires = "format_group",
        requires = "template_group",
        help = "Generate config file"
    )]
    pub generate: bool,

    #[clap(
        short,
        long,
        requires = "format_group",
        help = "Convert between markup formats"
    )]
    pub convert: bool,

    #[clap(help = "Formatted string specifying command targets")]
    pub targets: Option<String>,
}
