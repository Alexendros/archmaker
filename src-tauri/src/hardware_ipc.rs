//! IPC tipado de observación/planificación (solo lectura). A6 fuera.
//! Handles opacos; sin path/shell desde el frontend.

use std::sync::Mutex;

use archmaker_core::CoreError;
use archmaker_domain::StoragePlan;
use archmaker_hardware::{
    propose_storage_plan as hw_propose_storage_plan,
    validate_storage_plan as hw_validate_storage_plan, ObservationOrigin, PlanRequest, PlanResult,
    SessionDeviceRegistry, StorageSnapshot, TargetBinding,
};
use serde::{Deserialize, Serialize};
use tauri::State;

#[derive(Default)]
pub struct HardwareSession {
    pub registry: SessionDeviceRegistry,
    pub last_snapshot: Option<StorageSnapshot>,
}

pub struct HardwareState(pub Mutex<HardwareSession>);

fn core_err(detail: &str) -> serde_json::Value {
    serde_json::to_value(CoreError::schema_invalid(&format!(
        "archmaker-tauri.hardware: {detail}"
    )))
    .expect("CoreError serializable")
}

fn proto_denied(detail: &str) -> serde_json::Value {
    serde_json::to_value(CoreError::proto_denied(detail)).expect("serializable")
}

fn gate_main(window: &tauri::Window) -> Result<(), serde_json::Value> {
    if window.label() != "main" {
        return Err(proto_denied(&format!(
            "archmaker-invoke.{}.hardware",
            window.label()
        )));
    }
    Ok(())
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct InspectHostInput {
    #[serde(default)]
    pub confirm_as_target: bool,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct InspectStorageInput {
    #[serde(default)]
    pub confirm_as_target: bool,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InspectStorageOutput {
    pub snapshot: StorageSnapshot,
    /// Tokens opacos emitidos; el frontend no recibe /dev ni serial.
    pub handle_count: usize,
}

#[cfg(all(feature = "hardware-probe", target_os = "linux"))]
fn do_inspect_host(confirm: bool) -> Result<serde_json::Value, serde_json::Value> {
    use archmaker_hardware::probe::inspect_host_hardware;
    let binding = if confirm {
        TargetBinding::CurrentHostConfirmed
    } else {
        TargetBinding::Unbound
    };
    let report = inspect_host_hardware(binding);
    serde_json::to_value(report).map_err(|e| core_err(&e.to_string()))
}

#[cfg(not(all(feature = "hardware-probe", target_os = "linux")))]
fn do_inspect_host(_confirm: bool) -> Result<serde_json::Value, serde_json::Value> {
    Err(core_err(
        "inspect_host_hardware no disponible fuera de Linux nativo (web/WASM: manual o importación)",
    ))
}

#[cfg(all(feature = "hardware-probe", target_os = "linux"))]
fn do_inspect_storage(
    state: &HardwareState,
    confirm: bool,
) -> Result<InspectStorageOutput, serde_json::Value> {
    use archmaker_hardware::probe::inspect_storage_devices;
    let binding = if confirm {
        TargetBinding::CurrentHostConfirmed
    } else {
        TargetBinding::Unbound
    };
    let (snap, registry) =
        inspect_storage_devices(binding).map_err(|e| core_err(&e.to_string()))?;
    let handle_count = registry.by_token.len();
    let mut guard = state
        .0
        .lock()
        .map_err(|_| core_err("hardware session lock"))?;
    guard.registry = registry;
    guard.last_snapshot = Some(snap.clone());
    Ok(InspectStorageOutput {
        snapshot: snap,
        handle_count,
    })
}

#[cfg(not(all(feature = "hardware-probe", target_os = "linux")))]
fn do_inspect_storage(
    _state: &HardwareState,
    _confirm: bool,
) -> Result<InspectStorageOutput, serde_json::Value> {
    Err(core_err(
        "inspect_storage_devices no disponible fuera de Linux nativo",
    ))
}

#[tauri::command]
pub fn inspect_host_hardware(
    window: tauri::Window,
    input: serde_json::Value,
) -> Result<serde_json::Value, serde_json::Value> {
    gate_main(&window)?;
    let typed: InspectHostInput =
        serde_json::from_value(input).map_err(|e| core_err(&e.to_string()))?;
    do_inspect_host(typed.confirm_as_target)
}

#[tauri::command]
pub fn inspect_storage_devices(
    window: tauri::Window,
    state: State<'_, HardwareState>,
    input: serde_json::Value,
) -> Result<serde_json::Value, serde_json::Value> {
    gate_main(&window)?;
    let typed: InspectStorageInput =
        serde_json::from_value(input).map_err(|e| core_err(&e.to_string()))?;
    let out = do_inspect_storage(&state, typed.confirm_as_target)?;
    serde_json::to_value(out).map_err(|e| core_err(&e.to_string()))
}

#[tauri::command]
pub fn refresh_storage_snapshot(
    window: tauri::Window,
    state: State<'_, HardwareState>,
    input: serde_json::Value,
) -> Result<serde_json::Value, serde_json::Value> {
    gate_main(&window)?;
    let typed: InspectStorageInput =
        serde_json::from_value(input).map_err(|e| core_err(&e.to_string()))?;
    #[cfg(all(feature = "hardware-probe", target_os = "linux"))]
    {
        use archmaker_hardware::probe::refresh_storage_snapshot as refresh;
        let binding = if typed.confirm_as_target {
            TargetBinding::CurrentHostConfirmed
        } else {
            TargetBinding::Unbound
        };
        let mut guard = state
            .0
            .lock()
            .map_err(|_| core_err("hardware session lock"))?;
        let previous = guard
            .last_snapshot
            .clone()
            .ok_or_else(|| core_err("no hay snapshot previo"))?;
        let (snap, registry) = refresh(&previous, binding).map_err(|e| core_err(&e.to_string()))?;
        let handle_count = registry.by_token.len();
        guard.registry = registry;
        guard.last_snapshot = Some(snap.clone());
        let out = InspectStorageOutput {
            snapshot: snap,
            handle_count,
        };
        return serde_json::to_value(out).map_err(|e| core_err(&e.to_string()));
    }
    #[cfg(not(all(feature = "hardware-probe", target_os = "linux")))]
    {
        let _ = (state, typed);
        Err(core_err("refresh_storage_snapshot no disponible"))
    }
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ProposePlanInput {
    pub request: PlanRequest,
}

#[tauri::command]
pub fn propose_storage_plan(
    window: tauri::Window,
    input: serde_json::Value,
) -> Result<serde_json::Value, serde_json::Value> {
    gate_main(&window)?;
    // Rechazar path/shell embebidos en request (defensa en profundidad).
    let raw = input.to_string();
    if raw.contains("/dev/") || raw.contains("sh -c") || raw.contains("pacstrap") {
        return Err(proto_denied(
            "archmaker-tauri.hardware: path/shell denegado",
        ));
    }
    let typed: ProposePlanInput =
        serde_json::from_value(input).map_err(|e| core_err(&e.to_string()))?;
    let result: PlanResult = hw_propose_storage_plan(&typed.request);
    serde_json::to_value(result).map_err(|e| core_err(&e.to_string()))
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ValidatePlanInput {
    pub request: PlanRequest,
    pub existing: StoragePlan,
}

#[tauri::command]
pub fn validate_storage_plan(
    window: tauri::Window,
    input: serde_json::Value,
) -> Result<serde_json::Value, serde_json::Value> {
    gate_main(&window)?;
    let raw = input.to_string();
    if raw.contains("/dev/") || raw.contains("sh -c") {
        return Err(proto_denied(
            "archmaker-tauri.hardware: path/shell denegado",
        ));
    }
    let typed: ValidatePlanInput =
        serde_json::from_value(input).map_err(|e| core_err(&e.to_string()))?;
    let result = hw_validate_storage_plan(&typed.request, &typed.existing);
    serde_json::to_value(result).map_err(|e| core_err(&e.to_string()))
}

/// Importa un snapshot ya redactado (web/manual). Sin IO de host.
#[tauri::command]
pub fn import_storage_snapshot(
    window: tauri::Window,
    state: State<'_, HardwareState>,
    input: serde_json::Value,
) -> Result<serde_json::Value, serde_json::Value> {
    gate_main(&window)?;
    #[derive(Deserialize)]
    #[serde(deny_unknown_fields, rename_all = "camelCase")]
    struct ImportIn {
        snapshot: StorageSnapshot,
    }
    let typed: ImportIn = serde_json::from_value(input).map_err(|e| core_err(&e.to_string()))?;
    if typed.snapshot.origin == ObservationOrigin::LocalProbe {
        // En web no debe fingirse sonda local.
        return Err(core_err(
            "import_storage_snapshot rechaza origin local-probe fingido",
        ));
    }
    let mut guard = state
        .0
        .lock()
        .map_err(|_| core_err("hardware session lock"))?;
    // Sin identidades privadas en importación.
    guard.registry = SessionDeviceRegistry::default();
    guard.last_snapshot = Some(typed.snapshot.clone());
    serde_json::to_value(typed.snapshot).map_err(|e| core_err(&e.to_string()))
}
