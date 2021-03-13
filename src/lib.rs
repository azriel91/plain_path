#![deny(missing_debug_implementations, missing_docs)]

//! Expands `~` in a path if present.
//!
//! # Examples
//!
//! ```rust
//! use std::path::Path;
//! use plain_path::PlainPathExt;
//!
//! # fn main() -> Result<(), plain_path::HomeDirNotFound> {
//! let path = Path::new("~/.ssh/config").plain()?;
//!
//! // 🍏: "/Users/<user>/.ssh/config"
//! // 🐧: "/home/<user>/.ssh/config"
//! println!("{}", path.display());
//! # Ok(())
//! # }
//! ```

use std::{
    fmt,
    path::{Path, PathBuf},
};

use std::borrow::Cow;

/// Provides the [`plain`][PlainPathExt::plain] method to expand `~`.
pub trait PlainPathExt {
    /// Returns the path without special expansion characters.
    ///
    /// If there are no expansion characters, the original path is returned
    /// under the `Cow::Borrowed` variant, otherwise an owned
    /// [`PathBuf`][std::path::PathBuf] is returned.
    fn plain(&self) -> Result<Cow<'_, Path>, HomeDirNotFound>;
}

impl PlainPathExt for Path {
    fn plain(&self) -> Result<Cow<'_, Path>, HomeDirNotFound> {
        crate::plain(self)
    }
}

impl PlainPathExt for PathBuf {
    fn plain(&self) -> Result<Cow<'_, Path>, HomeDirNotFound> {
        crate::plain(self)
    }
}

/// Returns the path without special expansion characters.
///
/// Currently this only expands `~` to the user's home directory.
/// Symlinks are not converted.
pub fn plain(path: &Path) -> Result<Cow<'_, Path>, HomeDirNotFound> {
    if path.starts_with("~") {
        // Replace `~` with user's home directory.
        dirs::home_dir()
            .map(|mut path_normalized| {
                path_normalized.extend(path.into_iter().skip(1));
                Cow::Owned(path_normalized)
            })
            .ok_or(HomeDirNotFound)
    } else {
        Ok(Cow::Borrowed(path))
    }
}

/// Error when the user's home directory cannot be found.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HomeDirNotFound;

impl fmt::Display for HomeDirNotFound {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Failed to determine user's home directory.")
    }
}

impl std::error::Error for HomeDirNotFound {}

#[cfg(test)]
mod tests {
    use std::path::{Component, Path};

    use super::{HomeDirNotFound, PlainPathExt};

    #[test]
    fn expands_tilde() -> Result<(), HomeDirNotFound> {
        let path = Path::new("~/.ssh/config").plain()?;

        let mut components = path.components();
        assert_eq!(Some(Component::RootDir), components.next());

        #[cfg(osx)]
        assert_eq!(
            Some(Component::Normal(OsStr::new("Users"))),
            components.next()
        );

        #[cfg(linux)]
        assert_eq!(
            Some(Component::Normal(OsStr::new("home"))),
            components.next()
        );

        Ok(())
    }
}
