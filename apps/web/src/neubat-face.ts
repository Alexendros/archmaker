/**
 * Presentation data for the NEUBAT FORGE face.
 * Canonical ids map to local-catalog-mvp1 where they exist; others are face gaps.
 */

export type EnvKind = "DE" | "WM";

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
  gap?: boolean;
};

export type FacePackage = {
  htmlId: string;
  optionId: string;
  title: string;
  description: string;
  size: number;
  tag: string;
  why: string;
  gap?: boolean;
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
    id: "intent",
    title: "Intención",
    hintSimple: "¿Para qué usarás el PC?",
    hintAdvanced: "¿Para qué usarás el PC?",
  },
  {
    id: "base",
    title: "Sistema base",
    hintSimple: "Kernel · Resumen",
    hintAdvanced: "Kernel · Detallado",
  },
  {
    id: "environment",
    title: "Entorno",
    hintSimple: "Solo escritorios",
    hintAdvanced: "Escritorios y gestores",
  },
  {
    id: "apps",
    title: "Apps",
    hintSimple: "4 capas esenciales",
    hintAdvanced: "Todas las capas",
  },
  {
    id: "identity",
    title: "Identidad",
    hintSimple: "Usuario y cifrado",
    hintAdvanced: "Usuario y cifrado",
  },
  {
    id: "review",
    title: "Revisión",
    hintSimple: "Resumen y forja",
    hintAdvanced: "Resumen y forja",
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
    gap: true,
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
    gap: true,
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
    gap: true,
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
    gap: true,
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
    gap: true,
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
    gap: true,
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
        tag: "Popular",
        why: "Captura y flechas",
        gap: true,
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
        tag: "Recomendado",
        why: "Bloquea rastreadores",
      },
      {
        htmlId: "vivaldi",
        optionId: "archmaker.browser.vivaldi",
        title: "Vivaldi",
        description: "Notas y paneles.",
        size: 380,
        tag: "Personalizable",
        why: "Control total",
      },
      {
        htmlId: "brave",
        optionId: "archmaker.browser.brave",
        title: "Brave",
        description: "Bloquea anuncios.",
        size: 320,
        tag: "Privacidad",
        why: "Menos anuncios en vídeo",
      },
      {
        htmlId: "zen",
        optionId: "archmaker.browser.zen",
        title: "Zen Browser",
        description: "Pestañas verticales modernas.",
        size: 270,
        tag: "Moderno",
        why: "Interfaz más limpia",
        gap: true,
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
        htmlId: "libreoffice",
        optionId: "archmaker.pkg.libreoffice-fresh",
        title: "LibreOffice",
        description: "Suite ofimática libre.",
        size: 650,
        tag: "Recomendado",
        why: "Abre documentos sin licencia",
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
        description: "Editor con asistencia.",
        size: 380,
        tag: "Recomendado",
        why: "Asistencia al escribir código",
      },
      {
        htmlId: "grokbot",
        optionId: "archmaker.app.grokbot",
        title: "Grokbot",
        description: "Asistente en escritorio.",
        size: 180,
        tag: "IA",
        why: "Ayuda con el contexto del PC",
        gap: true,
      },
      {
        htmlId: "grokbuild",
        optionId: "archmaker.app.grokbuild",
        title: "Grok Build",
        description: "De idea a aplicación.",
        size: 220,
        tag: "IA",
        why: "Prototipar con prompts",
        gap: true,
      },
      {
        htmlId: "zed",
        optionId: "archmaker.app.zed",
        title: "Zed",
        description: "Editor ultrarrápido.",
        size: 150,
        tag: "Rápido",
        why: "Proyectos grandes",
        gap: true,
      },
      {
        htmlId: "vscode",
        optionId: "archmaker.app.vscode",
        title: "VS Code",
        description: "Editor ampliamente usado.",
        size: 350,
        tag: "Popular",
        why: "Extensiones",
        gap: true,
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
        description: "Biblioteca de juegos.",
        size: 400,
        tag: "Gaming",
        why: "Catálogo amplio",
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
  {
    title: "Modo Sencillo / Avanzado",
    body: "Sencillo oculta kernels detallados, gestores técnicos y capas de desarrollo o gaming. Avanzado muestra todas las opciones. Lo ya elegido no se borra al cambiar de modo.",
  },
] as const;

/** Initial package option ids present when the face opens (canonical where mapped). */
export const INITIAL_APP_OPTION_IDS = [
  "archmaker.app.micro",
  "archmaker.browser.firefox",
  "archmaker.browser.vivaldi",
  "archmaker.pkg.libreoffice-fresh",
  "archmaker.app.cursor",
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

export function filteredEnvironments(simple: boolean): FaceOption[] {
  return simple ? ENVIRONMENTS.filter((e) => e.kind === "DE") : ENVIRONMENTS;
}

export function filteredLayers(simple: boolean): FaceLayer[] {
  return simple ? LAYERS.filter((l) => l.simple) : LAYERS;
}
