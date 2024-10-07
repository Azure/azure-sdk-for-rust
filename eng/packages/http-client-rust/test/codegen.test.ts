/*---------------------------------------------------------------------------------------------
*  Copyright (c) Microsoft Corporation. All rights reserved.
*  Licensed under the MIT License. See License.txt in the project root for license information.
*--------------------------------------------------------------------------------------------*/

import * as rust from '../src/codemodel/index.js';
import { CodeGenerator } from '../src/codegen/codeGenerator.js';
import * as helpers from '../src/codegen/helpers.js';
import { strictEqual } from 'assert';
import { describe, it } from 'vitest';

describe('typespec-rust: codegen', () => {
  describe('generateCargoTomlFile', () => {
    it('default Cargo.toml file', async () => {
      const expected = '[package]\n' +
        'name = "test_crate"\n' +
        'version = "1.2.3"\n' +
        'authors.workspace = true\n' +
        'edition.workspace = true\n' +
        'license.workspace = true\n' +
        'repository.workspace = true\n' +
        'rust-version.workspace = true\n';

      const codegen = new CodeGenerator(new rust.Crate('test_crate', '1.2.3', 'azure-arm'));
      const cargoToml = codegen.emitCargoToml();
      strictEqual(cargoToml, expected);
    });

    it('default Cargo.toml file with dependencies', async () => {
      const expected =   '[package]\n' +
        'name = "test_crate"\n' +
        'version = "1.2.3"\n' +
        'authors.workspace = true\n' +
        'edition.workspace = true\n' +
        'license.workspace = true\n' +
        'repository.workspace = true\n' +
        'rust-version.workspace = true\n' +
        '\n' +
        '[dependencies]\n' +
        'azure_core = { workspace = true }\n';

      const crate = new rust.Crate('test_crate', '1.2.3', 'data-plane');
      crate.dependencies.push(new rust.CrateDependency('azure_core'));
      const codegen = new CodeGenerator(crate);
      const cargoToml = codegen.emitCargoToml();
      strictEqual(cargoToml, expected);
    });
  });

  describe('helpers', () => {
    it('annotationDerive', async () => {
      strictEqual(helpers.annotationDerive(), '#[derive(Clone, Debug, Deserialize, Serialize)]\n');
      strictEqual(helpers.annotationDerive('Copy'), '#[derive(Clone, Copy, Debug, Deserialize, Serialize)]\n');
      strictEqual(helpers.annotationDerive('', 'Copy'), '#[derive(Clone, Copy, Debug, Deserialize, Serialize)]\n');
    });

    it('emitPub', async () => {
      strictEqual(helpers.emitPub(false), '');
      strictEqual(helpers.emitPub(true), 'pub ');
    });

    it('indent', async () => {
      const indentation = new helpers.indentation();
      strictEqual(indentation.get(), '    ');
      strictEqual(indentation.push().get(), '        ');
      strictEqual(indentation.push().get(), '            ');
      strictEqual(indentation.pop().get(), '        ');
      strictEqual(indentation.pop().get(), '    ');
      strictEqual(indentation.get(), '    ');
    });
  });
});
