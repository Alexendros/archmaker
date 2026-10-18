use std::fs;
use std::path::PathBuf;
use std::process::Command;

use archmaker_constitute::{
    fetch_packages, parse_live_profile, request_live_image, run_args, CODE_DEFERRED,
    CODE_IMAGE_ABSENT, CODE_IMAGE_REFUSED, CODE_OFFLINE, CODE_OUTSIDE, CODE_SCRIPT,
};
use sha2::{Digest, Sha256};

fn profile(install: &str, deferred: &str) -> String {
    format!(r#"{{"files":{{"packages.x86_64":"{install}"}},"deferredFetch":[{deferred}]}}"#)
}

fn temp_dir(label: &str) -> PathBuf {
    let path = std::env::temp_dir().join(format!(
        "archmaker-constitute-{label}-{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&path);
    fs::create_dir_all(&path).expect("temp");
    path
}

#[test]
fn fixture_fetch_stays_inside_profile_and_cites_digest() {
    let root = temp_dir("ok");
    let fixture = root.join("fixture");
    let out = root.join("cache");
    fs::create_dir_all(&fixture).unwrap();
    let bytes = b"linux-bytes";
    fs::write(fixture.join("linux"), bytes).unwrap();
    fs::write(fixture.join("gnome-shell"), b"gnome-bytes").unwrap();
    let digest = hex::encode(Sha256::digest(bytes));
    fs::write(fixture.join("linux.sha256"), format!("{digest}\n")).unwrap();

    let parsed = parse_live_profile(profile("linux\\ngnome-shell\\n", "").as_bytes()).unwrap();
    let outcome = fetch_packages(&parsed, &[], &fixture, &out).unwrap();
    assert_eq!(outcome.journal.len(), 2);
    assert!(outcome.journal.iter().all(|entry| {
        parsed.install.contains(&entry.name) && fs::read(out.join(&entry.name)).is_ok()
    }));
    let linux = outcome
        .journal
        .iter()
        .find(|entry| entry.name == "linux")
        .unwrap();
    assert_eq!(linux.sha256.as_deref(), Some(digest.as_str()));
    let gnome = outcome
        .journal
        .iter()
        .find(|entry| entry.name == "gnome-shell")
        .unwrap();
    assert!(gnome.sha256.is_none());
    assert!(!out.join("airootfs").exists());
    let _ = fs::remove_dir_all(&root);
}

#[test]
fn missing_fixture_is_offline_and_writes_nothing() {
    let root = temp_dir("offline");
    let out = root.join("cache");
    let parsed = parse_live_profile(profile("linux\\n", "").as_bytes()).unwrap();
    let error = fetch_packages(&parsed, &[], &root.join("no-fixture"), &out).unwrap_err();
    assert_eq!(error.code, CODE_OFFLINE);
    assert!(!out.exists());
    let _ = fs::remove_dir_all(&root);
}

#[test]
fn name_outside_profile_is_rejected() {
    let root = temp_dir("outside");
    let fixture = root.join("fixture");
    let out = root.join("cache");
    fs::create_dir_all(&fixture).unwrap();
    fs::write(fixture.join("linux"), b"linux").unwrap();
    fs::write(fixture.join("bash"), b"no").unwrap();
    let parsed = parse_live_profile(profile("linux\\n", "").as_bytes()).unwrap();
    let error = fetch_packages(&parsed, &["bash".to_string()], &fixture, &out).unwrap_err();
    assert_eq!(error.code, CODE_OUTSIDE);
    assert_eq!(error.name.as_deref(), Some("bash"));
    assert!(!out.join("bash").exists());
    let _ = fs::remove_dir_all(&root);
}

#[test]
fn deferred_bin_is_not_copied_into_output() {
    let root = temp_dir("deferred");
    let fixture = root.join("fixture");
    let out = root.join("cache");
    fs::create_dir_all(&fixture).unwrap();
    fs::write(fixture.join("linux"), b"linux").unwrap();
    fs::write(fixture.join("onlyoffice-bin"), b"blob").unwrap();
    let parsed = parse_live_profile(profile("linux\\n", "\"onlyoffice-bin\"").as_bytes()).unwrap();
    let error =
        fetch_packages(&parsed, &["onlyoffice-bin".to_string()], &fixture, &out).unwrap_err();
    assert_eq!(error.code, CODE_DEFERRED);
    assert!(!out.join("onlyoffice-bin").exists());
    assert!(!root.join("airootfs").exists());
    let _ = fs::remove_dir_all(&root);
}

#[test]
fn output_named_airootfs_is_refused() {
    let root = temp_dir("airoot");
    let fixture = root.join("fixture");
    let out = root.join("airootfs");
    fs::create_dir_all(&fixture).unwrap();
    fs::write(fixture.join("linux"), b"linux").unwrap();
    let parsed = parse_live_profile(profile("linux\\n", "").as_bytes()).unwrap();
    let error = fetch_packages(&parsed, &[], &fixture, &out).unwrap_err();
    assert_eq!(error.code, "constitution-unsafe-output");
    assert!(!out.join("linux").exists());
    let _ = fs::remove_dir_all(&root);
}

#[test]
fn a_profile_line_that_is_not_a_package_name_is_rejected() {
    let bad = r#"{"files":{"packages.x86_64":"linux;id\n"}}"#;
    let error = parse_live_profile(bad.as_bytes()).unwrap_err();
    assert_eq!(error.code, CODE_SCRIPT);
}

#[test]
fn argv_rejects_a_script_and_the_bin_fetches_the_fixture() {
    let rejected = run_args(&["sh".into(), "-c".into(), "true".into()]).unwrap_err();
    assert_eq!(rejected.code, CODE_SCRIPT);
    let rejected = run_args(&["fetch".into(), "--script".into(), "true".into()]).unwrap_err();
    assert_eq!(rejected.code, CODE_SCRIPT);

    let root = temp_dir("bin");
    let fixture = root.join("fixture");
    let out = root.join("cache");
    let profile_path = root.join("profile.json");
    fs::create_dir_all(&fixture).unwrap();
    fs::write(fixture.join("linux"), b"linux-bytes").unwrap();
    let digest = hex::encode(Sha256::digest(b"linux-bytes"));
    fs::write(fixture.join("linux.sha256"), &digest).unwrap();
    fs::write(&profile_path, profile("linux\\n", "")).unwrap();

    let bin = env!("CARGO_BIN_EXE_archmaker-constitute");
    let output = Command::new(bin)
        .arg("fetch")
        .arg("--profile")
        .arg(&profile_path)
        .arg("--fixture")
        .arg(&fixture)
        .arg("--out")
        .arg(&out)
        .output()
        .expect("proceso");
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stdout)
    );
    let body: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(body["ok"], true);
    assert_eq!(body["journal"][0]["name"], "linux");
    assert_eq!(body["journal"][0]["sha256"], digest);
    assert_eq!(fs::read(out.join("linux")).unwrap(), b"linux-bytes");

    let offline = Command::new(bin)
        .arg("fetch")
        .arg("--profile")
        .arg(&profile_path)
        .arg("--fixture")
        .arg(root.join("sin-red"))
        .arg("--out")
        .arg(root.join("vacio"))
        .output()
        .expect("proceso");
    assert!(!offline.status.success());
    let err: serde_json::Value = serde_json::from_slice(&offline.stdout).unwrap();
    assert_eq!(err["code"], CODE_OFFLINE);
    assert!(!root.join("vacio").exists());
    let _ = fs::remove_dir_all(&root);
}

#[test]
fn image_request_does_not_create_an_iso_or_accept_a_script() {
    let empty = temp_dir("sin-mkarchiso");
    let absent = request_live_image(empty.to_str().expect("utf8"));
    assert_eq!(absent.code, CODE_IMAGE_ABSENT);
    assert!(fs::read_dir(&empty).unwrap().next().is_none());

    let with_tool = temp_dir("con-nombre");
    fs::write(with_tool.join("mkarchiso"), b"").unwrap();
    let refused = request_live_image(with_tool.to_str().expect("utf8"));
    assert_eq!(refused.code, CODE_IMAGE_REFUSED);
    assert!(fs::read_dir(&with_tool)
        .unwrap()
        .all(|entry| { entry.unwrap().file_name() == "mkarchiso" }));

    let script = run_args(&["image".into(), "--script".into()]).expect_err("script");
    assert_eq!(script.code, CODE_SCRIPT);
    let host = run_args(&["image".into()]).expect_err("sin corrida");
    assert!(host.code == CODE_IMAGE_ABSENT || host.code == CODE_IMAGE_REFUSED);
    let _ = fs::remove_dir_all(empty);
    let _ = fs::remove_dir_all(with_tool);
}
