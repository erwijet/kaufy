/// <reference types="vite/client" />

interface ImportMetaEnv {
  readonly VITE_GOOGLE_OAUTH_CLIENT_ID: string;
  readonly VITE_GOOGLE_OAUTH_SECRET: string;
  readonly VITE_GOOGLE_OAUTH_REDIRECT_URI: string;
  readonly VITE_API_BASE_URL: string;
}

interface ImportMeta {
    readonly env: ImportMetaEnv
}
