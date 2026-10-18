/**
 * Presentation data for the Forge face (ArchMaker GUI).
 * Canonical ids map to local-catalog-mvp1; unverified requests stay selectable and never seal as fully supported.
 */

export type EnvKind = "DE" | "WM";

export type VerificationStatus = "verified" | "unverified";

export type FaceOption = {
  htmlId: string;
  optionId: string;
  stepId: string;
  title: string;
  description: string;
  tag?: string;
  version?: string;
  kind?: EnvKind;
  protocol?: string;
  ideal?: string;
  /** @deprecated Use verificationStatus === "unverified". Never disables selection. */
  gap?: boolean;
  verificationStatus?: VerificationStatus;
  browserBase?: string;
  renderingEngine?: string;
};

export type FacePackage = {
  htmlId: string;
  optionId: string;
  title: string;
  description: string;
  /**
   * Estimación editorial de cara (MB). No es metadato oficial del catálogo;
   * la UI debe etiquetarla como estimación o omitirla como cifra fiable (H09).
   */
  size: number;
  tag: string;
  why: string;
  /** @deprecated Use verificationStatus === "unverified". Never disables selection. */
  gap?: boolean;
  verificationStatus?: VerificationStatus;
  browserBase?: string;
  renderingEngine?: string;
};

export type FaceLayer = {
  id: string;
  title: string;
  icon: string;
  description: string;
  simple: boolean;
  packages: FacePackage[];
};

export const STEP_IDS = {
  intent: "archmaker.face.intent",
  kernel: "archmaker.base.kernel",
  boot: "archmaker.base.boot",
  filesystem: "archmaker.base.filesystem",
  compositor: "archmaker.environment.compositor",
  apps: "archmaker.face.apps",
  hostname: "archmaker.face.hostname",
  encrypt: "archmaker.face.encrypt",
} as const;

export const RAIL_STEPS = [
  {
    id: "base",
    title: "Sistema base",
    hint: "Kernel · Arranque",
  },
  {
    id: "environment",
    title: "Entorno",
    hint: "Escritorios y gestores",
  },
  {
    id: "apps",
    title: "Apps",
    hint: "Capas y paquetes",
  },
  {
    id: "identity-security",
    title: "Identidad y seguridad",
    hint: "Hostname",
  },
  {
    id: "review",
    title: "Revisión",
    hint: "Resumen e imagen",
  },
] as const;

export const INTENTS: FaceOption[] = [
  {
    htmlId: "desarrollo",
    optionId: "archmaker.intent.desarrollo",
    stepId: STEP_IDS.intent,
    title: "Desarrollo con IA",
    description: "Editor con asistencia, herramientas base y navegación",
  },
  {
    htmlId: "estudio",
    optionId: "archmaker.intent.estudio",
    stepId: STEP_IDS.intent,
    title: "Estudio y trabajo",
    description: "Ofimática, navegador y notas",
  },
  {
    htmlId: "creativo",
    optionId: "archmaker.intent.creativo",
    stepId: STEP_IDS.intent,
    title: "Creatividad",
    description: "Imagen, ilustración y captura",
  },
  {
    htmlId: "general",
    optionId: "archmaker.intent.general",
    stepId: STEP_IDS.intent,
    title: "Uso diario",
    description: "Navegar, correo y multimedia",
  },
];

export const KERNELS: FaceOption[] = [
  {
    htmlId: "linux",
    optionId: "archmaker.kernel.linux",
    stepId: STEP_IDS.kernel,
    title: "Linux",
    version: "estable",
    description: "Última estable. Buena compatibilidad de drivers.",
    tag: "Recomendado",
  },
  {
    htmlId: "linux-lts",
    optionId: "archmaker.kernel.linux-lts",
    stepId: STEP_IDS.kernel,
    title: "Linux LTS",
    version: "largo soporte",
    description: "Soporte largo. Menos cambios de kernel.",
    tag: "Estable",
  },
  {
    htmlId: "linux-zen",
    optionId: "archmaker.kernel.zen",
    stepId: STEP_IDS.kernel,
    title: "Linux Zen",
    version: "rendimiento",
    description: "Orientado a juegos y baja latencia.",
    tag: "Gaming",
  },
  {
    htmlId: "linux-hardened",
    optionId: "archmaker.kernel.linux-hardened",
    stepId: STEP_IDS.kernel,
    title: "Linux Hardened",
    version: "seguridad",
    description: "Protecciones extra para entornos sensibles.",
    tag: "Seguridad",
  },
];

export const BOOTLOADERS: FaceOption[] = [
  {
    htmlId: "systemd-boot",
    optionId: "archmaker.boot.systemd-boot",
    stepId: STEP_IDS.boot,
    title: "systemd-boot",
    description: "UEFI nativo, entradas clave-valor.",
    tag: "UEFI",
  },
  {
    htmlId: "grub",
    optionId: "archmaker.boot.grub",
    stepId: STEP_IDS.boot,
    title: "GRUB",
    description: "BIOS y UEFI, menú clásico.",
    tag: "Clásico",
  },
  {
    htmlId: "limine",
    optionId: "archmaker.boot.limine",
    stepId: STEP_IDS.boot,
    title: "Limine",
    description: "BIOS y UEFI, configuración propia.",
    tag: "Moderno",
  },
];

export const FILESYSTEMS: FaceOption[] = [
  {
    htmlId: "btrfs",
    optionId: "archmaker.filesystem.btrfs",
    stepId: STEP_IDS.filesystem,
    title: "Btrfs",
    description: "Snapshots y compresión. Recomendado.",
    tag: "Recomendado",
  },
  {
    htmlId: "ext4",
    optionId: "archmaker.filesystem.ext4",
    stepId: STEP_IDS.filesystem,
    title: "Ext4",
    description: "Clásico, maduro y simple.",
    tag: "Clásico",
  },
];

export const ENVIRONMENTS: FaceOption[] = [
  {
    htmlId: "hyprland",
    optionId: "archmaker.compositor.hyprland",
    stepId: STEP_IDS.compositor,
    title: "Hyprland",
    version: "0.47",
    kind: "WM",
    protocol: "Wayland",
    ideal: "Usuarios avanzados que quieren lo más moderno",
    description: "Tiling dinámico con animaciones suaves",
  },
  {
    htmlId: "gnome",
    optionId: "archmaker.compositor.gnome",
    stepId: STEP_IDS.compositor,
    title: "GNOME",
    version: "50",
    kind: "DE",
    protocol: "Wayland",
    ideal: "Quienes vienen de macOS o Windows y quieren algo pulido",
    description: "Pulido con triple buffering y acentos dinámicos",
  },
  {
    htmlId: "kde",
    optionId: "archmaker.compositor.kde",
    stepId: STEP_IDS.compositor,
    title: "KDE Plasma",
    version: "6.4",
    kind: "DE",
    protocol: "Wayland",
    ideal: "Quienes quieren personalizarlo todo",
    description: "Potente, personalizable, overview mejorada",
  },
  {
    htmlId: "sway",
    optionId: "archmaker.compositor.sway",
    stepId: STEP_IDS.compositor,
    title: "Sway",
    version: "1.10",
    kind: "WM",
    protocol: "Wayland",
    ideal: "Minimalistas de teclado",
    description: "i3 en Wayland",
  },
  {
    htmlId: "cinnamon",
    optionId: "archmaker.compositor.cinnamon",
    stepId: STEP_IDS.compositor,
    title: "Cinnamon",
    version: "6.4",
    kind: "DE",
    protocol: "X11/Wayland",
    ideal: "Usuarios de Windows familiar",
    description: "Clásico y estable",
  },
  {
    htmlId: "xfce",
    optionId: "archmaker.compositor.xfce",
    stepId: STEP_IDS.compositor,
    title: "XFCE",
    version: "4.20",
    kind: "DE",
    protocol: "X11/Wayland",
    ideal: "PCs antiguos",
    description: "Ligero y rápido",
  },
  {
    htmlId: "cosmic",
    optionId: "archmaker.compositor.cosmic",
    stepId: STEP_IDS.compositor,
    title: "COSMIC",
    version: "Epoch 1.0",
    kind: "DE",
    protocol: "Wayland",
    ideal: "Fans de System76 y Rust",
    description: "Escritorio nuevo en Rust",
  },
  {
    htmlId: "budgie",
    optionId: "archmaker.compositor.budgie",
    stepId: STEP_IDS.compositor,
    title: "Budgie",
    version: "10.9.2",
    kind: "DE",
    protocol: "Wayland",
    ideal: "Elegante y simple",
    description: "Escritorio elegante y simple",
  },
  {
    htmlId: "i3",
    optionId: "archmaker.compositor.i3",
    stepId: STEP_IDS.compositor,
    title: "i3",
    version: "4.24",
    kind: "WM",
    protocol: "X11",
    ideal: "Programadores de teclado",
    description: "Tiling clásico",
  },
];

export const LAYERS: FaceLayer[] = [
  {
    id: "base",
    title: "Herramientas base",
    icon: "🧰",
    description: "Lo esencial",
    simple: true,
    packages: [
      {
        htmlId: "micro",
        optionId: "archmaker.app.micro",
        title: "Micro",
        description: "Editor fácil en terminal.",
        size: 18,
        tag: "Recomendado",
        why: "Editar sin aprender Vim",
      },
      {
        htmlId: "btop",
        optionId: "archmaker.app.btop",
        title: "btop",
        description: "Ver CPU y RAM con claridad.",
        size: 2,
        tag: "Ligero",
        why: "Saber si el equipo va lento",
      },
      {
        htmlId: "flameshot",
        optionId: "archmaker.pkg.flameshot",
        title: "Flameshot",
        description: "Capturas con anotaciones.",
        size: 25,
        tag: "Histórico",
        why: "Captura y flechas",
        verificationStatus: "verified",
      },
      {
        htmlId: "apexshot",
        optionId: "archmaker.app.apexshot",
        title: "ApexShot",
        description:
          "Capturas modernas (solicitud; proveedor Arch/AUR no verificado).",
        size: 30,
        tag: "No verificada",
        why: "Alternativa a Flameshot",
        verificationStatus: "unverified",
      },
      {
        htmlId: "kitty",
        optionId: "archmaker.app.kitty",
        title: "Kitty",
        description: "Terminal GPU acelerada.",
        size: 15,
        tag: "Terminal",
        why: "Rápida y configurable",
        verificationStatus: "verified",
      },
      {
        htmlId: "ghostty",
        optionId: "archmaker.app.ghostty",
        title: "Ghostty",
        description: "Terminal nativa moderna.",
        size: 20,
        tag: "Terminal",
        why: "Rendimiento y tipografía",
        verificationStatus: "verified",
      },
    ],
  },
  {
    id: "browser",
    title: "Navegación web",
    icon: "🌐",
    description: "Internet",
    simple: true,
    packages: [
      {
        htmlId: "firefox",
        optionId: "archmaker.browser.firefox",
        title: "Firefox",
        description: "Privado y rápido.",
        size: 250,
        tag: "Histórico",
        why: "Bloquea rastreadores",
        browserBase: "Firefox/Mozilla",
        renderingEngine: "Gecko",
        verificationStatus: "verified",
      },
      {
        htmlId: "vivaldi",
        optionId: "archmaker.browser.vivaldi",
        title: "Vivaldi",
        description: "Notas y paneles.",
        size: 380,
        tag: "Personalizable",
        why: "Control total",
        browserBase: "Chromium",
        renderingEngine: "Blink",
        verificationStatus: "verified",
      },
      {
        htmlId: "brave",
        optionId: "archmaker.browser.brave",
        title: "Brave",
        description: "Bloquea anuncios.",
        size: 320,
        tag: "Privacidad",
        why: "Menos anuncios en vídeo",
        browserBase: "Chromium",
        renderingEngine: "Blink",
        verificationStatus: "verified",
      },
      {
        htmlId: "zen",
        optionId: "archmaker.browser.zen",
        title: "Zen Browser",
        description: "Pestañas verticales modernas.",
        size: 270,
        tag: "Predeterminado",
        why: "Interfaz más limpia",
        browserBase: "Firefox",
        renderingEngine: "Gecko",
        verificationStatus: "verified",
      },
    ],
  },
  {
    id: "comm",
    title: "Comunicación",
    icon: "💬",
    description: "Chat",
    simple: true,
    packages: [
      {
        htmlId: "discord",
        optionId: "archmaker.app.discord",
        title: "Discord",
        description: "Chat de comunidades.",
        size: 180,
        tag: "Popular",
        why: "Donde está la gente",
      },
      {
        htmlId: "telegram",
        optionId: "archmaker.app.telegram",
        title: "Telegram",
        description: "Mensajería rápida.",
        size: 85,
        tag: "Popular",
        why: "Archivos grandes",
      },
    ],
  },
  {
    id: "office",
    title: "Ofimática",
    icon: "📄",
    description: "Documentos",
    simple: true,
    packages: [
      {
        htmlId: "onlyoffice",
        optionId: "archmaker.pkg.onlyoffice-bin",
        title: "ONLYOFFICE",
        description: "Compatibilidad MS Office (AUR onlyoffice-bin).",
        size: 420,
        tag: "Predeterminado",
        why: "Documentos .docx complejos",
        verificationStatus: "verified",
      },
      {
        htmlId: "libreoffice",
        optionId: "archmaker.pkg.libreoffice-fresh",
        title: "LibreOffice",
        description: "Suite ofimática libre (histórica).",
        size: 650,
        tag: "Histórico",
        why: "Abre documentos sin licencia",
        verificationStatus: "verified",
      },
    ],
  },
  {
    id: "dev",
    title: "Desarrollo",
    icon: "🧑‍💻",
    description: "Crear software",
    simple: false,
    packages: [
      {
        htmlId: "cursor",
        optionId: "archmaker.app.cursor",
        title: "Cursor",
        description:
          "Editor con asistencia (sin provider en catálogo; solicitud no verificada).",
        size: 380,
        tag: "No verificada",
        why: "Asistencia al escribir código",
        verificationStatus: "unverified",
      },
      {
        htmlId: "grokbot",
        optionId: "archmaker.app.grokbot",
        title: "Grokbot",
        description: "Asistente en escritorio (solicitud no verificada).",
        size: 180,
        tag: "No verificada",
        why: "Ayuda con el contexto del PC",
        verificationStatus: "unverified",
      },
      {
        htmlId: "grokbuild",
        optionId: "archmaker.app.grokbuild",
        title: "Grok Build",
        description: "De idea a aplicación (solicitud no verificada).",
        size: 220,
        tag: "No verificada",
        why: "Prototipar con prompts",
        verificationStatus: "unverified",
      },
      {
        htmlId: "zed",
        optionId: "archmaker.app.zed",
        title: "Zed",
        description: "Editor ultrarrápido.",
        size: 150,
        tag: "Rápido",
        why: "Proyectos grandes",
        verificationStatus: "verified",
      },
      {
        htmlId: "vscode",
        optionId: "archmaker.app.vscode",
        title: "Code - OSS",
        description:
          "Editor Code - OSS del repositorio Extra (`code`). Distinto del binario propietario de Visual Studio Code.",
        size: 350,
        tag: "Code - OSS",
        why: "Extensiones",
        verificationStatus: "verified",
      },
    ],
  },
  {
    id: "gaming",
    title: "Gaming",
    icon: "🎮",
    description: "Jugar",
    simple: false,
    packages: [
      {
        htmlId: "steam",
        optionId: "archmaker.app.steam",
        title: "Steam",
        description:
          "Biblioteca de juegos. En Arch vive en multilib; el canal del catálogo sigue en extra hasta ADR de ProviderChannel (H07 blocked).",
        size: 400,
        tag: "Gaming",
        why: "Catálogo amplio",
        verificationStatus: "verified",
      },
    ],
  },
];

export const GLOSSARY = [
  {
    title: "linux",
    body: "Kernel estable del repositorio oficial. Buena compatibilidad general.",
  },
  {
    title: "linux-lts",
    body: "Kernel de soporte largo. Menos cambios entre versiones.",
  },
  {
    title: "linux-zen",
    body: "Kernel orientado a rendimiento en escritorio y juegos.",
  },
  {
    title: "micro",
    body: "Editor de terminal sencillo, sin curva de Vim.",
  },
  {
    title: "vivaldi",
    body: "Navegador con paneles y notas integradas.",
  },
  {
    title: "cursor",
    body: "Editor con asistencia para escribir código.",
  },
] as const;

/** Initial package option ids present when the face opens (canonical where mapped). */
/** Default browser for new profiles (also selected). */
export const DEFAULT_BROWSER_ID = "archmaker.browser.zen" as const;

/** Initial package option ids for new profiles (no silent historical rewrite). */
export const INITIAL_APP_OPTION_IDS = [
  "archmaker.app.micro",
  "archmaker.browser.zen",
  "archmaker.browser.vivaldi",
  "archmaker.pkg.onlyoffice-bin",
  // Cursor sin provider: no preseleccionar como verificado (H08).
] as const;

export const INITIAL_SINGLE = {
  intent: "archmaker.intent.desarrollo",
  kernel: "archmaker.kernel.linux",
  boot: "archmaker.boot.systemd-boot",
  filesystem: "archmaker.filesystem.btrfs",
  compositor: "archmaker.compositor.gnome",
} as const;

export function findOptionById(
  optionId: string,
): FaceOption | FacePackage | undefined {
  const singles = [
    ...INTENTS,
    ...KERNELS,
    ...BOOTLOADERS,
    ...FILESYSTEMS,
    ...ENVIRONMENTS,
  ];
  const hit = singles.find((o) => o.optionId === optionId);
  if (hit) return hit;
  for (const layer of LAYERS) {
    const pkg = layer.packages.find((p) => p.optionId === optionId);
    if (pkg) return pkg;
  }
  return undefined;
}

export function allPackages(): FacePackage[] {
  return LAYERS.flatMap((l) => l.packages);
}

/** Experiencia única: catálogo completo (sin filtro Sencillo/Avanzado). */
export function allEnvironments(): FaceOption[] {
  return ENVIRONMENTS;
}

export function allLayers(): FaceLayer[] {
  return LAYERS;
}

/** Unverified requests remain selectable; they do not seal a fully-supported manifest. */
export function isUnverifiedOption(
  option: FaceOption | FacePackage | undefined,
): boolean {
  if (!option) return false;
  if (option.verificationStatus === "unverified") return true;
  return Boolean("gap" in option && option.gap);
}
