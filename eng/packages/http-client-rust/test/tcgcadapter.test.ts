/*---------------------------------------------------------------------------------------------
*  Copyright (c) Microsoft Corporation. All rights reserved.
*  Licensed under the MIT License. See License.txt in the project root for license information.
*--------------------------------------------------------------------------------------------*/

import * as rust from '../src/codemodel/index.js';
import * as helpers from '../src/tcgcadapter/helpers.js';
import { deepEqual, strictEqual } from 'assert';
import { describe, it } from 'vitest';

describe('typespec-rust: tcgcadapter', () => {
  describe('helpers', () => {
    it('fixUpEnumValueName', async () => {
      strictEqual(helpers.fixUpEnumValueName('fooBar'), 'FooBar');
      strictEqual(helpers.fixUpEnumValueName('foo_bar'), 'FooBar');
      strictEqual(helpers.fixUpEnumValueName('V2022_12_01_preview'), 'V2022_12_01Preview');
      strictEqual(helpers.fixUpEnumValueName('V7.6_preview.1'), 'V7Dot6Preview1');
    });

    it('sortClientParameters', async () => {
      const endpointParam = new rust.ClientParameter('endpoint', new rust.StringType());
      const credentialParam = new rust.ClientParameter('credential', new rust.StringType());
      const someOtherParam = new rust.ClientParameter('something', new rust.StringType());

      let params = new Array<rust.ClientParameter>(endpointParam, credentialParam, someOtherParam);
      helpers.sortClientParameters(params);
      deepEqual(params, [endpointParam, credentialParam, someOtherParam]);

      params = new Array<rust.ClientParameter>(credentialParam, endpointParam, someOtherParam);
      helpers.sortClientParameters(params);
      deepEqual(params, [endpointParam, credentialParam, someOtherParam]);

      params = new Array<rust.ClientParameter>(someOtherParam, credentialParam, endpointParam);
      helpers.sortClientParameters(params);
      deepEqual(params, [endpointParam, credentialParam, someOtherParam]);

      params = new Array<rust.ClientParameter>(endpointParam, credentialParam);
      helpers.sortClientParameters(params);
      deepEqual(params, [endpointParam, credentialParam]);

      params = new Array<rust.ClientParameter>(credentialParam, endpointParam);
      helpers.sortClientParameters(params);
      deepEqual(params, [endpointParam, credentialParam]);

      params = new Array<rust.ClientParameter>(endpointParam, someOtherParam);
      helpers.sortClientParameters(params);
      deepEqual(params, [endpointParam, someOtherParam]);

      params = new Array<rust.ClientParameter>(someOtherParam, endpointParam);
      helpers.sortClientParameters(params);
      deepEqual(params, [endpointParam, someOtherParam]);
    });
  });
});
