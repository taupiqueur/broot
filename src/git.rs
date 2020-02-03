
use {
    std::{
        path::{
            Path,
            PathBuf,
        },
    },
};

/// return the closest parent (or self) containing a
/// .git file
pub fn closest_repo_dir(mut path: &Path) -> Option<PathBuf> {
    loop {
        let c = path.join(".git");
        if c.exists() {
            return Some(path.to_path_buf());
        }
        path = match path.parent() {
            Some(path) => path,
            None => {
                return None;
            }
        };
    }
}
