//! CAT: providers verificados, ONLYOFFICE canónico, cardinalidad de navegadores.

use archmaker_domain::{
    onlyoffice_ids_equivalent, resolve_onlyoffice_canonical, BrowserPolicy, Catalog,
    ProviderStatus, DEFAULT_BROWSER_ID_NEW_PROFILE, ONLYOFFICE_CANONICAL_ID,
    ONLYOFFICE_HISTORICAL_ALIAS_ID,
};

fn desktop_catalog() -> Catalog {
    serde_json::from_str(include_str!(
        "../fixtures/local-catalog-mvp1.json"
    ))
    .expect("local-catalog-mvp1.json parses")
}

fn option<'a>(catalog: &'a Catalog, id: &str) -> &'a archmaker_domain::CatalogOption {
    for step in &catalog.steps {
        for section in &step.sections {
            if let Some(opt) = section.options.iter().find(|o| o.id == id) {
                return opt;
            }
        }
    }
    panic!("option not found: {id}");
}

#[test]
fn catalog_loads_with_zen_and_browser_many() {
    let catalog = desktop_catalog();
    assert_eq!(catalog.version, "1.0.0");
    let browser = catalog
        .steps
        .iter()
        .find(|s| s.id == "archmaker.apps.browser")
        .expect("browser step");
    assert_eq!(
        browser.sections[0].cardinality,
        archmaker_domain::Cardinality::Many
    );
    let zen = option(&catalog, "archmaker.browser.zen");
    let provider = zen.provider.as_ref().expect("zen provider");
    assert_eq!(provider.package_id, "zen-browser-bin");
    assert_eq!(provider.status, ProviderStatus::Verified);
    assert_eq!(zen.browser_base.as_deref(), Some("Firefox"));
    assert_eq!(zen.rendering_engine.as_deref(), Some("Gecko"));
}

#[test]
fn onlyoffice_canonical_alias_not_duplicate_install() {
    let catalog = desktop_catalog();
    let canonical = option(&catalog, ONLYOFFICE_CANONICAL_ID);
    let aliases = canonical.aliases.as_ref().expect("canonical aliases");
    assert_eq!(aliases.as_slice(), [ONLYOFFICE_HISTORICAL_ALIAS_ID]);
    let alias = option(&catalog, ONLYOFFICE_HISTORICAL_ALIAS_ID);
    assert_eq!(
        alias.canonical_id.as_deref(),
        Some(ONLYOFFICE_CANONICAL_ID)
    );
    assert!(onlyoffice_ids_equivalent(
        ONLYOFFICE_CANONICAL_ID,
        ONLYOFFICE_HISTORICAL_ALIAS_ID
    ));
    assert_eq!(
        resolve_onlyoffice_canonical(ONLYOFFICE_HISTORICAL_ALIAS_ID),
        Some(ONLYOFFICE_CANONICAL_ID)
    );
}

#[test]
fn libreoffice_not_fixed_for_new_onlyoffice_profiles() {
    let catalog = desktop_catalog();
    let lo = option(&catalog, "archmaker.pkg.libreoffice-fresh");
    assert_ne!(lo.fixed, Some(true));
    let oo = option(&catalog, ONLYOFFICE_CANONICAL_ID);
    assert!(oo.provider.is_some());
}

#[test]
fn apexshot_unverified_cat01_open() {
    let catalog = desktop_catalog();
    let apex = option(&catalog, "archmaker.app.apexshot");
    let provider = apex.provider.as_ref().expect("apex provider");
    assert_eq!(provider.status, ProviderStatus::Unverified);
    assert_eq!(provider.package_id, "apexshot");
    let flameshot = option(&catalog, "archmaker.pkg.flameshot");
    assert_eq!(
        flameshot.provider.as_ref().unwrap().status,
        ProviderStatus::Verified
    );
}

#[test]
fn new_profile_browser_policy_zen_default() {
    let policy = BrowserPolicy::new_profile_zen();
    assert!(policy.validate().is_ok());
    assert_eq!(
        policy.default_browser_id.as_deref(),
        Some(DEFAULT_BROWSER_ID_NEW_PROFILE)
    );
    let empty = BrowserPolicy::none();
    assert!(empty.validate().is_ok());
    assert!(empty.default_browser_id.is_none());
}

#[test]
fn verified_gaps_closed_for_lts_des_editors() {
    let catalog = desktop_catalog();
    for id in [
        "archmaker.kernel.linux-lts",
        "archmaker.compositor.kde",
        "archmaker.compositor.cinnamon",
        "archmaker.compositor.xfce",
        "archmaker.compositor.budgie",
        "archmaker.compositor.i3",
        "archmaker.app.zed",
        "archmaker.app.vscode",
        "archmaker.app.kitty",
        "archmaker.app.ghostty",
    ] {
        let opt = option(&catalog, id);
        let provider = opt
            .provider
            .as_ref()
            .unwrap_or_else(|| panic!("missing provider for {id}"));
        assert_eq!(
            provider.status,
            ProviderStatus::Verified,
            "{id} must be verified"
        );
    }
}

#[test]
fn grok_requests_remain_unverified() {
    let catalog = desktop_catalog();
    for id in ["archmaker.app.grokbot", "archmaker.app.grokbuild"] {
        let opt = option(&catalog, id);
        assert_eq!(
            opt.provider.as_ref().unwrap().status,
            ProviderStatus::Unverified
        );
    }
}
