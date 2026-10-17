/** Metadatos canónicos del build (inyectados por Vite desde package.json). */
declare const __FORGE_APP_VERSION__: string;

export const APP_VERSION: string =
  typeof __FORGE_APP_VERSION__ === "string" && __FORGE_APP_VERSION__.length > 0
    ? __FORGE_APP_VERSION__
    : "0.0.0-dev";

export const PRODUCT_WINDOW_TITLE = "ArchMaker Forge";
