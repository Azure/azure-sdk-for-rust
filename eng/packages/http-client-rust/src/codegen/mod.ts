/*---------------------------------------------------------------------------------------------
*  Copyright (c) Microsoft Corporation. All rights reserved.
*  Licensed under the MIT License. See License.txt in the project root for license information.
*--------------------------------------------------------------------------------------------*/

import * as helpers from './helpers.js';
import * as rust from '../codemodel/index.js';

// emits the mod.rs file
export function emitMod(crate: rust.Crate): string {
  let content = helpers.contentPreamble();
  if (crate.clients.length > 0) {
    content += 'pub mod clients;\n';
  }
  if (crate.enums.length > 0) {
    content += 'pub mod enums;\n';
  }
  if (crate.models.length > 0) {
    content += 'pub mod models;\n';
  }
  return content;
}
