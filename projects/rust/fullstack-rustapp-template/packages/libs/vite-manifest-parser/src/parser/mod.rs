use serde::{Deserialize, Serialize};
use serde_json::Error;
use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use std::io::Read;
use std::path::PathBuf;
use std::str::FromStr;

#[cfg(test)]
pub(crate) mod test_assets;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(transparent)]
pub(crate) struct InternalManifest {
    bundles: HashMap<String, InternalManifestDetails>,
}

#[derive(Eq, Hash, PartialEq, Debug, Clone)]
struct ViteFile {
    filename: PathBuf,
    file_type: SupportedFileType,
}

impl ViteFile {
    pub fn parse(file: &PathBuf) -> Result<Self, String> {
        fn get_extension(file: &PathBuf) -> Result<SupportedFileType, String> {
            let extension = match file.extension() {
                Some(ext) => match ext.to_str() {
                    Some(ext) => ext,
                    None => {
                        return Err(format!(
                            "Couldn't convert extension '{:?}' in file '{:?}' to a str",
                            ext, file
                        ));
                    }
                },
                None => {
                    return Err(format!("File '{:?}' has no extension", file));
                }
            };
            match SupportedFileType::from_str(extension) {
                Ok(file_type) => Ok(file_type),
                Err(_) => Err(format!(
                    "File '{:?}' has unsupported extension '{:?}'",
                    file, extension
                )),
            }
        }

        fn get_filename(file: &PathBuf) -> Result<PathBuf, String> {
            let filename = match file.file_name() {
                Some(filename) => filename,
                None => {
                    return Err(format!("Path '{:?}' has no filename", file));
                }
            };
            Ok(PathBuf::from(filename))
        }

        Ok(Self {
            filename: get_filename(file)?,
            file_type: get_extension(file)?,
        })
    }
}

#[derive(Eq, Hash, PartialEq, Debug, Clone)]
pub enum SupportedFileType {
    Js,
    Css,
}

impl Display for SupportedFileType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SupportedFileType::Js => write!(f, "js"),
            SupportedFileType::Css => write!(f, "css"),
        }
    }
}

impl FromStr for SupportedFileType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "js" => Ok(SupportedFileType::Js),
            "css" => Ok(SupportedFileType::Css),
            _ => Err(()),
        }
    }
}

impl InternalManifest {
    fn get_all_files(&self) -> HashSet<ViteFile> {
        let mut set = HashSet::new();
        let manifests = self.bundles.values();
        for manifest in manifests {
            set.insert(ViteFile::parse(&PathBuf::from(&manifest.file)).unwrap());

            for css_file in manifest.css.as_ref().unwrap_or(&vec![]) {
                set.insert(ViteFile::parse(&PathBuf::from(css_file)).unwrap());
            }
        }

        set
    }

    pub fn get_typed_files(&self, file_type: SupportedFileType) -> HashSet<String> {
        let files = self.get_all_files();
        let mut set = HashSet::new();
        for file in files {
            if file.file_type == file_type {
                set.insert(file.filename.to_str().unwrap().to_string());
            }
        }
        set
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub(crate) struct InternalManifestDetails {
    css: Option<Vec<String>>,
    file: String,
    name: Option<String>,
    src: Option<String>,
    #[serde(rename = "isDynamicEntry")]
    is_dynamic_entry: Option<bool>,
    #[serde(rename = "isEntry")]
    is_entry: Option<bool>,
    imports: Option<Vec<String>>,
    #[serde(rename = "dynamicImports")]
    dynamic_imports: Option<Vec<String>>,
}

pub(crate) fn parse_manifest_json_str(manifest_json: &str) -> Result<InternalManifest, Error> {
    let manifest: InternalManifest = serde_json::from_str(manifest_json)?;
    Ok(manifest)
}

pub(crate) fn parse_manifest_json_buffer(
    manifest_json_buf: impl Read,
) -> Result<InternalManifest, Error> {
    let reader = std::io::BufReader::new(manifest_json_buf);
    let manifest: InternalManifest = serde_json::from_reader(reader)?;
    Ok(manifest)
}

#[cfg(test)]
mod tests {
    use test_assets::*;

    use super::*;

    mod proptests {
        use proptest::prelude::*;

        use super::*;

        mod regressions {
            use super::*;

            #[test]
            fn empty_manifest_doesnt_crash() {
                let manifest_json = "";
                let _parsed_manifest = parse_manifest_json_str(manifest_json);
                let _parsed_manifest = parse_manifest_json_buffer(manifest_json.as_bytes());
            }
        }

        proptest! {
            #[test]
            fn doesnt_crash(s in "\\PC*") {
                let _parsed_manifest = parse_manifest_json_str(&s);
            }

            #[test]
            fn doesnt_crash_buffered(s in "\\PC*") {
                let _parsed_manifest = parse_manifest_json_buffer(s.as_bytes());
            }
        }
    }

    #[test]
    fn can_parse_simple_manifest() {
        let parsed_manifest = parse_manifest_json_str(SIMPLE_MANIFEST_JSON_PRETTY).unwrap();
        assert_eq!(parsed_manifest.bundles, SIMPLE_MANIFEST.bundles);
    }

    #[test]
    fn can_parse_css_files() {
        let simple_manifest = parse_manifest_json_str(SIMPLE_MANIFEST_JSON_PRETTY).unwrap();
        assert_eq!(simple_manifest.bundles, SIMPLE_MANIFEST.bundles);
        let complex_manifest = parse_manifest_json_str(COMPLEX_MANIFEST_JSON_PRETTY).unwrap();
        assert_eq!(complex_manifest.bundles, COMPLEX_MANIFEST.bundles);

        let simple_css_parsed = simple_manifest.get_typed_files(SupportedFileType::Css);
        assert_eq!(
            simple_css_parsed,
            SIMPLE_MANIFEST_CSS.to_owned(),
            "List of simple css files didn't match test data"
        );

        let complex_css_parsed = complex_manifest.get_typed_files(SupportedFileType::Css);
        assert_eq!(
            complex_css_parsed,
            COMPLEX_MANIFEST_CSS.to_owned(),
            "List of complex css files didn't match test data"
        );
    }

    #[test]
    fn can_parse_js_files() {
        let simple_manifest = parse_manifest_json_str(SIMPLE_MANIFEST_JSON_PRETTY).unwrap();
        assert_eq!(simple_manifest.bundles, SIMPLE_MANIFEST.bundles);
        let complex_manifest = parse_manifest_json_str(COMPLEX_MANIFEST_JSON_PRETTY).unwrap();
        assert_eq!(complex_manifest.bundles, COMPLEX_MANIFEST.bundles);

        let simple_js_parsed = simple_manifest.get_typed_files(SupportedFileType::Js);
        assert_eq!(
            simple_js_parsed,
            SIMPLE_MANIFEST_JS.to_owned(),
            "List of simple js files didn't match test data"
        );

        let complex_js_parsed = complex_manifest.get_typed_files(SupportedFileType::Js);
        assert_eq!(
            complex_js_parsed,
            COMPLEX_MANIFEST_JS.to_owned(),
            "List of complex js files didn't match test data"
        );
    }

    #[test]
    fn can_parse_simple_buffered_manifest() {
        let file = std::fs::File::open(SIMPLE_MANIFEST_PATH.as_path()).unwrap();
        let parsed_manifest = parse_manifest_json_buffer(file).unwrap();
        assert_eq!(parsed_manifest.bundles, SIMPLE_MANIFEST.bundles);
    }

    #[test]
    fn can_parse_complex_manifest() {
        let parsed_manifest = parse_manifest_json_str(COMPLEX_MANIFEST_JSON_PRETTY).unwrap();
        for bundle in parsed_manifest.bundles.keys() {
            assert_eq!(
                parsed_manifest.bundles[bundle],
                COMPLEX_MANIFEST.bundles[bundle]
            );
        }
        assert_eq!(parsed_manifest.bundles, COMPLEX_MANIFEST.bundles);
    }

    #[test]
    fn can_parse_complex_buffered_manifest() {
        let file = std::fs::File::open(COMPLEX_MANIFEST_PATH.as_path()).unwrap();
        let parsed_manifest = parse_manifest_json_buffer(file).unwrap();
        for bundle in parsed_manifest.bundles.keys() {
            assert_eq!(
                parsed_manifest.bundles[bundle],
                COMPLEX_MANIFEST.bundles[bundle]
            );
        }
        assert_eq!(parsed_manifest.bundles, COMPLEX_MANIFEST.bundles);
    }

    #[test]
    fn serialize_and_deserialize_match_raw() {
        let simple_manifest: serde_json::Value =
            serde_json::from_str(SIMPLE_MANIFEST_JSON_PRETTY).unwrap();
        assert_eq!(
            SIMPLE_MANIFEST_JSON_PRETTY,
            serde_json::to_string_pretty(&simple_manifest).unwrap()
        );
        let complex_manifest: serde_json::Value =
            serde_json::from_str(COMPLEX_MANIFEST_JSON_PRETTY).unwrap();
        assert_eq!(
            COMPLEX_MANIFEST_JSON_PRETTY,
            serde_json::to_string_pretty(&complex_manifest).unwrap()
        );
    }
}
