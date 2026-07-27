use anyhow::{anyhow, bail, ensure, Context, Result};
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use std::fs;
use std::path::{Component, Path, PathBuf};

pub const EXPECTED_PACKAGE_NAME: &str = "OpenAI.Codex";
pub const EXPECTED_PACKAGE_PUBLISHER: &str = "CN=50BDFD77-8903-4850-9FFE-6E8522F64D5B";
pub const EXPECTED_ARCHITECTURE: &str = "x64";
pub const PORTABLE_MANIFEST_FILE: &str = ".appx-manifest.xml";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackageMetadata {
    pub version: String,
    pub executable: PathBuf,
}

pub fn parse_and_validate_manifest(xml: &str) -> Result<PackageMetadata> {
    let mut reader = Reader::from_str(xml);
    reader.config_mut().trim_text(true);
    let mut buffer = Vec::new();
    let mut identity: Option<Identity> = None;
    let mut executable: Option<String> = None;

    loop {
        match reader.read_event_into(&mut buffer) {
            Ok(Event::Start(element)) | Ok(Event::Empty(element)) => {
                match local_name(element.name().as_ref()) {
                    b"Identity" if identity.is_none() => identity = Some(read_identity(&element)?),
                    b"Application" if executable.is_none() => {
                        executable = attribute(&element, b"Executable")
                    }
                    _ => {}
                }
            }
            Ok(Event::Eof) => break,
            Err(error) => bail!("invalid AppxManifest.xml: {error}"),
            _ => {}
        }
        buffer.clear();
    }

    let identity = identity.ok_or_else(|| anyhow!("manifest has no package identity"))?;
    ensure!(
        identity.name == EXPECTED_PACKAGE_NAME,
        "unexpected package identity: {}",
        identity.name
    );
    ensure!(
        identity.publisher == EXPECTED_PACKAGE_PUBLISHER,
        "unexpected package publisher: {}",
        identity.publisher
    );
    ensure!(
        identity
            .architecture
            .eq_ignore_ascii_case(EXPECTED_ARCHITECTURE),
        "unsupported package architecture: {}",
        identity.architecture
    );

    let executable = executable.ok_or_else(|| anyhow!("manifest has no application executable"))?;
    let executable = flattened_app_path(&executable)?;

    Ok(PackageMetadata {
        version: identity.version,
        executable,
    })
}

pub fn resolve_installed_executable(version_dir: &Path) -> Result<PathBuf> {
    let manifest_path = version_dir.join(PORTABLE_MANIFEST_FILE);
    let manifest = fs::read_to_string(&manifest_path)
        .with_context(|| format!("reading {}", manifest_path.display()))?;
    let package = parse_and_validate_manifest(&manifest)?;
    let executable = version_dir.join(package.executable);
    ensure!(
        executable.is_file(),
        "declared application executable does not exist: {}",
        executable.display()
    );
    Ok(executable)
}

#[derive(Debug)]
struct Identity {
    name: String,
    publisher: String,
    architecture: String,
    version: String,
}

fn read_identity(element: &BytesStart<'_>) -> Result<Identity> {
    Ok(Identity {
        name: required_attribute(element, b"Name")?,
        publisher: required_attribute(element, b"Publisher")?,
        architecture: required_attribute(element, b"ProcessorArchitecture")?,
        version: required_attribute(element, b"Version")?,
    })
}

fn required_attribute(element: &BytesStart<'_>, name: &[u8]) -> Result<String> {
    attribute(element, name).ok_or_else(|| {
        anyhow!(
            "manifest element is missing {}",
            String::from_utf8_lossy(name)
        )
    })
}

fn attribute(element: &BytesStart<'_>, name: &[u8]) -> Option<String> {
    element.attributes().flatten().find_map(|attribute| {
        (local_name(attribute.key.as_ref()) == name)
            .then(|| {
                attribute
                    .unescape_value()
                    .ok()
                    .map(|value| value.into_owned())
            })
            .flatten()
    })
}

fn flattened_app_path(raw: &str) -> Result<PathBuf> {
    let normalized = raw.replace('\\', "/");
    let relative = normalized
        .strip_prefix("app/")
        .ok_or_else(|| anyhow!("application executable is outside the app payload: {raw}"))?;
    ensure!(
        !relative.is_empty(),
        "application executable is outside the app payload: {raw}"
    );

    let path = Path::new(relative);
    let mut safe = PathBuf::new();
    for component in path.components() {
        match component {
            Component::Normal(value) => safe.push(value),
            Component::CurDir => {}
            Component::ParentDir | Component::Prefix(_) | Component::RootDir => {
                bail!("application executable is outside the app payload: {raw}")
            }
        }
    }
    ensure!(
        !safe.as_os_str().is_empty(),
        "application executable is outside the app payload: {raw}"
    );
    Ok(safe)
}

fn local_name(name: &[u8]) -> &[u8] {
    match name.iter().rposition(|byte| *byte == b':') {
        Some(index) => &name[index + 1..],
        None => name,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;
    use std::time::{SystemTime, UNIX_EPOCH};

    const CURRENT_MANIFEST: &str = r#"<?xml version="1.0" encoding="utf-8"?>
<Package xmlns="http://schemas.microsoft.com/appx/manifest/foundation/windows10">
  <Identity Name="OpenAI.Codex" ProcessorArchitecture="x64" Version="26.721.4979.0" Publisher="CN=50BDFD77-8903-4850-9FFE-6E8522F64D5B" />
  <Applications>
    <Application Id="App" Executable="app/ChatGPT.exe" EntryPoint="Windows.FullTrustApplication" />
  </Applications>
</Package>"#;

    #[test]
    fn current_chatgpt_manifest_resolves_flattened_executable() {
        let package = parse_and_validate_manifest(CURRENT_MANIFEST).unwrap();

        assert_eq!(package.version, "26.721.4979.0");
        assert_eq!(package.executable, Path::new("ChatGPT.exe"));
    }

    #[test]
    fn wrong_package_identity_is_rejected() {
        let manifest = CURRENT_MANIFEST.replace("OpenAI.Codex", "Example.Imposter");

        let error = parse_and_validate_manifest(&manifest).unwrap_err();

        assert!(error.to_string().contains("unexpected package identity"));
    }

    #[test]
    fn wrong_publisher_is_rejected() {
        let manifest =
            CURRENT_MANIFEST.replace("CN=50BDFD77-8903-4850-9FFE-6E8522F64D5B", "CN=Example");

        let error = parse_and_validate_manifest(&manifest).unwrap_err();

        assert!(error.to_string().contains("unexpected package publisher"));
    }

    #[test]
    fn wrong_architecture_is_rejected() {
        let manifest = CURRENT_MANIFEST.replace(
            "ProcessorArchitecture=\"x64\"",
            "ProcessorArchitecture=\"arm64\"",
        );

        let error = parse_and_validate_manifest(&manifest).unwrap_err();

        assert!(error
            .to_string()
            .contains("unsupported package architecture"));
    }

    #[test]
    fn executable_outside_app_payload_is_rejected() {
        let manifest = CURRENT_MANIFEST.replace("app/ChatGPT.exe", "../ChatGPT.exe");

        let error = parse_and_validate_manifest(&manifest).unwrap_err();

        assert!(error.to_string().contains("outside the app payload"));
    }

    #[test]
    fn resolves_installed_executable_from_saved_manifest() {
        let sandbox = TestSandbox::new();
        fs::write(sandbox.root.join(PORTABLE_MANIFEST_FILE), CURRENT_MANIFEST).unwrap();
        fs::write(sandbox.root.join("ChatGPT.exe"), b"fixture executable").unwrap();

        let executable = resolve_installed_executable(&sandbox.root).unwrap();

        assert_eq!(executable, sandbox.root.join("ChatGPT.exe"));
    }

    #[test]
    fn rejects_installed_version_with_missing_executable() {
        let sandbox = TestSandbox::new();
        fs::write(sandbox.root.join(PORTABLE_MANIFEST_FILE), CURRENT_MANIFEST).unwrap();

        let error = resolve_installed_executable(&sandbox.root).unwrap_err();

        assert!(error.to_string().contains("does not exist"));
    }

    struct TestSandbox {
        root: PathBuf,
    }

    impl TestSandbox {
        fn new() -> Self {
            let unique = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos();
            let root = std::env::temp_dir().join(format!(
                "chatgpt-portable-package-test-{}-{unique}",
                std::process::id()
            ));
            fs::create_dir_all(&root).unwrap();
            Self { root }
        }
    }

    impl Drop for TestSandbox {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.root);
        }
    }
}
