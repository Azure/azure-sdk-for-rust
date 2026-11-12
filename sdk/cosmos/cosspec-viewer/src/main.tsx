// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import { App } from "./ui/App";
import "./ui/styles.css";

createRoot(document.getElementById("root")!).render(
    <StrictMode>
        <App />
    </StrictMode>,
);
