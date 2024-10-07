/*---------------------------------------------------------------------------------------------
 *  Copyright (c) Microsoft Corporation. All rights reserved.
 *  Licensed under the MIT License. See License.txt in the project root for license information.
 *--------------------------------------------------------------------------------------------*/

import { emitCargoToml } from './cargotoml.js';
import { ClientFiles, emitClients } from './clients.js';
import { Context } from './context.js';
import { emitEnums } from './enums.js';
import { emitLib } from './lib.js';
import { emitMod } from './mod.js';
import { emitModels } from './models.js';

import * as rust from '../codemodel/index.js';

// CodeGenerator exposes the APIs for obtaining generated code content.
export class CodeGenerator {
  private readonly context: Context;
  private readonly crate: rust.Crate;

  constructor(crate: rust.Crate) {
    this.context = new Context(crate);
    this.crate = crate;
  }

  // returns the contents for the Cargo.toml file
  emitCargoToml(): string {
    return emitCargoToml(this.crate);
  }

  // returns an array of all client files and their content.
  // if there are no clients, the array will be empty.
  emitClients(): Array<ClientFiles> {
    return emitClients(this.crate);
  }

  // returns the content for enums.rs
  // if there are no enums, the empty string is returned.
  emitEnums(): string {
    return emitEnums(this.crate, this.context);
  }

  // returns the content for lib.rs
  emitLib(): string {
    return emitLib(this.crate);
  }

  // returns the content for mod.rs
  emitMod(): string {
    return emitMod(this.crate);
  }

  // returns the content for models.rs
  // if there are no models, the empty string is returned.
  emitModels(): string {
    return emitModels(this.crate, this.context);
  }
}
