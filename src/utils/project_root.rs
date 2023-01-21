use std::path::PathBuf;
use std::env::current_dir;
use std::io;

pub enum ProjectError {
    PathError(io::Error),
    GitError(String),
}

/// FsStat allows listing files in a directory and fetching the pwd.
/// It's purpose as an abstraction is to allow mocking of a physical
/// filesystem for easy testing.
trait FsStat {
    fn pwd() -> Result<PathBuf, ProjectError>;
    fn ls() -> () {
        todo!();
    }
}

struct PhysicalFs;

impl FsStat for PhysicalFs {
    fn pwd() -> Result<PathBuf, ProjectError> {
        current_dir().map_err(|e| ProjectError::PathError(e))
    }
}

/// Starting from cwd, look upward until you find a .git directory.
/// This is the projcet root.
pub fn find_project_root() -> Result<PathBuf, ProjectError>  {
    find_project_root_dyn(Box::new(FsStat))
}

/// Inject a vtable to look up filesystem entries (for easy mocking).
fn find_project_root_dyn(fs: Box<dyn FsStat>) -> Result<PathBuf, ProjectError> {
    fs.pwd().map(|pwd| find_project_up(fs, pwd))    
}

// TODO: Use the `ignore` crate here?
fn find_project_up(fs: Box<dyn FsStat>, pwd: PathBuf) -> Result<PathBuf, ProjectError> {
    // List the files in this directory. Are any of them `.git`?
}