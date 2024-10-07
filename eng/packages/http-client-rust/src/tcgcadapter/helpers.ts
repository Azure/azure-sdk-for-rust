/*---------------------------------------------------------------------------------------------
*  Copyright (c) Microsoft Corporation. All rights reserved.
*  Licensed under the MIT License. See License.txt in the project root for license information.
*--------------------------------------------------------------------------------------------*/

import * as codegen from '@azure-tools/codegen';
import * as rust from '../codemodel/index.js';

// fixes up enum names to follow Rust conventions
export function fixUpEnumValueName(name: string): string {
  name = codegen.capitalize(name);

  // first replace any '.' chars between numbers with the word 'Dot'
  // any '.' between a letter and a numer will be removed.
  // e.g. V7.6_preview.1 becomes V7Dot6_preview1
  const numDotNumMatch = name.match(/(\d+\.\d+)/);
  if (numDotNumMatch) {
    name = name.replace(numDotNumMatch[0], numDotNumMatch[0].replace('.', 'Dot'));
  }

  const wordDotNumMatch = name.match(/\w+\.\d+/);
  if (wordDotNumMatch) {
    name = name.replace(wordDotNumMatch[0], wordDotNumMatch[0].replace('.', ''));
  }

  // if we have a name like V2022_12_01_preview, we want to
  // turn this into V2022_12_01Preview to make the linter happy
  const parts = name.split('_');
  if (parts.length > 1) {
    name = '';
    for (let i = 0; i < parts.length; ++i) {
      parts[i] = codegen.capitalize(parts[i]);
      name += parts[i];
      if (i + 1 < parts.length && parts[i + 1].match(/^\d/)) {
        name += '_';
      }
    }
  }

  return name;
}

// sorts client params so they're in the order, endpoint, [credential], other
export function sortClientParameters(params: Array<rust.ClientParameter>): void {
  params.sort((a: rust.ClientParameter, b: rust.ClientParameter): number => {
    if (a.name === 'endpoint' || (a.name === 'credential' && b.name !== 'endpoint')) {
      // endpoint always comes first, followed by credential (if applicable)
      return -1;
    }
    return 0;
  });
}
