import { CodegenConfig } from "@graphql-codegen/cli";

const config: CodegenConfig = {
  schema: [
    {
      "http://localhost:8000/graphql": {
        headers: {
          Authorization:
            "Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJlbWFpbCI6InR5bGVyLmhvbGV3aW5za2lAYnJ5eC5jb20iLCJnaXZlbl9uYW1lIjoiVHlsZXIiLCJmYW1pbHlfbmFtZSI6IkhvbGV3aW5za2kiLCJwaWN0dXJlIjoiaHR0cHM6Ly9saDMuZ29vZ2xldXNlcmNvbnRlbnQuY29tL2EvQUNnOG9jSW1MLW5kSm9DT0gtbHdUYmNLWHZPZ0JZenJOeTFXeHVtSVFsNkdoWUVqPXM5Ni1jIiwiaWQiOjEsImV4cCI6MTY5OTIxMzY2Mn0.jwix_TljtJBvPMl6z15jfONvtcW13PtHSurnL3nuOJQ",
        },
      },
    },
  ],
  documents: ["src/**/*.tsx"],
  ignoreNoDocuments: true, // for better experience with the watcher
  generates: {
    "./src/gql/": {
      preset: "client",
    },
  },
};

export default config;
