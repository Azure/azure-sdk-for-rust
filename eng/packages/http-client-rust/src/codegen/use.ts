/*---------------------------------------------------------------------------------------------
*  Copyright (c) Microsoft Corporation. All rights reserved.
*  Licensed under the MIT License. See License.txt in the project root for license information.
*--------------------------------------------------------------------------------------------*/

import * as codegen from '@azure-tools/codegen';
import * as helpers from './helpers.js';
import * as rust from '../codemodel/index.js';

// used to generate use statements
export class Use {
  private uses: Array<moduleTypes>;
  private scope?: 'models';

  // scope indicates a scope in which use statements are constructed.
  // e.g. 'models' indicates we're "in" the crate::models scope so there's
  // no need to add a use statement for types in crate::models
  // no scope will add all using statements as required.
  constructor(scope?: 'models') {
    this.uses = new Array<moduleTypes>();
    this.scope = scope;
  }

  // adds the specified module and type if not already in the list
  // e.g. ('azure_core', 'Context') or ('crate::models', 'FooType')
  addType(module: string, type: string): void {
    let mod = this.uses.find((v: moduleTypes, i: number, o: Array<moduleTypes>) => { return v.module === module; });
    if (!mod) {
      mod = {
        module: module,
        types: new Array<string>(),
      };
      this.uses.push(mod);
    }
    if (!mod.types.find((v: string, i: number, o: Array<string>) => { return v === type; })) {
      mod.types.push(type);
    }
  }

  // adds the specified module and types if not already in the list
  addTypes(module: string, types: Array<string>): void {
    if (types.length === 0) {
      throw new Error('types can\'t be empty');
    }
    for (const type of types) {
      this.addType(module, type);
    }
  }

  // adds the specified type if not already in the list
  addForType(type: rust.Client | rust.Type): void {
    switch (type.kind) {
      case 'arc':
        return this.addForType(type.type);
      case 'client': {
        const mod = codegen.deconstruct(type.name).join('_');
        this.addType(`crate::${mod}`, type.name);
        break;
      }
      case 'enum':
        this.addType('crate::models', type.name);
        break;
      case 'model':
        if (this.scope !== 'models') {
          this.addType('crate::models', type.name);
        }
        break;
      case 'option':
      case 'requestContent':
      case 'response':
      case 'result':
      case 'hashmap':
      case 'vector':
        this.addForType(type.type);
        break;
    }

    if (type.kind !== 'client') {
      if ((<rust.StdType>type).name !== undefined && (<rust.StdType>type).use !== undefined) {
        this.addType((<rust.StdType>type).use, (<rust.StdType>type).name);
      } else if ((<rust.External>type).crate !== undefined && (<rust.External>type).name !== undefined) {
        let module = (<rust.External>type).crate;
        if ((<rust.External>type).namespace) {
          module += `::${(<rust.External>type).namespace}`;
        }
        this.addType(module, (<rust.External>type).name);
      }
    }
  }

  // returns Rust formatted use statements
  text(): string {
    if (this.uses.length === 0) {
      return '';
    }

    let content = '';

    const indentation = new helpers.indentation();

    // sort by module name, then sort types if more than one type
    const sortedMods = this.uses.sort((a: moduleTypes, b: moduleTypes) => { return helpers.sortAscending(a.module, b.module); });
    for (const sortedMod of sortedMods) {
      if (sortedMod.types.length === 1) {
        content += `use ${sortedMod.module}::${sortedMod.types[0]};\n`;
      } else {
        const sortedTypes = sortedMod.types.sort((a: string, b: string) => { return helpers.sortAscending(a, b); });
        content += `use ${sortedMod.module}::{\n`;
        content += `${indentation.get()}${sortedTypes.join(', ')}`;
        content += ',\n};\n';
      }
    }

    content += '\n';
    return content;
  }
}

interface moduleTypes {
  module: string;
  types: Array<string>;
}
