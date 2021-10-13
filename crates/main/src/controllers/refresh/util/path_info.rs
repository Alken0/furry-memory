use regex::Regex;
use std::{
    io::{Error, ErrorKind},
    path::{Path, PathBuf},
};

#[derive(Clone)]
pub struct PathInfo {
    path: PathBuf,
}

impl PathInfo {
    pub fn new(path: &Path) -> Self {
        Self {
            path: path.to_owned(),
        }
    }

    /// returns everything between last "/" and last "." => "/path/name.adsf.extension" -> "name.asdf"
    pub fn name(&self) -> String {
        let path = self.path();
        let regex_name = Regex::new(r"([^/]+$)").unwrap();
        let regex_extension = Regex::new(r"(\.[^.]+$)").unwrap();

        let regex_name_match = match regex_name.captures(&path) {
            Some(s) => s.get(0),
            None => return String::from(""),
        };

        let name_with_extension = match regex_name_match {
            Some(s) => s.as_str(),
            None => return String::from(""),
        };

        let regex_extension_match = match regex_extension.captures(&path) {
            Some(s) => s.get(0),
            None => return name_with_extension.to_owned(),
        };

        return match regex_extension_match {
            Some(s) => name_with_extension.replace(s.as_str(), "").to_owned(),
            None => return name_with_extension.to_owned(),
        };
    }

    pub fn mime(&self) -> Result<String, Error> {
        mime_guess::from_path(&self.path)
            .first()
            .ok_or_else(|| {
                Error::new(
                    ErrorKind::Unsupported,
                    format!("could not guess MimeType (path: {})", self.path()),
                )
            })
            .map(|m| m.to_string())
    }

    pub fn path(&self) -> String {
        self.path
            .as_os_str()
            .to_owned()
            .to_string_lossy()
            .to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[rocket::async_test]
    async fn path() {
        assert_eq!(
            PathInfo::new(&PathBuf::from("/path/name.extension")).path(),
            String::from("/path/name.extension")
        );
    }

    mod name {
        use super::*;

        #[rocket::async_test]
        async fn normal_path() {
            assert_eq!(
                PathInfo::new(&PathBuf::from("/path/name.extension")).name(),
                String::from("name")
            );
        }

        #[rocket::async_test]
        async fn no_extension() {
            assert_eq!(
                PathInfo::new(&PathBuf::from("/path/name")).name(),
                String::from("name")
            );
        }

        #[rocket::async_test]
        async fn no_name_but_extension() {
            assert_eq!(
                PathInfo::new(&PathBuf::from("/path/.extension")).name(),
                String::from("")
            );
        }

        #[rocket::async_test]
        async fn no_prepath() {
            assert_eq!(
                PathInfo::new(&PathBuf::from("name.extension")).name(),
                String::from("name")
            );
        }

        #[rocket::async_test]
        async fn no_name_and_no_extension() {
            assert_eq!(
                PathInfo::new(&PathBuf::from("/path/")).name(),
                String::from("")
            );
        }

        #[rocket::async_test]
        async fn name_with_dot() {
            assert_eq!(
                PathInfo::new(&PathBuf::from("/path/name.asdf.extension")).name(),
                String::from("name.asdf")
            );
        }
    }
}
