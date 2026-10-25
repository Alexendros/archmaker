export type BuildIsoResult = { isoPath: string; sha256: string };

export async function buildIso(
  draft: unknown,
  onLog: (line: string) => void,
): Promise<BuildIsoResult> {
  const [{ invoke }, { listen }] = await Promise.all([
    import("@tauri-apps/api/core"),
    import("@tauri-apps/api/event"),
  ]);
  const unlisten = await listen<string>("build-iso-log", (event) => {
    onLog(event.payload);
  });
  try {
    return await invoke<BuildIsoResult>("build_iso", { input: { draft } });
  } finally {
    unlisten();
  }
}

export async function cancelBuildIso(): Promise<void> {
  const { invoke } = await import("@tauri-apps/api/core");
  await invoke("cancel_build_iso");
}
