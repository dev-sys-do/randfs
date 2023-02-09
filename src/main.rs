use clap::Parser;
use fuser::{
    FileAttr, FileType, Filesystem, MountOption, ReplyAttr, ReplyDirectory, ReplyEntry, Request,
};
use libc::ENOENT;
use std::ffi::OsStr;
use std::path::PathBuf;
use std::time::{Duration, UNIX_EPOCH};

const RANDFS_ROOT_DIR_INODE: u64 = 1;

const RANDFS_DIR_ATTR: FileAttr = FileAttr {
    ino: RANDFS_ROOT_DIR_INODE,
    size: 0,
    blocks: 0,
    atime: UNIX_EPOCH, // 1970-01-01 00:00:00
    mtime: UNIX_EPOCH,
    ctime: UNIX_EPOCH,
    crtime: UNIX_EPOCH,
    kind: FileType::Directory,
    perm: 0o755,
    nlink: 2,
    uid: 501,
    gid: 20,
    rdev: 0,
    flags: 0,
    blksize: 512,
};

#[derive(Debug)]
enum Error {}

struct ReplyDirectoryEntry {
    inode: u64,
    kind: FileType,
    name: String,
}

struct RandFs;

impl RandFs {
    fn new() -> Self {
        RandFs {}
    }
}

impl Filesystem for RandFs {
    fn lookup(&mut self, _req: &Request, parent: u64, name: &OsStr, reply: ReplyEntry) {
        log::debug!("lookup {:?} for parent {parent}", name);

        reply.error(ENOENT);
    }

    fn getattr(&mut self, _req: &Request, inode: u64, reply: ReplyAttr) {
        log::debug!("getattr for {inode}");
        if inode == RANDFS_ROOT_DIR_INODE {
            reply.attr(&Duration::from_secs(1), &RANDFS_DIR_ATTR);
        } else {
            reply.error(ENOENT);
        }
    }

    fn readdir(
        &mut self,
        _req: &Request,
        inode: u64,
        _fh: u64,
        offset: i64,
        mut reply: ReplyDirectory,
    ) {
        log::debug!("readdir for {inode} at {offset}");
        if inode != RANDFS_ROOT_DIR_INODE {
            return reply.error(ENOENT);
        }

        let entries = vec![
            ReplyDirectoryEntry {
                inode: RANDFS_ROOT_DIR_INODE,
                kind: FileType::Directory,
                name: ".".to_string(),
            },
            ReplyDirectoryEntry {
                inode: RANDFS_ROOT_DIR_INODE,
                kind: FileType::Directory,
                name: "..".to_string(),
            },
        ];

        for (i, e) in entries.into_iter().enumerate().skip(offset as usize) {
            // i + 1 means the index of the next entry
            if reply.add(e.inode, (i + 1) as i64, e.kind, e.name) {
                break;
            }
        }

        reply.ok();
    }
}

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
