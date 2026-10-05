import React, { useState, useEffect, useMemo, useRef } from 'react';

// === ARCHMAKER v5.1 - Single Compositor + LibPacks + Hardened ===
const INSTANCE = {
  archmaker: { name: "ArchMaker", version: "5.1.0", date: "2026-10-05", arch: ["x86_64", "aarch64"], deploy: "local", iso: { name: "archmaker-forge" } },
  configurator: {
    type: "multi",
    steps: [
      {
        id: "base",
        name: "Sistema Base",
        icon: "🧰",
        mode: "grouped",
        sections: [
          {
            id: "kernel", name: "Kernel", single: true, packages: [
              { id: "linux", name: "Linux", description: "7.1.2 última estable, rendimiento balanceado", size: 180, pkg: "linux", tags: ["Recomendado", "x86_64"], detail: "Kernel vanilla oficial Arch, mejor compatibilidad drivers." },
              { id: "linux-zen", name: "Linux Zen", description: "7.1-zen optimizado para gaming y baja latencia", size: 185, pkg: "linux-zen", tags: ["Gaming", "Zen"], detail: "Scheduler PDS/BMQ, ideal para juegos y desktop fluido." },
              { id: "linux-hardened", name: "Linux Hardened", description: "Kernel endurecido con protecciones extra, ideal para entornos sensibles", size: 182, pkg: "linux-hardened", tags: ["Seguridad"], detail: "pacman -S linux-hardened — +0 MB extra, hardening, ASLR, stack protector.", cmd: "pacman -S linux-hardened" },
            ]
          },
          {
            id: "filesystem", name: "Sistema de Archivos", packages: [
              { id: "btrfs-progs", name: "Btrfs", description: "Moderno con snapshots + compresión zstd, recomendado para ArchMaker + snapper", size: 8, pkg: "btrfs-progs", tags: ["Recomendado"], fixed: true, essentialReason: "Btrfs permite snapshots antes de actualizar con snapper, compresión zstd ahorra 30% disco, subvolúmenes @/@home compatibles con rollback." },
              { id: "e2fsprogs", name: "Ext4", description: "Estable clásico, maduro y simple", size: 3, pkg: "e2fsprogs", tags: ["Clásico"], essentialReason: "Filesystem tradicional, máxima compatibilidad." },
            ]
          },
          {
            id: "base-tools", name: "Base Devel", packages: [
              { id: "base-devel", name: "base-devel", description: "Herramientas compilación: gcc, make, patch - esencial para AUR", size: 85, pkg: "base-devel", tags: ["Esencial"], fixed: true, essentialReason: "Arch actual requiere base-devel para compilar AUR (yay/paru). Incluye gcc, make, patch, pkgconf. Sin esto no puedes compilar Ghostty, Steam deps ni drivers." },
            ]
          },
          {
            id: "boot", name: "Bootloader", single: true, packages: [
              { id: "systemd-boot", name: "systemd-boot", description: "Ligero UEFI nativo, recomendado Arch 2025+, sin config compleja", size: 2, pkg: "systemd-boot", tags: ["Recomendado", "UEFI"], detail: "Boot <1s, config en /boot/loader/entries/, nativo systemd." },
              { id: "grub", name: "GRUB", description: "Clásico BIOS+UEFI, compatible todo", size: 15, pkg: "grub", tags: ["Clásico"], detail: "Soporta BIOS legacy, dual-boot Windows, temas." },
              { id: "limine", name: "Limine", description: "Bootloader moderno 2024-2026, soporte BIOS/UEFI, tema, más actual que efibootmgr", size: 4, pkg: "limine", tags: ["Actual", "Moderno"], detail: "Reemplaza efibootmgr: no solo gestiona entradas, bootea kernel con stivale2, config limine.conf visual." },
            ]
          },
        ]
      },
      {
        id: "environment",
        name: "Entorno Gráfico",
        icon: "🖥️",
        mode: "single",
        sections: [
          {
            id: "compositor", name: "Compositor / DE — Selección Única", single: true, packages: [
              { id: "hyprland", name: "Hyprland", description: "Hyprland 0.47 - Tiling dinámico Wayland con animaciones suaves a 144Hz", size: 45, pkg: "hyprland", tags: ["Moderno", "Wayland"] },
              { id: "gnome", name: "GNOME", description: "GNOME 50 - Pulido triple buffering, touch optimizado", size: 800, pkg: "gnome", tags: ["Pulido", "GNOME"] },
              { id: "sway", name: "Sway", description: "Sway 1.10 - Minimalista teclado, i3 en Wayland", size: 8, pkg: "sway", tags: ["Ligero", "Tiling"] },
              { id: "cosmic", name: "COSMIC", description: "COSMIC Epoch 1.0 - Nuevo en Rust, reemplaza a KDE Plasma, personalizable", size: 520, pkg: "cosmic-epoch", tags: ["Rust", "Nuevo"] },
            ]
          },
          {
            id: "dm", name: "Display Manager", single: true, packages: [
              { id: "gdm", name: "GDM", description: "GNOME Display Manager — Auto con GNOME, Wayland nativo", size: 12, pkg: "gdm", tags: ["GNOME"] },
              { id: "sddm", name: "SDDM", description: "Simple Desktop Display Manager — Para Hyprland/COSMIC, QML temable", size: 10, pkg: "sddm", tags: ["Recomendado"] },
              { id: "greetd-tuigreet", name: "greetd + tuigreet", description: "Minimalista TUI, ideal Sway/Hyprland", size: 3, pkg: "greetd-tuigreet", tags: ["Minimal"] },
            ]
          }
        ]
      },
      {
        id: "libpacks",
        name: "Packs de librerías",
        icon: "📚",
        mode: "grouped",
        sections: [
          {
            id: "office-pack", name: "Oficina", icon: "📄", packages: [
              { id: "libreoffice-fresh", name: "LibreOffice Fresh", description: "Suite completa Writer/Calc/Impress", size: 380, pkg: "libreoffice-fresh", tags: ["Esencial"], fixed: true, essentialReason: "Suite ofimática base compatible ODT/DOCX/XLSX, requerida para trabajo sin MS Office." },
              { id: "hunspell-es", name: "Hunspell ES", description: "Diccionario español ortografía", size: 2, pkg: "hunspell-es_es", tags: ["Esencial"], fixed: true, essentialReason: "Corrector ortográfico para LibreOffice y editores." },
              { id: "poppler", name: "Poppler", description: "Render PDF esencial", size: 5, pkg: "poppler", tags: ["Esencial"], fixed: true, essentialReason: "Librería base para visualizar PDFs en todo el sistema." },
              { id: "onlyoffice-bin", name: "OnlyOffice", description: "Compatibilidad MS Office 100%", size: 420, pkg: "onlyoffice-bin", tags: ["Recomendado"], essentialReason: "Mejor compatibilidad con .docx complejos, interfaz ribbon." },
              { id: "pdfarranger", name: "PDF Arranger", description: "Une/separa páginas PDF", size: 8, pkg: "pdfarranger", tags: ["Utilidad"] },
              { id: "nextcloud-client", name: "Nextcloud", description: "Sync cloud auto-hospedado", size: 25, pkg: "nextcloud-client", tags: ["Cloud"] },
            ]
          },
          {
            id: "dev-pack", name: "Desarrollo", icon: "🧑‍💻", packages: [
              { id: "git", name: "git", description: "Control versiones distribuido", size: 12, pkg: "git", tags: ["Esencial"], fixed: true, essentialReason: "Base para todo desarrollo, clonar AUR, commits." },
              { id: "python", name: "Python", description: "Python 3.13 + pip", size: 95, pkg: "python", tags: ["Esencial"], fixed: true, essentialReason: "Scripts, herramientas ArchMaker, backend ML." },
              { id: "nodejs", name: "Node.js", description: "Runtime JS + npm", size: 85, pkg: "nodejs", tags: ["Esencial"], fixed: true, essentialReason: "Cursor, frontend, herramientas web." },
              { id: "docker", name: "Docker", description: "Contenedores OCI", size: 120, pkg: "docker", tags: ["Recomendado"], essentialReason: "Aisla entornos dev sin ensuciar sistema." },
              { id: "podman", name: "Podman", description: "Docker sin daemon, rootless", size: 60, pkg: "podman", tags: ["Recomendado"] },
              { id: "rustup", name: "Rustup", description: "Toolchain Rust", size: 15, pkg: "rustup", tags: ["Rust"] },
              { id: "go", name: "Go", description: "Lenguaje Go", size: 140, pkg: "go", tags: ["Dev"] },
              { id: "jdk-openjdk", name: "JDK OpenJDK", description: "Java 21 LTS", size: 180, pkg: "jdk-openjdk", tags: ["Java"] },
            ]
          },
          {
            id: "gaming-pack", name: "Gaming", icon: "🎮", packages: [
              { id: "vulkan-icd-loader", name: "Vulkan ICD", description: "Loader Vulkan base", size: 1, pkg: "vulkan-icd-loader", tags: ["Esencial"], fixed: true, essentialReason: "API gráfica moderna requerida por 90% juegos 2024+" },
              { id: "lib32-vulkan-icd-loader", name: "lib32 Vulkan", description: "Vulkan 32-bit para Proton", size: 1, pkg: "lib32-vulkan-icd-loader", tags: ["Esencial"], fixed: true, essentialReason: "Steam Proton necesita libs 32-bit para juegos Windows." },
              { id: "gamemode", name: "GameMode", description: "Optimiza CPU/GPU al jugar", size: 2, pkg: "gamemode", tags: ["Esencial"], fixed: true, essentialReason: "Feral GameMode pone CPU governor performance automáticamente." },
              { id: "mangohud", name: "MangoHud", description: "Overlay FPS/temps", size: 4, pkg: "mangohud", tags: ["Recomendado"] },
              { id: "gamescope", name: "Gamescope", description: "Compositor gaming Valve", size: 6, pkg: "gamescope", tags: ["Recomendado"] },
              { id: "protonup-qt", name: "ProtonUp-Qt", description: "Gestiona Proton-GE", size: 12, pkg: "protonup-qt", tags: ["Proton"] },
              { id: "lutris", name: "Lutris", description: "Launcher juegos libres", size: 18, pkg: "lutris", tags: ["Launcher"] },
            ]
          }
        ]
      },
      {
        id: "apps",
        name: "Aplicaciones",
        icon: "📦",
        mode: "flat",
        sections: [
          {
            id: "browser", name: "Navegación", packages: [
              { id: "firefox", name: "Firefox", description: "Privado, rápido, motor Gecko", size: 250, pkg: "firefox", tags: ["Recomendado", "Privacidad"] },
              { id: "vivaldi", name: "Vivaldi", description: "Se adapta a ti, sidebars y workspaces", size: 380, pkg: "vivaldi", tags: ["Personalizable"] },
              { id: "brave", name: "Brave", description: "Bloquea anuncios nativo, BAT", size: 320, pkg: "brave-bin", tags: ["Privacidad"] },
            ]
          },
          {
            id: "comm", name: "Comunicación", packages: [
              { id: "discord", name: "Discord", description: "Chat voz y comunidades gaming", size: 210, pkg: "discord", tags: ["Gaming"] },
              { id: "telegram", name: "Telegram", description: "Mensajería rápida encriptada", size: 95, pkg: "telegram-desktop", tags: ["Ligero"] },
            ]
          },
          {
            id: "term", name: "Terminales", packages: [
              { id: "ghostty", name: "Ghostty", description: "Terminal GPU moderna escrita en Zig, tabs nativos, más rápida que Alacritty 2025+", size: 35, pkg: "ghostty", tags: ["Moderno", "Zig"] },
              { id: "kitty", name: "Kitty", description: "GPU, ligatures, extensible, estable", size: 28, pkg: "kitty", tags: ["Recomendado", "GPU"] },
              { id: "micro", name: "Micro", description: "Editor fácil en terminal, sin curva", size: 18, pkg: "micro", tags: ["Recomendado", "Ligero"] },
              { id: "cursor", name: "Cursor", description: "Editor con IA, fork VSCode", size: 450, pkg: "cursor-bin", tags: ["IA"] },
              { id: "btop", name: "btop", description: "Monitor de sistema bonito y rápido", size: 2, pkg: "btop", tags: ["Ligero"] },
            ]
          },
          {
            id: "media", name: "Multimedia", packages: [
              { id: "vlc", name: "VLC", description: "Reproductor universal, todo codec", size: 45, pkg: "vlc", tags: ["Fundamental", "Media"] },
              { id: "gimp", name: "GIMP", description: "Edición imagen, alternativa Photoshop", size: 150, pkg: "gimp", tags: ["Fundamental", "Creativo"] },
            ]
          },
          {
            id: "productivity", name: "Productividad", packages: [
              { id: "obsidian", name: "Obsidian", description: "Notas markdown + grafos, vault local", size: 120, pkg: "obsidian", tags: ["Fundamental", "Notas"] },
              { id: "onlyoffice-standalone", name: "OnlyOffice (app)", description: "Suite compatible MS Office (app separada)", size: 420, pkg: "onlyoffice-bin", tags: ["Oficina"] },
            ]
          },
          {
            id: "gaming", name: "Gaming", packages: [
              { id: "steam", name: "Steam", description: "Juegos Linux/Proton, biblioteca, Workshop", size: 180, pkg: "steam", tags: ["Gaming", "Fundamental"] },
            ]
          }
        ]
      },
      {
        id: "build",
        name: "Build & ISO",
        icon: "🏗️",
        mode: "flat",
        sections: [
          {
            id: "output", name: "Salida", packages: [
              { id: "zstd", name: "ZSTD", description: "Compresión rápida moderna ratio 15", size: 0, pkg: "zstd", tags: ["Recomendado"] },
              { id: "xz", name: "XZ", description: "Máxima compresión lenta", size: 0, pkg: "xz", tags: ["Estable"] },
            ]
          }
        ]
      }
    ]
  },
  validation: {
    engine: "live", blocking: true, rules: [
      { id: "single-compositor", type: "error", blocking: true, when: { count_gt: ["compositor", 1] }, action: "block", target: "compositor", note: "Solo 1 compositor permitido — selección única activada" },
      { id: "kernel-required", type: "error", blocking: true, when: { not: { in: ["linux", "linux-zen", "linux-hardened"] } }, action: "block", target: "kernel", note: "Debes seleccionar un kernel para bootear" },
      { id: "browser-required", type: "info", when: { not: { in: ["firefox", "brave", "vivaldi"] } }, action: "suggest", target: "browser", note: "Recomendado al menos un navegador para la ISO" },
      { id: "gdm-auto", type: "info", when: { eq: ["gnome"] }, action: "auto", target: "dm", note: "GDM auto-seleccionado y bloqueado para GNOME" },
    ]
  },
  palette: {
    canonical: "oklch",
    tokens: [
      { token: "--rh-red", hex: "#ee0000", oklch: "oklch(0.62 0.24 29)", usage: "primary action" },
      { token: "--rh-blue", hex: "#0066cc", oklch: "oklch(0.55 0.18 250)", usage: "focus info" },
      { token: "--rh-green", hex: "#3e8635", oklch: "oklch(0.60 0.15 145)", usage: "success" },
      { token: "--rh-yellow", hex: "#f0ab00", oklch: "oklch(0.80 0.16 85)", usage: "warning" },
      { token: "--rh-bg", hex: "#ffffff", oklch: "oklch(1 0 0)", usage: "background" },
      { token: "--rh-surface", hex: "#f5f5f5", oklch: "oklch(0.97 0 0)", usage: "surface" },
      { token: "--rh-border", hex: "#d2d2d2", oklch: "oklch(0.88 0 0)", usage: "border" },
    ]
  },
  build: { output_dir: "./out", iso_name_template: "archmaker-{version}-{arch}.iso", compression: "zstd", compression_level: 15, checksums: ["sha256"], pipeline: ["validate", "resolve", "derive_palette", "pacstrap", "assemble", "checksum"] },
  presets: [
    { id: "minimal-dev", name: "Minimal Dev", desc: "Hyprland + Ghostty + dev <1.2GB", selections: { kernel: "linux", boot: "systemd-boot", compositor: "hyprland", dm: "sddm", pkgs: ["btrfs-progs", "base-devel", "git", "ghostty", "firefox", "btop", "micro"] } },
    { id: "full-workstation", name: "Full Workstation", desc: "GNOME + GDM auto + Steam/VLC", selections: { kernel: "linux-zen", boot: "systemd-boot", compositor: "gnome", dm: "gdm", pkgs: ["btrfs-progs", "base-devel", "libreoffice-fresh", "ghostty", "firefox", "steam", "vlc", "obsidian"] } },
    { id: "gaming", name: "Gaming Station", desc: "Zen + COSMIC + Steam", selections: { kernel: "linux-zen", boot: "limine", compositor: "cosmic", dm: "sddm", pkgs: ["btrfs-progs", "vulkan-icd-loader", "lib32-vulkan-icd-loader", "gamemode", "steam", "mangohud", "brave", "ghostty"] } },
  ]
};

type Pkg = { id: string; name: string; description: string; size: number; pkg: string; tags: string[]; fixed?: boolean; essentialReason?: string; detail?: string; cmd?: string };
type Section = { id: string; name: string; icon?: string; single?: boolean; packages: Pkg[] };
type Step = { id: string; name: string; icon: string; mode: string; sections: Section[] };

const FIXED_IDS = ["btrfs-progs", "base-devel", "libreoffice-fresh", "hunspell-es", "poppler", "git", "python", "nodejs", "vulkan-icd-loader", "lib32-vulkan-icd-loader", "gamemode"];

export default function App() {
  // Single selections
  const [selectedKernel, setSelectedKernel] = useState<string>("linux");
  const [selectedBoot, setSelectedBoot] = useState<string>("systemd-boot");
  const [selectedCompositor, setSelectedCompositor] = useState<string>("hyprland");
  const [selectedDM, setSelectedDM] = useState<string>("sddm");
  const [gdmLocked, setGdmLocked] = useState(false);

  const [selectedOthers, setSelectedOthers] = useState<Set<string>>(new Set(["btrfs-progs", "base-devel", "libreoffice-fresh", "hunspell-es", "poppler", "git", "python", "nodejs", "vulkan-icd-loader", "lib32-vulkan-icd-loader", "gamemode", "firefox", "ghostty", "micro", "btop"]));
  const [currentStep, setCurrentStep] = useState<string>("base");
  const [dark, setDark] = useState(false);
  const [arch, setArch] = useState<string>("x86_64");
  const [deploy, setDeploy] = useState<string>("local");
  const [search, setSearch] = useState("");
  const [detail, setDetail] = useState<Pkg | null>(null);
  const [cmdOpen, setCmdOpen] = useState(false);
  const [archInfoOpen, setArchInfoOpen] = useState(false);
  const [outputDir, setOutputDir] = useState("./out");
  const [compression, setCompression] = useState("zstd");
  const [pipeline, setPipeline] = useState<string[]>(INSTANCE.build.pipeline);
  const [toast, setToast] = useState<{ msg: string; undo?: () => void } | null>(null);
  const cmdInputRef = useRef<HTMLInputElement>(null);

  const steps: Step[] = INSTANCE.configurator.steps as any;

  // Auto GDM when GNOME selected
  useEffect(() => {
    if (selectedCompositor === "gnome") {
      setSelectedDM("gdm");
      setGdmLocked(true);
    } else {
      setGdmLocked(false);
    }
  }, [selectedCompositor]);

  const allPkgs = useMemo(() => {
    const map = new Map<string, Pkg & { sectionId: string; stepId: string }>();
    steps.forEach(st => st.sections.forEach(sec => sec.packages.forEach(p => map.set(p.id, { ...p, sectionId: sec.id, stepId: st.id }))));
    return map;
  }, [steps]);

  const allSelectedIds = useMemo(() => {
    const s = new Set(selectedOthers);
    if (selectedKernel) s.add(selectedKernel);
    if (selectedBoot) s.add(selectedBoot);
    if (selectedCompositor) s.add(selectedCompositor);
    if (selectedDM) s.add(selectedDM);
    return s;
  }, [selectedOthers, selectedKernel, selectedBoot, selectedCompositor, selectedDM]);

  const totalSize = useMemo(() => {
    let s = 0;
    allSelectedIds.forEach(id => { const p = allPkgs.get(id); if (p) s += p.size; });
    return s;
  }, [allSelectedIds, allPkgs]);

  const filteredSteps = useMemo(() => {
    if (!search) return steps;
    const q = search.toLowerCase();
    return steps.map(st => ({
      ...st,
      sections: st.sections.map(sec => ({
        ...sec,
        packages: sec.packages.filter(p => p.name.toLowerCase().includes(q) || p.description.toLowerCase().includes(q) || p.pkg.toLowerCase().includes(q) || p.tags.some(t => t.toLowerCase().includes(q)))
      })).filter(sec => sec.packages.length > 0)
    })).filter(st => st.sections.length > 0);
  }, [search, steps]);

  const currentStepData = filteredSteps.find(s => s.id === currentStep) || steps.find(s => s.id === currentStep) || steps[0];

  const selectedInStep = (stepId: string) => {
    const st = steps.find(s => s.id === stepId);
    if (!st) return 0;
    let c = 0;
    st.sections.forEach(sec => sec.packages.forEach(p => { if (allSelectedIds.has(p.id)) c++; }));
    return c;
  };
  const totalInStep = (stepId: string) => {
    const st = steps.find(s => s.id === stepId);
    if (!st) return 0;
    return st.sections.reduce((acc, sec) => acc + sec.packages.length, 0);
  };

  const togglePkg = (id: string) => {
    if (FIXED_IDS.includes(id)) {
      setToast({ msg: `Fijo: ${allPkgs.get(id)?.name} no se puede quitar` });
      setTimeout(() => setToast(null), 2500);
      return;
    }
    setSelectedOthers(prev => {
      const next = new Set(prev);
      if (next.has(id)) {
        next.delete(id);
        setToast({ msg: `Removido ${allPkgs.get(id)?.name}`, undo: () => setSelectedOthers(s => { const n = new Set(s); n.add(id); return n; }) });
        setTimeout(() => setToast(null), 3500);
      } else {
        next.add(id);
      }
      return next;
    });
  };

  const handleKernelSelect = (id: string) => setSelectedKernel(id);
  const handleBootSelect = (id: string) => setSelectedBoot(id);
  const handleCompositorSelect = (id: string) => setSelectedCompositor(id);
  const handleDMSelect = (id: string) => {
    if (gdmLocked && id !== "gdm") return;
    setSelectedDM(id);
  };

  const pacstrapCmd = useMemo(() => {
    const pkgs = Array.from(allSelectedIds).map(id => allPkgs.get(id)?.pkg).filter(Boolean).join(" ");
    return `pacstrap /mnt base ${pkgs} --needed`;
  }, [allSelectedIds, allPkgs]);

  const validations = useMemo(() => {
    const results: { id: string; type: string; note: string; active: boolean }[] = [];
    const browsers = ["firefox", "brave", "vivaldi"].filter(id => allSelectedIds.has(id));
    results.push({
      id: "kernel-required",
      type: "error",
      note: "Debes seleccionar un kernel para bootear",
      active: !selectedKernel
    });
    results.push({
      id: "single-compositor",
      type: "error",
      note: "Solo 1 compositor permitido — modelo single activo ✓",
      active: false
    });
    results.push({
      id: "browser-required",
      type: "info",
      note: "Recomendado al menos un navegador para la ISO",
      active: browsers.length === 0
    });
    results.push({
      id: "gdm-auto",
      type: "info",
      note: selectedCompositor === "gnome" ? "GDM auto-bloqueado por GNOME — requerido" : "GNOME no seleccionado — DM libre",
      active: selectedCompositor === "gnome"
    });
    return results.filter(r => r.active);
  }, [allSelectedIds, selectedKernel, selectedCompositor]);

  useEffect(() => {
    const onKey = (e: KeyboardEvent) => {
      if ((e.metaKey || e.ctrlKey) && e.key.toLowerCase() === "k") {
        e.preventDefault();
        setCmdOpen(o => !o);
      }
      if (e.key === "Escape") {
        setCmdOpen(false);
        setDetail(null);
        setArchInfoOpen(false);
      }
    };
    window.addEventListener("keydown", onKey);
    return () => window.removeEventListener("keydown", onKey);
  }, []);
  useEffect(() => { if (cmdOpen) setTimeout(() => cmdInputRef.current?.focus(), 50); }, [cmdOpen]);

  useEffect(() => {
    document.title = "ARCHMAKER FORGE v5.1 - Single Compositor + LibPacks";
  }, []);

  const applyPreset = (presetId: string) => {
    const preset = INSTANCE.presets.find(p => p.id === presetId) as any;
    if (!preset) return;
    setSelectedKernel(preset.selections.kernel || "linux");
    setSelectedBoot(preset.selections.boot || "systemd-boot");
    setSelectedCompositor(preset.selections.compositor || "hyprland");
    setSelectedDM(preset.selections.dm || "sddm");
    const others = new Set(FIXED_IDS);
    (preset.selections.pkgs || []).forEach((id: string) => others.add(id));
    setSelectedOthers(others);
    setToast({ msg: `Preset ${preset.name} aplicado` });
    setTimeout(() => setToast(null), 3000);
  };

  const togglePipeline = (stage: string) => {
    setPipeline(prev => prev.includes(stage) ? prev.filter(s => s !== stage) : [...prev, stage]);
  };

  return (
    <div className={`${dark ? "dark" : ""} min-h-screen`}>
      <style>{`
        @import url('https://fonts.googleapis.com/css2?family=Red+Hat+Display:wght@500;700&family=Red+Hat+Text:wght@400;500&family=JetBrains+Mono:wght@400;500&display=swap');
        :root{--rh-red:#ee0000;--rh-black:#151515;--rh-gray:#6a6e73;--rh-gray-dark:#3c3f42;--rh-border:#d2d2d2;--rh-bg:#ffffff;--rh-surface:#f5f5f5;--rh-blue:#0066cc;--rh-green:#3e8635;--rh-yellow:#f0ab00;}
        .dark{--rh-bg:#151515;--rh-surface:#1e1e1e;--rh-border:#2a2a2a;--rh-black:#ffffff;--rh-gray:#a8a8a8;}
        *{font-family:'Red Hat Text',system-ui,sans-serif}
        h1,h2,h3{font-family:'Red Hat Display',sans-serif}
        .mono{font-family:'JetBrains Mono',monospace}
        .focus-ring:focus{outline:2px solid var(--rh-blue);outline-offset:2px}
        ::-webkit-scrollbar{width:6px;height:6px}
        ::-webkit-scrollbar-thumb{background:var(--rh-border);border-radius:10px}
        ::-webkit-scrollbar-track{background:transparent}
        .card-shadow{box-shadow:0 1px 0 var(--rh-border),0 1px 3px rgba(0,0,0,.06)}
      `}</style>

      <div className="min-h-screen bg-[var(--rh-bg)] text-[var(--rh-black)] antialiased flex flex-col">
        {/* HEADER */}
        <header className="sticky top-0 z-30 flex h-[64px] items-center justify-between border-b border-[var(--rh-border)] bg-[var(--rh-bg)] px-4 lg:px-6">
          <div className="flex items-center gap-5">
            <div className="flex items-center gap-3">
              <div className="flex h-8 w-8 items-center justify-center rounded-[4px] bg-[var(--rh-red)] text-white font-bold text-[13px] tracking-tighter">AM</div>
              <div className="leading-none">
                <h1 className="text-[17px] font-bold tracking-tight">ARCHMAKER</h1>
                <div className="mono text-[10px] tracking-widest text-[var(--rh-gray)]">FORGE v5.1 • {INSTANCE.archmaker.version} • Single+LibPacks</div>
              </div>
              <div className="hidden lg:flex ml-4 items-center gap-1 rounded-[6px] border border-[var(--rh-border)] bg-[var(--rh-surface)] p-1">
                {INSTANCE.archmaker.arch.map(a => (
                  <button key={a} onClick={() => { setArch(a); setToast({ msg: `Arch → ${a}` }); setTimeout(() => setToast(null), 2000); }} className={`mono px-2.5 py-1 text-[11px] rounded-[4px] transition ${arch === a ? "bg-[var(--rh-black)] text-[var(--rh-bg)]" : "text-[var(--rh-gray)] hover:text-[var(--rh-black)]"}`}>{a}</button>
                ))}
                <button onClick={() => setArchInfoOpen(true)} className="focus-ring ml-1 flex h-6 w-6 items-center justify-center rounded-[4px] border border-[var(--rh-border)] bg-[var(--rh-bg)] text-[12px] hover:border-[var(--rh-black)]">ⓘ</button>
              </div>
              <div className="hidden lg:flex items-center gap-1 rounded-[6px] border border-[var(--rh-border)] bg-[var(--rh-surface)] p-1">
                {["local", "remote", "chroot"].map(d => (
                  <button key={d} onClick={() => { setDeploy(d); setToast({ msg: `Deploy → ${d}` }); setTimeout(() => setToast(null), 2000); }} className={`mono px-2.5 py-1 text-[11px] rounded-[4px] capitalize transition ${deploy === d ? "bg-[var(--rh-black)] text-[var(--rh-bg)]" : "text-[var(--rh-gray)] hover:text-[var(--rh-black)]"}`}>{d}</button>
                ))}
              </div>
            </div>
          </div>

          <div className="flex items-center gap-2">
            <div className="relative hidden md:flex">
              <input value={search} onChange={e => setSearch(e.target.value)} placeholder="Filtrar paquetes..." className="focus-ring mono h-9 w-[260px] rounded-[6px] border border-[var(--rh-border)] bg-[var(--rh-surface)] px-3 pr-16 text-[12px] placeholder:text-[var(--rh-gray)]" />
              <div className="pointer-events-none absolute right-2 top-1.5 flex items-center gap-1">
                <kbd className="mono rounded bg-[var(--rh-bg)] border border-[var(--rh-border)] px-1.5 py-0.5 text-[10px]">⌘K</kbd>
              </div>
            </div>
            <button onClick={() => setCmdOpen(true)} className="focus-ring hidden md:flex h-9 w-9 items-center justify-center rounded-[6px] border border-[var(--rh-border)] bg-[var(--rh-surface)] text-[var(--rh-gray)] hover:text-[var(--rh-black)]">⌘</button>
            <button onClick={() => setArchInfoOpen(true)} className="focus-ring flex md:hidden h-9 w-9 items-center justify-center rounded-[6px] border border-[var(--rh-border)] bg-[var(--rh-surface)] text-[var(--rh-gray)] hover:text-[var(--rh-black)]">ⓘ</button>
            <button onClick={() => setDark(v => !v)} className="focus-ring h-9 w-9 rounded-[6px] border border-[var(--rh-border)] bg-[var(--rh-surface)] text-[var(--rh-gray)] hover:text-[var(--rh-black)]">{dark ? "☀︎" : "◐"}</button>
            <div className="ml-2 hidden lg:flex items-center gap-2 rounded-full border border-[var(--rh-border)] bg-[var(--rh-surface)] px-2 py-1">
              <div className="h-6 w-6 rounded-full bg-[var(--rh-black)] text-[var(--rh-bg)] grid place-items-center text-[11px] font-bold">R</div>
              <span className="mono text-[11px] pr-1">root</span>
            </div>
          </div>
        </header>

        {/* MAIN 3 COL LAYOUT */}
        <div className="flex flex-1 flex-col lg:flex-row min-h-0">
          {/* COL IZQ */}
          <aside className="w-full lg:w-[300px] shrink-0 border-b lg:border-b-0 lg:border-r border-[var(--rh-border)] bg-[var(--rh-surface)] flex flex-col">
            <div className="p-5">
              <div className="mono text-[10px] font-bold tracking-[0.14em] text-[var(--rh-gray)] uppercase">Configurator • single compositor</div>
              <div className="mt-4 space-y-2.5">
                {steps.map(step => {
                  const active = currentStep === step.id;
                  const sel = selectedInStep(step.id);
                  const tot = totalInStep(step.id);
                  return (
                    <button key={step.id} onClick={() => setCurrentStep(step.id)} className={`focus-ring group flex w-full items-center gap-3 rounded-[8px] border px-3 py-[11px] text-left transition ${active ? "bg-[var(--rh-bg)] border-[var(--rh-black)] card-shadow" : "border-transparent hover:bg-[var(--rh-bg)] hover:border-[var(--rh-border)]"}`}>
                      <span className="text-[16px]">{step.icon}</span>
                      <div className="flex-1">
                        <div className="flex items-center gap-2">
                          <span className="text-[13px] font-medium leading-none">{step.name}</span>
                          {active && <span className="h-1.5 w-1.5 rounded-full bg-[var(--rh-red)]" />}
                          {step.id === "libpacks" && <span className="mono rounded bg-[var(--rh-red)] text-white px-1 py-0.5 text-[8px]">NUEVO</span>}
                        </div>
                        <div className="mono mt-1 text-[10px] text-[var(--rh-gray)]">{step.id} • {sel}/{tot} {step.mode === "single" && "• único"}</div>
                      </div>
                      <div className="flex flex-col items-end gap-1">
                        <div className="h-1 w-12 overflow-hidden rounded-full bg-[var(--rh-border)]">
                          <div className="h-full bg-[var(--rh-red)] transition-all" style={{ width: `${tot ? (sel / tot) * 100 : 0}%` }} />
                        </div>
                        <span className="mono text-[10px] text-[var(--rh-gray)]">{Math.round(tot ? (sel / tot) * 100 : 0)}%</span>
                      </div>
                    </button>
                  );
                })}
              </div>
            </div>

            <div className="mt-auto border-t border-[var(--rh-border)] p-4">
              <div className="mono text-[10px] font-bold tracking-widest text-[var(--rh-gray)] uppercase">Schema Stats v5.1</div>
              <div className="mt-3 grid grid-cols-2 gap-2">
                <div className="rounded-[6px] border border-[var(--rh-border)] bg-[var(--rh-bg)] p-2">
                  <div className="mono text-[10px] text-[var(--rh-gray)]">STEPS</div>
                  <div className="text-[14px] font-bold">{steps.length}</div>
                </div>
                <div className="rounded-[6px] border border-[var(--rh-border)] bg-[var(--rh-bg)] p-2">
                  <div className="mono text-[10px] text-[var(--rh-gray)]">PACKAGES</div>
                  <div className="text-[14px] font-bold">{allPkgs.size}</div>
                </div>
              </div>
              <div className="mt-3 mono text-[10px] leading-[1.5] text-[var(--rh-gray)]">
                <div className="flex justify-between"><span>engine</span><span className="text-[var(--rh-black)]">live • blocking</span></div>
                <div className="flex justify-between"><span>compositor</span><span className="text-[var(--rh-red)] font-bold">single ✓</span></div>
                <div className="flex justify-between"><span>fixed pkgs</span><span className="text-[var(--rh-black)]">{FIXED_IDS.length}</span></div>
              </div>
            </div>
          </aside>

          {/* COL CENTRAL */}
          <main className="flex-1 min-w-0 flex flex-col bg-[var(--rh-bg)]">
            <div className="flex h-[44px] items-center justify-between border-b border-[var(--rh-border)] px-4 lg:px-6">
              <div className="mono flex items-center gap-2 text-[11px] text-[var(--rh-gray)]">
                <span className="text-[var(--rh-black)] font-medium">ArchMaker</span><span>›</span><span>configurator</span><span>›</span><span className="rounded bg-[var(--rh-surface)] border border-[var(--rh-border)] px-1.5 py-0.5 text-[var(--rh-black)]">{currentStepData.id}</span>
                <span className="hidden md:inline-flex ml-3 items-center gap-2">
                  <span className="h-3 w-px bg-[var(--rh-border)]" />
                  <span className="text-[10px]">{currentStepData.mode} • {currentStepData.sections.reduce((a, s) => a + s.packages.length, 0)} paquetes</span>
                </span>
              </div>
              <div className="mono hidden md:flex items-center gap-3 text-[10px] text-[var(--rh-gray)]">
                <span className="flex items-center gap-1"><span className="h-2 w-2 rounded-full bg-[var(--rh-green)]" /> {allSelectedIds.size} sel</span>
                <span className="flex items-center gap-1"><span className="h-2 w-2 rounded-full bg-[var(--rh-blue)]" /> {totalSize} MB</span>
                <span className="flex items-center gap-1 rounded bg-[var(--rh-black)] text-[var(--rh-bg)] px-1.5 py-0.5">{arch} • {selectedCompositor}</span>
              </div>
            </div>

            <div className="flex-1 overflow-auto p-5 lg:p-7">
              {currentStep === "build" ? (
                <div className="mx-auto max-w-[900px] space-y-6">
                  <div>
                    <h2 className="text-[22px] font-bold tracking-tight">Build & ISO</h2>
                    <p className="mono mt-1 text-[12px] text-[var(--rh-gray)]">v5.1 • single compositor + libpacks • schema $defs</p>
                  </div>
                  <div className="grid grid-cols-1 lg:grid-cols-2 gap-4">
                    <div className="rounded-[10px] border border-[var(--rh-border)] bg-[var(--rh-surface)] p-4">
                      <div className="mono text-[11px] font-bold tracking-widest uppercase text-[var(--rh-gray)]">Output</div>
                      <div className="mt-4 space-y-3">
                        <label className="block">
                          <span className="mono text-[11px] text-[var(--rh-gray)]">output_dir</span>
                          <input value={outputDir} onChange={e => setOutputDir(e.target.value)} className="focus-ring mono mt-1 w-full rounded-[6px] border border-[var(--rh-border)] bg-[var(--rh-bg)] px-3 py-2 text-[13px]" />
                        </label>
                        <label className="block">
                          <span className="mono text-[11px] text-[var(--rh-gray)]">iso_name_template</span>
                          <input value={INSTANCE.build.iso_name_template} readOnly className="mono mt-1 w-full rounded-[6px] border border-[var(--rh-border)] bg-[var(--rh-bg)] px-3 py-2 text-[13px] text-[var(--rh-gray)]" />
                          <span className="mono mt-1 block text-[10px] text-[var(--rh-gray)]">preview: archmaker-{INSTANCE.archmaker.version}-{arch}.iso • {selectedCompositor} + {selectedKernel}</span>
                        </label>
                        <div className="grid grid-cols-2 gap-3">
                          <label className="block">
                            <span className="mono text-[11px] text-[var(--rh-gray)]">compression</span>
                            <select value={compression} onChange={e => setCompression(e.target.value)} className="focus-ring mono mt-1 w-full rounded-[6px] border border-[var(--rh-border)] bg-[var(--rh-bg)] px-3 py-2 text-[13px]">
                              <option value="zstd">zstd</option><option value="xz">xz</option><option value="none">none</option>
                            </select>
                          </label>
                          <label className="block">
                            <span className="mono text-[11px] text-[var(--rh-gray)]">level {INSTANCE.build.compression_level}</span>
                            <input type="range" min={1} max={22} defaultValue={15} className="mt-3 w-full accent-[var(--rh-red)]" />
                          </label>
                        </div>
                      </div>
                    </div>
                    <div className="rounded-[10px] border border-[var(--rh-border)] bg-[var(--rh-surface)] p-4">
                      <div className="mono text-[11px] font-bold tracking-widest uppercase text-[var(--rh-gray)]">Pipeline • {pipeline.length} stages</div>
                      <div className="mt-4 space-y-2">
                        {INSTANCE.build.pipeline.map((stage, i) => (
                          <label key={stage} className={`flex items-center gap-3 rounded-[8px] border bg-[var(--rh-bg)] px-3 py-2.5 cursor-pointer ${pipeline.includes(stage) ? "border-[var(--rh-black)]" : "border-[var(--rh-border)] opacity-60"}`}>
                            <input type="checkbox" checked={pipeline.includes(stage)} onChange={() => togglePipeline(stage)} className="h-4 w-4 rounded border-[var(--rh-border)] accent-[var(--rh-red)]" />
                            <span className="mono text-[11px] text-[var(--rh-gray)]">{String(i + 1).padStart(2, "0")}</span>
                            <span className="mono text-[12px] font-medium">{stage}</span>
                            <span className="ml-auto mono text-[10px] px-1.5 py-0.5 rounded bg-[var(--rh-surface)] border border-[var(--rh-border)]">{stage === "pacstrap" ? `${allSelectedIds.size} pkgs` : stage === "checksum" ? "sha256" : "auto"}</span>
                          </label>
                        ))}
                      </div>
                      <div className="mt-3 mono text-[10px] text-[var(--rh-gray)]">checksums: {INSTANCE.build.checksums.join(", ")} • deploy: {deploy} • compositor: {selectedCompositor} (single)</div>
                    </div>
                  </div>
                  <div className="rounded-[10px] border border-[var(--rh-red)]/30 bg-[var(--rh-red)]/5 p-4">
                    <div className="flex items-center gap-2"><div className="h-2 w-2 rounded-full bg-[var(--rh-red)] animate-pulse" /><span className="mono text-[11px] font-bold tracking-widest uppercase">Build Preview v5.1</span></div>
                    <div className="mono mt-3 rounded-[8px] bg-[var(--rh-black)] text-[var(--rh-bg)] p-3 text-[11px] leading-relaxed">
                      <div><span className="text-[var(--rh-gray)]">$</span> archmaker build --arch {arch} --deploy {deploy} --compositor {selectedCompositor}</div>
                      <div className="text-[var(--rh-gray)]"># pipeline: {pipeline.join(" → ")}</div>
                      <div className="mt-1 text-[var(--rh-yellow)]"># output: {outputDir}/archmaker-{INSTANCE.archmaker.version}-{arch}.iso ({compression})</div>
                    </div>
                  </div>
                </div>
              ) : (
                <div className="space-y-8">
                  {currentStepData.sections.map(section => {
                    const isSingle = (section as any).single;
                    const isLibPack = currentStep === "libpacks";
                    const essentials = isLibPack ? section.packages.filter(p => p.fixed) : [];
                    const recommend = isLibPack ? section.packages.filter(p => !p.fixed) : section.packages;

                    return (
                      <div key={section.id}>
                        <div className="flex items-center justify-between">
                          <div className="flex items-center gap-3">
                            <h3 className="text-[15px] font-bold tracking-tight flex items-center gap-2">{section.icon && <span>{section.icon}</span>}{section.name}</h3>
                            <span className="mono rounded-full border border-[var(--rh-border)] bg-[var(--rh-surface)] px-2 py-0.5 text-[10px] text-[var(--rh-gray)]">{section.id} • {section.packages.length} {isSingle && "• único"}</span>
                            {isLibPack && <span className="mono rounded-full bg-[var(--rh-black)] text-[var(--rh-bg)] px-2 py-0.5 text-[10px]">{essentials.length} fijas • {recommend.length} rec</span>}
                          </div>
                          <div className="mono hidden md:flex items-center gap-2 text-[10px] text-[var(--rh-gray)]">
                            <span>{section.packages.filter(p => allSelectedIds.has(p.id)).length} / {section.packages.length} seleccionados {isSingle && "(single)"}</span>
                          </div>
                        </div>

                        {isLibPack ? (
                          <div className="mt-4 space-y-5">
                            <div>
                              <div className="mono text-[11px] font-bold tracking-widest uppercase text-[var(--rh-gray)] flex items-center gap-2"><span className="h-2 w-2 rounded-full bg-[var(--rh-green)]" />Esenciales (fijas) • {essentials.length}</div>
                              <div className="mt-3 grid grid-cols-1 md:grid-cols-2 gap-3">
                                {essentials.map(pkg => (
                                  <div key={pkg.id} className="relative flex gap-3 rounded-[10px] border border-[var(--rh-green)]/20 bg-[var(--rh-green)]/5 p-4">
                                    <input type="checkbox" checked disabled className="h-[18px] w-[18px] rounded-[4px] accent-[var(--rh-green)]" />
                                    <div className="min-w-0 flex-1">
                                      <div className="flex items-center gap-2">
                                        <span className="text-[13px] font-bold">{pkg.name}</span>
                                        <span className="mono rounded-full bg-[var(--rh-green)] text-white px-1.5 py-0.5 text-[9px]">FIJA</span>
                                        <span className="mono rounded border border-[var(--rh-border)] bg-[var(--rh-bg)] px-1.5 py-0.5 text-[9px]">{pkg.pkg}</span>
                                      </div>
                                      <p className="mt-1 text-[12px] leading-[1.4] text-[var(--rh-gray-dark)]">{pkg.description}</p>
                                      <div className="mt-2 mono rounded-[6px] bg-[var(--rh-bg)] border border-[var(--rh-border)] p-2 text-[10px] leading-[1.4]"><span className="font-bold">Por qué esencial:</span> {pkg.essentialReason}</div>
                                    </div>
                                  </div>
                                ))}
                              </div>
                            </div>
                            <div>
                              <div className="mono text-[11px] font-bold tracking-widest uppercase text-[var(--rh-gray)] flex items-center gap-2"><span className="h-2 w-2 rounded-full bg-[var(--rh-blue)]" />Recomendadas • {recommend.length}</div>
                              <div className="mt-3 grid grid-cols-1 md:grid-cols-2 gap-3">
                                {recommend.map(pkg => {
                                  const isSelected = allSelectedIds.has(pkg.id);
                                  return (
                                    <div key={pkg.id} className={`group relative flex gap-3 rounded-[10px] border p-4 transition card-shadow ${isSelected ? "bg-[var(--rh-black)] text-[var(--rh-bg)] border-[var(--rh-black)]" : "bg-[var(--rh-bg)] border-[var(--rh-border)] hover:border-[var(--rh-black)]"}`}>
                                      <input type="checkbox" checked={isSelected} onChange={() => togglePkg(pkg.id)} className={`h-[18px] w-[18px] rounded-[4px] border focus-ring ${isSelected ? "accent-[var(--rh-red)] border-white" : "accent-[var(--rh-red)] border-[var(--rh-border)]"}`} />
                                      <div className="min-w-0 flex-1">
                                        <div className="flex items-start justify-between gap-2">
                                          <span className={`text-[13px] font-bold truncate ${isSelected ? "text-white" : "text-[var(--rh-black)]"}`}>{pkg.name}</span>
                                          <button onClick={() => setDetail(pkg)} className={`focus-ring mono flex h-6 w-6 shrink-0 items-center justify-center rounded-[6px] border text-[11px] ${isSelected ? "border-white/20 bg-white/10 text-white" : "border-[var(--rh-border)] bg-[var(--rh-surface)]"}`}>i</button>
                                        </div>
                                        <p className={`mt-1 text-[12px] leading-[1.4] ${isSelected ? "text-white/70" : "text-[var(--rh-gray)]"}`}>{pkg.description}</p>
                                        {pkg.essentialReason && <p className={`mt-1 mono text-[10px] ${isSelected ? "text-white/60" : "text-[var(--rh-gray)]"}`}>{pkg.essentialReason}</p>}
                                        <div className="mt-2 flex gap-1.5">
                                          {pkg.tags.map(t => <span key={t} className={`mono rounded-full border px-2 py-0.5 text-[10px] ${isSelected ? "border-white/20 text-white/80" : "border-[var(--rh-border)] bg-[var(--rh-surface)]"}`}>{t}</span>)}
                                          <span className="mono ml-auto text-[10px]">{pkg.size} MB</span>
                                        </div>
                                      </div>
                                    </div>
                                  );
                                })}
                              </div>
                            </div>
                          </div>
                        ) : (
                          <div className="mt-3 grid grid-cols-1 md:grid-cols-2 gap-3">
                            {section.packages.map(pkg => {
                              const isFixed = FIXED_IDS.includes(pkg.id) || (pkg as any).fixed;
                              let isSelected = false;
                              let handleSelect = () => {};
                              if (section.id === "kernel") { isSelected = selectedKernel === pkg.id; handleSelect = () => handleKernelSelect(pkg.id); }
                              else if (section.id === "boot") { isSelected = selectedBoot === pkg.id; handleSelect = () => handleBootSelect(pkg.id); }
                              else if (section.id === "compositor") { isSelected = selectedCompositor === pkg.id; handleSelect = () => handleCompositorSelect(pkg.id); }
                              else if (section.id === "dm") { isSelected = selectedDM === pkg.id; handleSelect = () => handleDMSelect(pkg.id); }
                              else { isSelected = allSelectedIds.has(pkg.id); handleSelect = () => togglePkg(pkg.id); }

                              const isGdmAuto = section.id === "dm" && pkg.id === "gdm" && gdmLocked;

                              return (
                                <div key={pkg.id} className={`group relative flex gap-3 rounded-[10px] border p-4 transition card-shadow ${isSelected ? "bg-[var(--rh-black)] text-[var(--rh-bg)] border-[var(--rh-black)]" : "bg-[var(--rh-bg)] border-[var(--rh-border)] hover:border-[var(--rh-black)]"} ${isGdmAuto ? "ring-2 ring-[var(--rh-blue)] ring-offset-1" : ""}`}>
                                  <div className="pt-0.5">
                                    {isSingle ? (
                                      <input type="radio" checked={isSelected} onChange={handleSelect} disabled={isGdmAuto && selectedDM === "gdm" ? false : (section.id === "dm" && gdmLocked && pkg.id !== "gdm")} className="h-[18px] w-[18px] accent-[var(--rh-red)] focus-ring" />
                                    ) : (
                                      <input type="checkbox" checked={isSelected} disabled={isFixed} onChange={handleSelect} className={`h-[18px] w-[18px] rounded-[4px] border focus-ring ${isSelected ? "accent-[var(--rh-red)] border-white" : "accent-[var(--rh-red)] border-[var(--rh-border)]"}`} />
                                    )}
                                  </div>
                                  <div className="min-w-0 flex-1">
                                    <div className="flex items-start justify-between gap-2">
                                      <div className="flex items-center gap-2 min-w-0">
                                        <span className={`text-[13px] font-bold truncate ${isSelected ? "text-white" : "text-[var(--rh-black)]"}`}>{pkg.name}</span>
                                        <span className={`mono hidden md:inline-flex shrink-0 rounded-[4px] border px-1.5 py-0.5 text-[9px] tracking-widest uppercase ${isSelected ? "border-white/20 bg-white/10 text-white" : "border-[var(--rh-border)] bg-[var(--rh-surface)] text-[var(--rh-gray)]"}`}>{pkg.pkg}</span>
                                        {isFixed && <span className="mono rounded bg-[var(--rh-green)] text-white px-1 py-0.5 text-[8px]">FIJA</span>}
                                        {isGdmAuto && <span className="mono rounded bg-[var(--rh-blue)] text-white px-1.5 py-0.5 text-[8px] animate-pulse">AUTO</span>}
                                        {pkg.tags.includes("Recomendado") && !isFixed && !isGdmAuto && <span className="mono rounded bg-[var(--rh-red)] text-white px-1 py-0.5 text-[8px]">REC</span>}
                                      </div>
                                      <button onClick={() => setDetail(pkg)} className={`focus-ring mono flex h-6 w-6 shrink-0 items-center justify-center rounded-[6px] border text-[11px] ${isSelected ? "border-white/20 bg-white/10 text-white hover:bg-white/20" : "border-[var(--rh-border)] bg-[var(--rh-surface)] text-[var(--rh-gray)] hover:text-[var(--rh-black)]"}`}>i</button>
                                    </div>
                                    <p className={`mt-1 line-clamp-2 text-[12px] leading-[1.4] ${isSelected ? "text-white/70" : "text-[var(--rh-gray)]"}`}>{pkg.description}</p>
                                    {pkg.essentialReason && section.id !== "compositor" && (
                                      <div className={`mt-2 mono rounded-[6px] border p-1.5 text-[10px] leading-[1.3] ${isSelected ? "border-white/20 bg-white/5 text-white/70" : "border-[var(--rh-border)] bg-[var(--rh-surface)] text-[var(--rh-gray-dark)]"}`}>
                                        {isFixed ? "Por qué fijo: " : ""}{pkg.essentialReason}
                                      </div>
                                    )}
                                    <div className="mt-2.5 flex flex-wrap items-center gap-1.5">
                                      {pkg.tags.map(tag => (
                                        <span key={tag} className={`mono rounded-full border px-2 py-0.5 text-[10px] ${isSelected ? "border-white/20 bg-white/5 text-white/80" : tag === "Recomendado" ? "border-[var(--rh-red)]/20 bg-[var(--rh-red)]/10 text-[var(--rh-red)]" : tag === "Seguridad" ? "border-[var(--rh-green)]/20 bg-[var(--rh-green)]/10 text-[var(--rh-green)]" : "border-[var(--rh-border)] bg-[var(--rh-surface)] text-[var(--rh-gray-dark)]"}`}>{tag}</span>
                                      ))}
                                      <span className={`mono ml-auto rounded-full px-2 py-0.5 text-[10px] ${isSelected ? "bg-white/10 text-white" : "bg-[var(--rh-surface)] border border-[var(--rh-border)] text-[var(--rh-gray)]"}`}>{pkg.size} MB</span>
                                    </div>
                                    {isGdmAuto && <div className="mono mt-2 text-[10px] text-[var(--rh-blue)]">GDM requerido por GNOME — bloqueado</div>}
                                  </div>
                                  {isSelected && <div className="absolute left-0 top-1/2 -translate-y-1/2 h-[60%] w-[3px] rounded-r bg-[var(--rh-red)]" />}
                                </div>
                              );
                            })}
                          </div>
                        )}
                      </div>
                    );
                  })}
                </div>
              )}
            </div>
          </main>

          {/* COL DER */}
          <aside className="w-full lg:w-[360px] shrink-0 border-t lg:border-t-0 lg:border-l border-[var(--rh-border)] bg-[var(--rh-surface)] flex flex-col overflow-hidden">
            <div className="border-b border-[var(--rh-border)] bg-[var(--rh-bg)] p-4">
              <div className="flex items-center justify-between">
                <div className="mono text-[11px] font-bold tracking-[0.14em] uppercase">Manifiesto v5.1</div>
                <div className="flex items-center gap-1.5">
                  <span className="mono rounded-full bg-[var(--rh-black)] text-[var(--rh-bg)] px-2 py-0.5 text-[10px]">{allSelectedIds.size}</span>
                  <span className="mono rounded-full border border-[var(--rh-border)] bg-[var(--rh-surface)] px-2 py-0.5 text-[10px]">{totalSize} MB</span>
                </div>
              </div>
              <div className="mt-3 grid grid-cols-4 gap-2">
                <div className="rounded-[8px] border border-[var(--rh-border)] bg-[var(--rh-surface)] p-2">
                  <div className="mono text-[9px] text-[var(--rh-gray)]">KERNEL</div>
                  <div className="mono text-[11px] font-bold truncate">{selectedKernel}</div>
                </div>
                <div className="rounded-[8px] border border-[var(--rh-border)] bg-[var(--rh-surface)] p-2">
                  <div className="mono text-[9px] text-[var(--rh-gray)]">BOOT</div>
                  <div className="mono text-[11px] font-bold truncate">{selectedBoot}</div>
                </div>
                <div className="rounded-[8px] border border-[var(--rh-border)] bg-[var(--rh-surface)] p-2 col-span-2">
                  <div className="mono text-[9px] text-[var(--rh-gray)]">COMPOSITOR (único)</div>
                  <div className="mono text-[11px] font-bold truncate">{selectedCompositor} {gdmLocked && "→ gdm auto"}</div>
                </div>
                <div className="rounded-[8px] border border-[var(--rh-border)] bg-[var(--rh-surface)] p-2">
                  <div className="mono text-[10px] text-[var(--rh-gray)]">ARCH</div>
                  <div className="mono text-[12px] font-bold">{arch}</div>
                </div>
                <div className="rounded-[8px] border border-[var(--rh-border)] bg-[var(--rh-surface)] p-2 col-span-2">
                  <div className="mono text-[10px] text-[var(--rh-gray)]">DEPLOY</div>
                  <div className="mono text-[12px] font-bold">{deploy}</div>
                </div>
                <div className="rounded-[8px] border border-[var(--rh-border)] bg-[var(--rh-bg)] p-2">
                  <div className="mono text-[9px] text-[var(--rh-gray)]">FIJAS</div>
                  <div className="mono text-[12px] font-bold">{FIXED_IDS.length}</div>
                </div>
              </div>

              {/* Arch info mini */}
              <div className="mt-3 rounded-[8px] border border-[var(--rh-blue)]/20 bg-[var(--rh-blue)]/5 p-2.5">
                <div className="mono text-[10px] font-bold flex items-center justify-between"><span>Arch: {arch}</span><button onClick={() => setArchInfoOpen(true)} className="underline">ver tabla ⓘ</button></div>
                <div className="mono mt-1 text-[10px] leading-[1.4] text-[var(--rh-gray-dark)]">
                  {arch === "x86_64" ? "Intel/AMD 64-bit • compat total • Steam nativo • drivers maduros" : "ARM 64-bit • Apple Silicon/RPi5/Snapdragon X • eficiencia batería • box64 para Steam"}
                </div>
              </div>

              <div className="mt-3">
                <div className="mono mb-1.5 text-[10px] text-[var(--rh-gray)] uppercase tracking-widest">Selecciones • {Array.from(allSelectedIds).slice(0, 8).join(", ")}{allSelectedIds.size > 8 ? ` +${allSelectedIds.size - 8}` : ""}</div>
                <div className="flex flex-wrap gap-1 max-h-[90px] overflow-auto">
                  {Array.from(allSelectedIds).map(id => {
                    const isFixed = FIXED_IDS.includes(id);
                    return (
                      <span key={id} className={`mono inline-flex items-center gap-1 rounded-[6px] border px-2 py-1 text-[10px] ${isFixed ? "border-[var(--rh-green)]/20 bg-[var(--rh-green)]/10" : "border-[var(--rh-border)] bg-[var(--rh-surface)]"}`}>
                        {allPkgs.get(id)?.pkg || id}{isFixed && " • fija"}
                        {!isFixed && <button onClick={() => {
                          if (id === selectedKernel || id === selectedBoot || id === selectedCompositor || id === selectedDM) return;
                          togglePkg(id);
                        }} className="ml-1 rounded-full bg-[var(--rh-border)] px-1 leading-none">×</button>}
                      </span>
                    );
                  })}
                </div>
              </div>

              <div className="mt-4">
                <div className="mono mb-1.5 flex items-center justify-between text-[10px] tracking-widest uppercase text-[var(--rh-gray)]">
                  <span>pacstrap generado</span>
                  <button onClick={() => { navigator.clipboard?.writeText(pacstrapCmd); setToast({ msg: "Comando copiado" }); setTimeout(() => setToast(null), 2000); }} className="focus-ring rounded border border-[var(--rh-border)] bg-[var(--rh-bg)] px-1.5 py-0.5 text-[10px] hover:border-[var(--rh-black)]">copiar</button>
                </div>
                <div className="mono max-h-[110px] overflow-auto rounded-[8px] bg-[var(--rh-black)] p-3 text-[11px] leading-[1.6] text-[var(--rh-bg)]">
                  <span className="text-[var(--rh-gray)]">$</span> {pacstrapCmd}
                </div>
              </div>
            </div>

            <div className="border-b border-[var(--rh-border)] bg-[var(--rh-bg)] p-4">
              <div className="flex items-center justify-between">
                <div className="mono text-[11px] font-bold tracking-[0.14em] uppercase">Validación • live blocking</div>
                <div className={`mono rounded-full px-2 py-0.5 text-[10px] border ${validations.some(v => v.type === "error") ? "bg-[var(--rh-red)] text-white border-[var(--rh-red)]" : validations.length ? "bg-[var(--rh-yellow)]/20 text-[var(--rh-gray-dark)] border-[var(--rh-yellow)]/30" : "bg-[var(--rh-green)]/10 text-[var(--rh-green)] border-[var(--rh-green)]/20"}`}>
                  {validations.some(v => v.type === "error") ? `${validations.length} bloqueos` : validations.length ? `${validations.length} avisos` : "válido • single ✓"}
                </div>
              </div>
              <div className="mt-3 space-y-2">
                {validations.length === 0 && (
                  <div className="flex gap-2 rounded-[8px] border border-[var(--rh-green)]/20 bg-[var(--rh-green)]/5 p-3">
                    <span className="text-[var(--rh-green)]">✓</span>
                    <div className="mono text-[11px] leading-[1.4]"><span className="font-bold text-[var(--rh-green)]">Sin conflictos. Single compositor OK.</span><br /><span className="text-[var(--rh-gray)]">Configuración lista: {selectedCompositor} + {selectedKernel} + {selectedBoot}</span></div>
                  </div>
                )}
                {validations.map(v => (
                  <div key={v.id} className={`flex gap-2 rounded-[8px] border p-3 ${v.type === "error" ? "border-[var(--rh-red)]/30 bg-[var(--rh-red)]/5" : "border-[var(--rh-blue)]/20 bg-[var(--rh-blue)]/5"}`}>
                    <span className={`mt-0.5 text-[12px] ${v.type === "error" ? "text-[var(--rh-red)]" : "text-[var(--rh-blue)]"}`}>{v.type === "error" ? "✕" : "ℹ"}</span>
                    <div className="min-w-0">
                      <div className="mono text-[11px] font-bold">{v.id} • {v.type}</div>
                      <div className="mono mt-0.5 text-[11px] leading-[1.4] text-[var(--rh-gray-dark)]">{v.note}</div>
                    </div>
                  </div>
                ))}
              </div>
              <div className="mt-2 mono text-[10px] text-[var(--rh-gray)]">single-compositor error blocking true • gdm-auto info • kernel-required error</div>
            </div>

            <div className="flex-1 overflow-auto p-4 space-y-5">
              <div>
                <div className="mono text-[11px] font-bold tracking-[0.14em] uppercase">Palette • {INSTANCE.palette.canonical}</div>
                <div className="mt-3 space-y-2">
                  {INSTANCE.palette.tokens.map(t => (
                    <div key={t.token} className="flex items-center gap-3 rounded-[8px] border border-[var(--rh-border)] bg-[var(--rh-bg)] px-3 py-2.5">
                      <div className="h-8 w-8 rounded-[6px] border border-[var(--rh-border)] shadow-inner" style={{ background: t.hex }} />
                      <div className="min-w-0 flex-1">
                        <div className="mono text-[11px] font-bold">{t.token}</div>
                        <div className="mono text-[10px] text-[var(--rh-gray)] truncate">{t.hex} • {t.oklch}</div>
                      </div>
                      <div className="mono rounded-full border border-[var(--rh-border)] bg-[var(--rh-surface)] px-2 py-0.5 text-[9px]">{t.usage}</div>
                    </div>
                  ))}
                </div>
              </div>

              <div>
                <div className="mono text-[11px] font-bold tracking-[0.14em] uppercase">Presets v5.1 • {INSTANCE.presets.length}</div>
                <div className="mt-3 space-y-2">
                  {INSTANCE.presets.map(preset => (
                    <button key={preset.id} onClick={() => applyPreset(preset.id)} className="focus-ring w-full text-left rounded-[10px] border border-[var(--rh-border)] bg-[var(--rh-bg)] p-3 hover:border-[var(--rh-black)] transition">
                      <div className="flex items-center justify-between">
                        <span className="text-[13px] font-bold">{preset.name}</span>
                        <span className="mono rounded bg-[var(--rh-black)] text-[var(--rh-bg)] px-1.5 py-0.5 text-[9px]">aplicar</span>
                      </div>
                      <div className="mono mt-1 text-[11px] text-[var(--rh-gray)] leading-[1.3]">{preset.desc}</div>
                      <div className="mono mt-2 flex flex-wrap gap-1">
                        <span className="rounded bg-[var(--rh-surface)] border border-[var(--rh-border)] px-1.5 py-0.5 text-[9px]">{(preset as any).selections.kernel}</span>
                        <span className="rounded bg-[var(--rh-surface)] border border-[var(--rh-border)] px-1.5 py-0.5 text-[9px]">{(preset as any).selections.compositor}</span>
                        <span className="rounded bg-[var(--rh-surface)] border border-[var(--rh-border)] px-1.5 py-0.5 text-[9px]">{(preset as any).selections.dm}</span>
                        <span className="rounded bg-[var(--rh-surface)] border border-[var(--rh-border)] px-1.5 py-0.5 text-[9px]">{(preset as any).selections.pkgs?.length} pkgs</span>
                      </div>
                    </button>
                  ))}
                </div>
              </div>

              <div className="rounded-[10px] border border-[var(--rh-border)] bg-[var(--rh-bg)] p-3">
                <div className="mono text-[10px] font-bold tracking-widest uppercase text-[var(--rh-gray)]">Arch Info • {arch}</div>
                <div className="mono mt-1 text-[10px] leading-[1.5] text-[var(--rh-gray-dark)]">
                  {arch === "x86_64" ? "Intel/AMD 64-bit: AUR completo, Steam nativo, Nvidia/AMD drivers maduros. Recomendado PC/laptop estándar." : "ARM 64-bit: Apple Silicon, RPi5, Snapdragon X Elite. Eficiencia superior, batería. Limit: algunos AUR no compilados, Steam box64, drivers en evolución."}
                </div>
                <button onClick={() => setArchInfoOpen(true)} className="mono mt-2 w-full rounded-[6px] border border-[var(--rh-border)] bg-[var(--rh-surface)] py-1.5 text-[10px] hover:border-[var(--rh-black)]">Ver comparativa completa ⓘ</button>
              </div>
            </div>
          </aside>
        </div>

        {toast && (
          <div className="fixed bottom-4 left-4 z-50 flex items-center gap-3 rounded-[10px] border border-[var(--rh-border)] bg-[var(--rh-black)] px-4 py-3 text-white shadow-2xl">
            <span className="mono text-[12px]">{toast.msg}</span>
            {toast.undo && <button onClick={() => { toast.undo?.(); setToast(null); }} className="focus-ring mono rounded-[6px] bg-white px-2.5 py-1 text-[11px] font-bold text-black">Deshacer</button>}
            <button onClick={() => setToast(null)} className="mono text-[11px] opacity-60 hover:opacity-100">✕</button>
          </div>
        )}

        {detail && (
          <div className="fixed inset-0 z-40 flex items-center justify-center bg-black/50 backdrop-blur-[2px] p-4" onClick={() => setDetail(null)}>
            <div onClick={e => e.stopPropagation()} className="w-full max-w-[560px] rounded-[14px] border border-[var(--rh-border)] bg-[var(--rh-bg)] card-shadow overflow-hidden">
              <div className="flex items-center justify-between border-b border-[var(--rh-border)] px-5 py-4">
                <div className="flex items-center gap-3">
                  <div className="grid h-9 w-9 place-items-center rounded-[8px] bg-[var(--rh-surface)] border border-[var(--rh-border)] mono text-[12px] font-bold">{detail.pkg.slice(0, 2)}</div>
                  <div>
                    <div className="text-[14px] font-bold flex items-center gap-2">{detail.name} {detail.fixed && <span className="mono rounded bg-[var(--rh-green)] text-white px-1.5 py-0.5 text-[9px]">FIJA</span>}</div>
                    <div className="mono text-[11px] text-[var(--rh-gray)]">{detail.pkg} • {detail.size} MB</div>
                  </div>
                </div>
                <button onClick={() => setDetail(null)} className="focus-ring h-8 w-8 rounded-[8px] border border-[var(--rh-border)] bg-[var(--rh-surface)]">✕</button>
              </div>
              <div className="p-5 space-y-4">
                <p className="text-[13px] leading-[1.5] text-[var(--rh-gray-dark)]">{detail.description}. Paquete oficial Arch, derivado del schema $defs/option con id {detail.id}.</p>
                {detail.essentialReason && <div className="mono rounded-[8px] bg-[var(--rh-surface)] border border-[var(--rh-border)] p-3 text-[11px]"><span className="font-bold">Por qué:</span> {detail.essentialReason}</div>}
                {detail.detail && <div className="mono rounded-[8px] bg-[var(--rh-blue)]/5 border border-[var(--rh-blue)]/20 p-3 text-[11px]">{detail.detail}</div>}
                {detail.cmd && <div className="mono rounded-[8px] bg-[var(--rh-black)] text-[var(--rh-bg)] p-2.5 text-[11px]">{detail.cmd}</div>}
                <div className="flex flex-wrap gap-1.5">
                  {detail.tags.map(t => <span key={t} className="mono rounded-full border border-[var(--rh-border)] bg-[var(--rh-surface)] px-2.5 py-1 text-[11px]">{t}</span>)}
                </div>
                <div className="mono rounded-[8px] bg-[var(--rh-surface)] border border-[var(--rh-border)] p-3 text-[11px]">
                  <div className="flex justify-between"><span className="text-[var(--rh-gray)]">id</span><span>{detail.id}</span></div>
                  <div className="flex justify-between"><span className="text-[var(--rh-gray)]">size</span><span>{detail.size} MB</span></div>
                  <div className="flex justify-between"><span className="text-[var(--rh-gray)]">pkg</span><span>{detail.pkg}</span></div>
                  <div className="flex justify-between"><span className="text-[var(--rh-gray)]">fixed</span><span>{detail.fixed ? "true • no deseleccionable" : "false"}</span></div>
                  <div className="flex justify-between"><span className="text-[var(--rh-gray)]">source</span><span>v5.1 single+libpacks</span></div>
                </div>
                <div className="flex gap-2">
                  <button onClick={() => {
                    if (FIXED_IDS.includes(detail.id)) return;
                    if (["linux", "linux-zen", "linux-hardened"].includes(detail.id)) { setSelectedKernel(detail.id); setDetail(null); return; }
                    if (["systemd-boot", "grub", "limine"].includes(detail.id)) { setSelectedBoot(detail.id); setDetail(null); return; }
                    if (["hyprland", "gnome", "sway", "cosmic"].includes(detail.id)) { setSelectedCompositor(detail.id); setDetail(null); return; }
                    if (["gdm", "sddm", "greetd-tuigreet"].includes(detail.id)) { if (gdmLocked && detail.id !== "gdm") return; setSelectedDM(detail.id); setDetail(null); return; }
                    togglePkg(detail.id); setDetail(null);
                  }} disabled={FIXED_IDS.includes(detail.id)} className={`focus-ring flex-1 rounded-[8px] px-4 py-2.5 text-[13px] font-bold ${FIXED_IDS.includes(detail.id) ? "bg-[var(--rh-surface)] border border-[var(--rh-border)] text-[var(--rh-gray)]" : allSelectedIds.has(detail.id) || selectedKernel === detail.id || selectedBoot === detail.id || selectedCompositor === detail.id || selectedDM === detail.id ? "bg-[var(--rh-surface)] border border-[var(--rh-border)] text-[var(--rh-black)]" : "bg-[var(--rh-red)] text-white"}`}>{FIXED_IDS.includes(detail.id) ? "Fija — no removible" : (allSelectedIds.has(detail.id) || selectedKernel === detail.id || selectedBoot === detail.id || selectedCompositor === detail.id || selectedDM === detail.id) ? "Quitar" : "Añadir"}</button>
                  <button onClick={() => setDetail(null)} className="focus-ring rounded-[8px] border border-[var(--rh-border)] bg-[var(--rh-bg)] px-4 py-2.5 text-[13px]">Cerrar</button>
                </div>
              </div>
            </div>
          </div>
        )}

        {archInfoOpen && (
          <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/50 backdrop-blur-[2px] p-4" onClick={() => setArchInfoOpen(false)}>
            <div onClick={e => e.stopPropagation()} className="w-full max-w-[720px] rounded-[14px] border border-[var(--rh-border)] bg-[var(--rh-bg)] card-shadow overflow-hidden max-h-[90vh] overflow-auto">
              <div className="sticky top-0 bg-[var(--rh-bg)] flex items-center justify-between border-b border-[var(--rh-border)] px-6 py-4">
                <div>
                  <h2 className="text-[18px] font-bold">x86_64 vs aarch64</h2>
                  <div className="mono text-[11px] text-[var(--rh-gray)]">ArchMaker soporta ambos con mismo schema, pero pacstrap y kernel difieren.</div>
                </div>
                <button onClick={() => setArchInfoOpen(false)} className="h-8 w-8 rounded-[8px] border border-[var(--rh-border)] bg-[var(--rh-surface)]">✕</button>
              </div>
              <div className="p-6 space-y-6">
                <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                  <div className="rounded-[10px] border border-[var(--rh-border)] bg-[var(--rh-surface)] p-4">
                    <div className="flex items-center gap-2"><span className="mono rounded bg-[var(--rh-black)] text-[var(--rh-bg)] px-2 py-0.5 text-[10px]">x86_64</span><span className="text-[14px] font-bold">Intel/AMD 64-bit</span></div>
                    <p className="mono mt-2 text-[12px] leading-[1.5] text-[var(--rh-gray-dark)]">Compatibilidad total, AUR completo, Steam/Gaming nativo, drivers Nvidia/AMD maduros. Recomendado para PC/laptop estándar.</p>
                    <ul className="mono mt-3 space-y-1 text-[11px] text-[var(--rh-gray-dark)]">
                      <li>• Kernel: linux / linux-zen / linux-hardened nativo</li>
                      <li>• Gaming: Vulkan, Proton, Steam sin traducción</li>
                      <li>• AUR: 99% paquetes compilan directo</li>
                      <li>• Boot: systemd-boot, GRUB, Limine BIOS+UEFI</li>
                    </ul>
                  </div>
                  <div className="rounded-[10px] border border-[var(--rh-blue)]/20 bg-[var(--rh-blue)]/5 p-4">
                    <div className="flex items-center gap-2"><span className="mono rounded bg-[var(--rh-blue)] text-white px-2 py-0.5 text-[10px]">aarch64</span><span className="text-[14px] font-bold">ARM 64-bit</span></div>
                    <p className="mono mt-2 text-[12px] leading-[1.5] text-[var(--rh-gray-dark)]">Apple Silicon, Raspberry Pi 5, Snapdragon X Elite, eficiencia energética superior, batería. Limitaciones: algunos paquetes AUR no compilados, Steam requiere box64, drivers en evolución.</p>
                    <ul className="mono mt-3 space-y-1 text-[11px] text-[var(--rh-gray-dark)]">
                      <li>• Kernel: linux-aarch64 variantes</li>
                      <li>• Eficiencia: 2-3x batería vs x86</li>
                      <li>• Limit: box64/binfmt para x86 emulation</li>
                      <li>• Ideal: dev portátil, servidores ARM</li>
                    </ul>
                  </div>
                </div>

                <div className="rounded-[10px] border border-[var(--rh-border)] overflow-hidden">
                  <div className="mono bg-[var(--rh-surface)] px-4 py-2 text-[11px] font-bold tracking-widest uppercase">Tabla comparativa</div>
                  <div className="mono text-[11px] divide-y divide-[var(--rh-border)]">
                    {[
                      ["Compatibilidad AUR", "✓ Completo", "⚠ ~85% (box64)", arch === "x86_64"],
                      ["Steam nativo", "✓ Proton directo", "✗ box64 + proton", arch === "x86_64"],
                      ["Drivers Nvidia/AMD", "✓ Maduros", "⚠ En evolución", arch === "x86_64"],
                      ["Eficiencia batería", "⚠ 4-6h", "✓ 10-18h", arch === "aarch64"],
                      ["Apple Silicon", "✗ No", "✓ Nativo", arch === "aarch64"],
                      ["Raspberry Pi 5", "✗ No", "✓ Soportado", arch === "aarch64"],
                      ["Snapdragon X Elite", "✗ No", "✓ Soportado", arch === "aarch64"],
                      ["pacstrap", "base + kernel x86_64", "base + kernel aarch64", null],
                    ].map(([feat, x86, arm, highlight]) => (
                      <div key={feat as string} className="grid grid-cols-[1.2fr_1fr_1fr] gap-2 px-4 py-2.5">
                        <span className="font-bold">{feat}</span>
                        <span className={`${highlight === true ? "bg-[var(--rh-green)]/10 rounded px-1.5" : ""}`}>{x86}</span>
                        <span className={`${highlight === false ? "bg-[var(--rh-blue)]/10 rounded px-1.5" : ""}`}>{arm}</span>
                      </div>
                    ))}
                  </div>
                </div>

                <div className="mono rounded-[8px] bg-[var(--rh-black)] text-[var(--rh-bg)] p-3 text-[11px] leading-[1.6]">
                  <div className="text-[var(--rh-yellow)]"># ArchMaker soporta ambos con mismo schema</div>
                  <div><span className="text-[var(--rh-gray)]">$</span> pacstrap /mnt base {arch === "x86_64" ? "linux" : "linux-aarch64"} btrfs-progs base-devel {selectedBoot} {selectedCompositor}</div>
                  <div className="text-[var(--rh-gray)]"># arch actual: {arch} • compositor único: {selectedCompositor} • dm: {selectedDM}{gdmLocked ? " (auto GDM)" : ""}</div>
                </div>

                <div className="flex gap-2">
                  <button onClick={() => { setArch("x86_64"); setArchInfoOpen(false); }} className={`flex-1 rounded-[8px] py-2.5 mono text-[12px] font-bold border ${arch === "x86_64" ? "bg-[var(--rh-black)] text-[var(--rh-bg)] border-[var(--rh-black)]" : "bg-[var(--rh-bg)] border-[var(--rh-border)]"}`}>Usar x86_64</button>
                  <button onClick={() => { setArch("aarch64"); setArchInfoOpen(false); }} className={`flex-1 rounded-[8px] py-2.5 mono text-[12px] font-bold border ${arch === "aarch64" ? "bg-[var(--rh-blue)] text-white border-[var(--rh-blue)]" : "bg-[var(--rh-bg)] border-[var(--rh-border)]"}`}>Usar aarch64</button>
                </div>
              </div>
            </div>
          </div>
        )}

        {cmdOpen && (
          <div className="fixed inset-0 z-[60] flex items-start justify-center bg-black/40 backdrop-blur-sm p-4 pt-[12vh]" onClick={() => setCmdOpen(false)}>
            <div onClick={e => e.stopPropagation()} className="w-full max-w-[640px] overflow-hidden rounded-[14px] border border-[var(--rh-border)] bg-[var(--rh-bg)] shadow-2xl">
              <div className="flex items-center gap-3 border-b border-[var(--rh-border)] px-4 py-3">
                <span className="text-[var(--rh-gray)]">⌘</span>
                <input ref={cmdInputRef} value={search} onChange={e => setSearch(e.target.value)} placeholder="Buscar paquete, step, regla... (eq, in, not)" className="mono flex-1 bg-transparent text-[14px] placeholder:text-[var(--rh-gray)] outline-none" />
                <kbd className="mono rounded border border-[var(--rh-border)] bg-[var(--rh-surface)] px-1.5 py-0.5 text-[10px]">ESC</kbd>
              </div>
              <div className="max-h-[380px] overflow-auto p-2">
                <div className="mono px-3 py-2 text-[10px] font-bold tracking-widest uppercase text-[var(--rh-gray)]">Paquetes • {allPkgs.size} • single compositor activo</div>
                {Array.from(allPkgs.values()).filter(p => !search || p.name.toLowerCase().includes(search.toLowerCase()) || p.pkg.toLowerCase().includes(search.toLowerCase())).slice(0, 14).map(p => (
                  <button key={p.id} onClick={() => {
                    if (["linux", "linux-zen", "linux-hardened"].includes(p.id)) setSelectedKernel(p.id);
                    else if (["systemd-boot", "grub", "limine"].includes(p.id)) setSelectedBoot(p.id);
                    else if (["hyprland", "gnome", "sway", "cosmic"].includes(p.id)) setSelectedCompositor(p.id);
                    else if (["gdm", "sddm", "greetd-tuigreet"].includes(p.id)) { if (!(gdmLocked && p.id !== "gdm")) setSelectedDM(p.id); }
                    else togglePkg(p.id);
                    setCmdOpen(false);
                  }} className="flex w-full items-center gap-3 rounded-[8px] px-3 py-2.5 text-left hover:bg-[var(--rh-surface)]">
                    <input type={["linux", "linux-zen", "linux-hardened", "systemd-boot", "grub", "limine", "hyprland", "gnome", "sway", "cosmic", "gdm", "sddm", "greetd-tuigreet"].includes(p.id) ? "radio" : "checkbox"} checked={allSelectedIds.has(p.id)} readOnly className="h-4 w-4 rounded accent-[var(--rh-red)]" />
                    <div className="min-w-0 flex-1">
                      <div className="text-[13px] font-medium">{p.name} <span className="mono text-[11px] text-[var(--rh-gray)]">— {p.pkg}</span></div>
                      <div className="mono text-[11px] text-[var(--rh-gray)] truncate">{p.description}</div>
                    </div>
                    <span className="mono rounded bg-[var(--rh-surface)] border border-[var(--rh-border)] px-1.5 py-0.5 text-[10px]">{p.stepId}/{p.sectionId}</span>
                  </button>
                ))}
                <div className="mono px-3 py-2 text-[10px] font-bold tracking-widest uppercase text-[var(--rh-gray)] mt-2">Steps</div>
                {steps.map(s => (
                  <button key={s.id} onClick={() => { setCurrentStep(s.id); setCmdOpen(false); }} className="flex w-full items-center gap-3 rounded-[8px] px-3 py-2.5 text-left hover:bg-[var(--rh-surface)]">
                    <span>{s.icon}</span><span className="text-[13px]">{s.name}</span><span className="mono ml-auto text-[10px] text-[var(--rh-gray)]">{s.id}</span>
                  </button>
                ))}
              </div>
              <div className="flex items-center justify-between border-t border-[var(--rh-border)] bg-[var(--rh-surface)] px-4 py-2 mono text-[10px] text-[var(--rh-gray)]">
                <span>↑↓ navegar • ⏎ seleccionar • v5.1 single compositor</span><span>FORGE v5.1</span>
              </div>
            </div>
          </div>
        )}

        <div className="lg:hidden border-t border-[var(--rh-border)] bg-[var(--rh-surface)] p-3 flex gap-2 md:hidden">
          <input value={search} onChange={e => setSearch(e.target.value)} placeholder="Filtrar..." className="focus-ring mono flex-1 rounded-[8px] border border-[var(--rh-border)] bg-[var(--rh-bg)] px-3 py-2.5 text-[12px]" />
          <button onClick={() => setCmdOpen(true)} className="focus-ring mono rounded-[8px] border border-[var(--rh-border)] bg-[var(--rh-bg)] px-3 py-2.5 text-[12px]">⌘K</button>
        </div>
      </div>
    </div>
  );
}
