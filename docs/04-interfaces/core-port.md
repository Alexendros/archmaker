# CorePort

```ts
export interface CorePort {
  productInfo(): Promise<Result<ProductInfo, CoreError>>;
  capabilities(): Promise<Result<CapabilitySet, CoreError>>;
  loadCatalog(input: LoadCatalogInput): Promise<Result<CatalogView, CoreError>>;
  createDraft(input: CreateDraftInput): Promise<Result<Draft, CoreError>>;
  importDraft(input: ImportDraftInput): Promise<Result<ImportResult, CoreError>>;
  planMigration(input: PlanMigrationInput): Promise<Result<MigrationPlan, CoreError>>;
  applyMigration(input: ApplyMigrationInput): Promise<Result<MigrationResult, CoreError>>;
  applyPreset(input: ApplyPresetInput): Promise<Result<ResolveResult, CoreError>>;
  resolveDraft(input: ResolveDraftInput): Promise<Result<ResolveResult, CoreError>>;
  validateDraft(input: ValidateDraftInput): Promise<Result<ValidationResult, CoreError>>;
  buildManifest(input: BuildManifestInput): Promise<Result<ManifestResult, CoreError>>;
  compareDrafts(input: CompareDraftsInput): Promise<Result<DraftDiff, CoreError>>;
  listExportTargets(): Promise<Result<ExportTarget[], CoreError>>;
  checkTarget(input: CheckTargetInput): Promise<Result<TargetCompatibility, CoreError>>;
  exportArtifact(input: ExportArtifactInput): Promise<Result<ExportResult, CoreError>>;
}
```

## Invariantes

- Operaciones puras son idempotentes.
- Todo input tiene límites.
- Errores son DTO tipados.
- Las operaciones largas aceptan `requestId` y emiten eventos correlacionados.
- Ningún método acepta shell, comandos o rutas arbitrarias.
