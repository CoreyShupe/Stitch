use clap::{Parser, ValueEnum};
use std::fmt::Display;
use std::path::PathBuf;

const VERSION: &str = env!("CARGO_PKG_VERSION");

/// A command line tool for managing javascript projects at scale.
///
/// This tool provides debugging systems for finding and fixing various common user-errors in
/// javascript projects.
///
/// This tool also provides a way to view the structure of your javascript project; seeing how
/// each file and function interacts with the others. This can potentially help find problems
/// in the basic design of your program. Complicated intertwined programs which are hard to
/// explain can often be re-designed in a more "simple" way. This tool aims to help developers
/// find these offending modules.
#[derive(Debug, Parser)]
#[command(
    name = "Stitch",
    author = "Corey Shupe",
    version = VERSION,
    verbatim_doc_comment,
)]
pub struct CommandLineArgs {
    /// Entry point path of the javascript program.
    #[arg(short, long, required = true, default_value = "index.js")]
    pub entry_point: PathBuf,

    /// Verbosity level of the program.
    #[arg(short, long, required = false, default_value_t)]
    pub verbose: bool,

    /// The project type to use.
    ///
    /// Currently only ecma is officially "supported" but the default ecma parser may simply work for all projects.
    ///
    /// The project structures which have known different compilation requirements will be listed separately from ecma.
    #[arg(short, long, required = false, default_value_t)]
    pub project_type: ProjectType,
}

#[derive(ValueEnum, Debug, Clone)]
pub enum ProjectType {
    #[value(name = "ecma", alias = "ecma")]
    Ecma,
}

impl Default for ProjectType {
    fn default() -> Self {
        ProjectType::Ecma
    }
}

impl Display for ProjectType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProjectType::Ecma => write!(f, "ecma"),
        }
    }
}
