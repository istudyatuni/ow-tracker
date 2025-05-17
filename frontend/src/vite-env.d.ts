/// <reference types="svelte" />
/// <reference types="vite/client" />
/// <reference types="unplugin-icons/types/svelte" />

interface ViteTypeOptions {
  // Disallow unknown keys.
  strictImportMetaEnv: unknown
}

interface ImportMetaEnv {
  readonly VITE_BUILD_VERSION: string;
}

interface ImportMeta {
  readonly env: ImportMetaEnv;
}
