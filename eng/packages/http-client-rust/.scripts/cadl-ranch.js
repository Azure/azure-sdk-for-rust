// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License. See License.txt in the project root for license information.
import { execSync } from 'child_process';

const toolsModRoot = execSync('git rev-parse --show-toplevel').toString().trim() + '/eng/packages/http-client-rust/node_modules/@azure-tools/';

const switches = [];
let execSyncOptions;

switch (process.argv[2]) {
  case '--serve':
    switches.push('serve');
    switches.push(toolsModRoot + 'cadl-ranch-specs/http');
    execSyncOptions = {stdio: 'inherit'};
    break;
  case '--start':
    switches.push('server');
    switches.push('start');
    switches.push(toolsModRoot + 'cadl-ranch-specs/http');
    break;
  case '--stop':
    switches.push('server');
    switches.push('stop');
    break;
}

if (switches.length === 0) {
  throw new Error('missing arg: [--start] [--stop]');
}

const cmdLine = 'npx cadl-ranch ' + switches.join(' ');
console.log(cmdLine);
execSync(cmdLine, execSyncOptions);
