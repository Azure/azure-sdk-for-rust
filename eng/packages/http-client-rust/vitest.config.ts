/*---------------------------------------------------------------------------------------------
 *  Copyright (c) Microsoft Corporation. All rights reserved.
 *  Licensed under the MIT License. See License.txt in the project root for license information.
 *--------------------------------------------------------------------------------------------*/

import { defineConfig } from 'vitest/config';

export default defineConfig({
  test: {
    environment: 'node',
    testTimeout: 10000,
    isolate: false,
    coverage: {
      reporter: ['cobertura', 'json', 'text'],
      reportsDirectory: './coverage/tmp'
    },
    outputFile: {
      junit: './test-results.xml',
    },
  },
});
