import globals from "globals";
import pluginJs from "@eslint/js";
import pluginReact from "eslint-plugin-react";

export default [
  {
    files: ["**/*.{js,mjs,cjs,jsx}"],
    languageOptions: {
      globals: {
        ...globals.browser,
        self: "readonly",       // Allow 'self'
        globalThis: "readonly", // Allow 'globalThis'
      },
    },
    extends: [
      pluginJs.configs.recommended,
      pluginReact.configs.flat.recommended,
    ],
  },
];
