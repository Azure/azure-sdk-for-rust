// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

import react from "@vitejs/plugin-react";
import { defineConfig } from "vite";

// `bun run dev` serves the UI with hot reload. Start `bun run serve` alongside
// it to browse results through the API; otherwise use the folder picker.
export default defineConfig({
    base: "./",
    plugins: [react()],
    server: {
        proxy: {
            "/api": process.env.COSSPEC_VIEWER_API ?? "http://127.0.0.1:4173",
        },
    },
});
