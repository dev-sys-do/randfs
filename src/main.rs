use clap::Parser;
use fuser::{
    FileAttr, FileType, Filesystem, MountOption, ReplyAttr, ReplyData, ReplyDirectory, ReplyEntry,
    Request,
};
use libc::ENOENT;
use std::collections::HashMap;
use std::ffi::OsStr;
use std::path::PathBuf;
use std::time::{Duration, UNIX_EPOCH};

#[derive(Debug)]
enum Error {}

struct RandFs;

impl RandFs {
    fn new() -> Self {
        RandFs {
        }
    }
}

impl Filesystem for RandFs {}

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// randfs mount point (e.g. /tmp/randfs)
    #[arg(required = true, short, long)]
    mount_point: PathBuf,
}

fn main() -> Result<(), Error> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("warn"));
    let cli = Cli::parse();
    let options = vec![MountOption::RO, MountOption::FSName("randfs".to_string())];

    let mount_point = cli.mount_point.to_str().unwrap();

    fuser::mount2(RandFs::new(), mount_point, &options).unwrap();

    Ok(())
}
