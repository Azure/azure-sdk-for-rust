/*---------------------------------------------------------------------------------------------
*  Copyright (c) Microsoft Corporation. All rights reserved.
*  Licensed under the MIT License. See License.txt in the project root for license information.
*--------------------------------------------------------------------------------------------*/

import { Context } from './context.js';
import * as helpers from './helpers.js';
import { Use } from './use.js';
import * as rust from '../codemodel/index.js';

// emits the enums.rs file for this crate
export function emitEnums(crate: rust.Crate, context: Context): string {
  if (crate.enums.length === 0) {
    return '';
  }

  const use = new Use('models');
  use.addTypes('serde', ['Deserialize', 'Serialize']);

  const indentation = new helpers.indentation();

  let body = '';
  for (const rustEnum of crate.enums) {
    body += helpers.formatDocComment(rustEnum.docs);
    // only derive Copy for fixed enums
    body += helpers.annotationDerive(!rustEnum.extensible ? 'Copy' : '', 'Eq', 'PartialEq');
    body += helpers.AnnotationNonExhaustive;
    body += `${helpers.emitPub(rustEnum.pub)}enum ${rustEnum.name} {\n`;

    for (const value of rustEnum.values) {
      body += helpers.formatDocComment(value.docs);
      if (value.name !== value.value) {
        // only emit the serde annotation when the names aren't equal
        body += `${indentation.get()}#[serde(rename = "${value.value}")]\n`;
      }
      body += `${indentation.get()}${value.name},\n`;
    }

    if (rustEnum.extensible) {
      body += `${indentation.get()}#[serde(untagged)]\n`;
      // TODO: hard-coded String type
      // https://github.com/Azure/autorest.rust/issues/25
      body += `${indentation.get()}UnknownValue(String),\n`;
    }
    body += '}\n\n';
  }

  // emit TryFrom as required
  for (const rustEnum of crate.enums) {
    body += context.getTryFromForRequestContent(rustEnum, use);
    body += context.getTryFromResponseForType(rustEnum, use);
  }

  let content = helpers.contentPreamble();
  content += use.text();
  content += body;

  return content;
}
