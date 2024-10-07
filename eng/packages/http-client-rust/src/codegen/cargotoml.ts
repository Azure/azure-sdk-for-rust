/*---------------------------------------------------------------------------------------------
*  Copyright (c) Microsoft Corporation. All rights reserved.
*  Licensed under the MIT License. See License.txt in the project root for license information.
*--------------------------------------------------------------------------------------------*/

import * as rust from '../codemodel/index.js';

// emits the Cargo.toml file for this crate
export function emitCargoToml(crate: rust.Crate): string {
  let content = `[package]\nname = "${crate.name}"\nversion = "${crate.version}"\n`;
  content += 'authors.workspace = true\n';
  content += 'edition.workspace = true\n';
  content += 'license.workspace = true\n';
  content += 'repository.workspace = true\n';
  content += 'rust-version.workspace = true\n';
  if (crate.dependencies.length > 0) {
    content += '\n[dependencies]\n';
    for (const dependency of crate.dependencies) {
      // dependency versions are managed by the workspace's Cargo.toml file
      content += `${dependency.name} = { workspace = true }\n`;
    }
  }
  return content;
}
