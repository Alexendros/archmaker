//! Límites de política de planificación (versionados; sin IO).

/// Política de referencia ArchMaker (UEFI64→GPT→ESP FAT32 `/efi` UKI→systemd-boot).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StoragePlanningLimits {
    /// ESP objetivo (política ArchMaker, no regla universal).
    pub esp_size_bytes: u64,
    /// Mínimo absoluto ESP para UKI (política).
    pub esp_min_bytes: u64,
    /// Reserva GPT + márgenes (bytes).
    pub gpt_reserve_bytes: u64,
    /// Reserva cabecera LUKS2 estimada (bytes) cuando la raíz va cifrada.
    pub luks2_header_reserve_bytes: u64,
    /// Sector lógico por defecto solo si se declara explícitamente; no se inventa en observación.
    pub default_declared_logical_sector: u64,
}

impl StoragePlanningLimits {
    pub const fn reference() -> Self {
        Self {
            esp_size_bytes: 2 * 1024 * 1024 * 1024,
            esp_min_bytes: 512 * 1024 * 1024,
            gpt_reserve_bytes: 34 * 512 + 16 * 1024 * 1024,
            luks2_header_reserve_bytes: 16 * 1024 * 1024,
            default_declared_logical_sector: 512,
        }
    }
}
