mod parser;

use crate::parser::{parse_manifest_json_buffer, parse_manifest_json_str, SupportedFileType};
use std::collections::HashSet;

pub struct ViteReferences {
    pub js_files: HashSet<String>,
    pub css_files: HashSet<String>,
}

pub fn get_vite_references_str(
    manifest_str: &str,
) -> Result<ViteReferences, Box<dyn std::error::Error>> {
    let manifest = parse_manifest_json_str(manifest_str)?;
    Ok(ViteReferences {
        js_files: manifest.get_typed_files(SupportedFileType::Js),
        css_files: manifest.get_typed_files(SupportedFileType::Css),
    })
}

pub fn get_vite_references_buffer(
    manifest_buf: impl std::io::Read,
) -> Result<ViteReferences, Box<dyn std::error::Error>> {
    let manifest = parse_manifest_json_buffer(manifest_buf)?;
    Ok(ViteReferences {
        js_files: manifest.get_typed_files(SupportedFileType::Js),
        css_files: manifest.get_typed_files(SupportedFileType::Css),
    })
}
