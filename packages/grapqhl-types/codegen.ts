import type { CodegenConfig } from "@graphql-codegen/cli";

const config: CodegenConfig = {
  schema: "http://localhost:4000",
  documents: ["src/graphql/**/*.graphql"],
  ignoreNoDocuments: true,
  overwrite: true,
  generates: {
    "./src/generated/graphql.ts": {
      plugins: ["typescript", "typescript-urql", "typescript-operations"],
    },
  },
};

export default config;
