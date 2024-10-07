/*---------------------------------------------------------------------------------------------
 *  Copyright (c) Microsoft Corporation. All rights reserved.
 *  Licensed under the MIT License. See License.txt in the project root for license information.
 *--------------------------------------------------------------------------------------------*/

import * as helpers from './helpers.js';
import { Use } from './use.js';
import * as rust from '../codemodel/index.js';

// Context contains contextual information about how types are used.
// It's an implementation detail of CodeGenerator and isn't intended
// for use outside of that class.
export class Context {
  private readonly tryFromForRequestTypes = new Map<string, rust.SerdeFormat>();
  private readonly tryFromResponseTypes = new Map<string, rust.SerdeFormat>();

  constructor(crate: rust.Crate) {
    // enumerate all client methods, looking for enum and model
    // params/responses and their wire format (JSON/XML etc).
    for (const client of crate.clients) {
      for (const method of client.methods) {
        if (method.kind === 'clientaccessor') {
          continue;
        }

        // TODO: this doesn't handle the case where a method sends/receives a HashMap<T>
        // or Vec<T> where T is an enum or model type.
        // https://github.com/Azure/autorest.rust/issues/65

        for (const param of method.params) {
          if (param.kind === 'body' && (param.type.type.kind === 'enum' || param.type.type.kind === 'model')) {
            this.tryFromForRequestTypes.set(helpers.getTypeDeclaration(param.type.type), param.type.format);
          }
        }

        if (method.returns.type.kind === 'response' && (method.returns.type.type.kind === 'enum' || method.returns.type.type.kind === 'model')) {
          if (!method.returns.type.format) {
            throw new Error(`method ${client.name}.${method.name} returns a body but no format was specified`);
          }
          this.tryFromResponseTypes.set(helpers.getTypeDeclaration(method.returns.type.type), method.returns.type.format);
        }
      }
    }
  }

  // returns the TryFrom<T> for RequestContent<T> where T is type.
  // if no impl is required, it returns the empty string.
  getTryFromForRequestContent(type: rust.Type, use: Use): string {
    const format = this.tryFromForRequestTypes.get(helpers.getTypeDeclaration(type));
    if (!format) {
      return '';
    }

    use.addType('azure_core', 'RequestContent');

    const indent = new helpers.indentation();
    let content = `impl TryFrom<${helpers.getTypeDeclaration(type)}> for RequestContent<${helpers.getTypeDeclaration(type)}> {\n`;
    content += `${indent.get()}type Error = azure_core::Error;\n`;
    content += `${indent.get()}fn try_from(value: ${helpers.getTypeDeclaration(type)}) -> Result<Self, Self::Error> {\n`;
    content += `${indent.push().get()}Ok(RequestContent::from(serde_${format}::to_vec(&value)?))\n`;
    content += `${indent.pop().get()}}\n`;
    content += '}\n\n';
    return content;
  }

  // returns the TryFrom<Response<T>> for T where T is type.
  // if no impl is required, it returns the empty string.
  getTryFromResponseForType(type: rust.Type, use: Use): string {
    const format = this.tryFromResponseTypes.get(helpers.getTypeDeclaration(type));
    if (!format) {
      return '';
    }

    use.addType('azure_core', 'Response');
    use.addType('async_std::task', 'block_on');

    const indent = new helpers.indentation();
    let content = `impl TryFrom<Response<${helpers.getTypeDeclaration(type)}>> for ${helpers.getTypeDeclaration(type)} {\n`;
    content += `${indent.get()}type Error = azure_core::Error;\n`;
    content += `${indent.get()}fn try_from(value: Response<${helpers.getTypeDeclaration(type)}>) -> Result<Self, Self::Error> {\n`;
    content += `${indent.push().get()}let f = || value.into_body().${format}::<${helpers.getTypeDeclaration(type)}>();\n`;
    content += `${indent.get()}let r = block_on(f())?;\n`;
    content += `${indent.get()}Ok(r)\n`;
    content += `${indent.pop().get()}}\n`;
    content += '}\n\n';
    return content;
  }
}
