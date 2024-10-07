/*---------------------------------------------------------------------------------------------
*  Copyright (c) Microsoft Corporation. All rights reserved.
*  Licensed under the MIT License. See License.txt in the project root for license information.
*--------------------------------------------------------------------------------------------*/

import * as codegen from '@azure-tools/codegen';
import * as helpers from './helpers.js';
import { Use } from './use.js';
import * as rust from '../codemodel/index.js';

// the files and their content to emit
export interface ClientFiles {
  readonly name: string;
  readonly content: string;
}

// emits the content for all client files
export function emitClients(crate: rust.Crate): Array<ClientFiles> {
  const clientFiles = new Array<ClientFiles>();
  if (crate.clients.length === 0) {
    return clientFiles;
  }

  const clientMods = new Array<string>();

  // emit the clients, one file per client
  for (const client of crate.clients) {
    const use = new Use();
    const indentation = new helpers.indentation();

    let pubInClients = '';
    if (!client.constructable) {
      // the constructable client will need access to these fields
      pubInClients = 'pub(in crate::generated::clients) ';
    }

    let body = `pub struct ${client.name} {\n`;
    for (const field of client.fields) {
      use.addForType(field.type);
      body += `${indentation.get()}${pubInClients}${field.name}: ${helpers.getTypeDeclaration(field.type)},\n`;
    }
    use.addType('azure_core', 'Pipeline');
    body += `${indentation.get()}${pubInClients}pipeline: Pipeline,\n`;
    body += '}\n\n'; // end client

    if (client.constructable) {
      body += '#[derive(Clone, Debug)]\n';
      body += `pub struct ${client.constructable.options.type.name}`;
      if (client.constructable.options.type.fields.length > 0) {
        body += ' {\n';
        for (const field of client.constructable.options.type.fields) {
          use.addForType(field.type);
          body += `${indentation.get()}${field.name}: ${helpers.getTypeDeclaration(field.type)},\n`;
        }
        body += '}\n\n'; // end client options
      } else {
        body += ';\n\n';
      }
    }

    body += `impl ${client.name} {\n`;

    if (client.constructable) {
      // this is an instantiable client, so we need to emit client options and constructors
      use.addType('azure_core', 'Result');

      for (let i = 0; i < client.constructable.constructors.length; ++i) {
        const constructor = client.constructable.constructors[i];
        body += `${indentation.get()}pub fn ${constructor.name}(${getConstructorParamsSig(constructor.parameters, client.constructable.options, use)}) -> Result<Self> {\n`;
        // by convention, the endpoint param is always the first ctor param
        const endpointParamName = constructor.parameters[0].name;
        body += `${indentation.push().get()}let mut ${endpointParamName} = Url::parse(${endpointParamName}.as_ref())?;\n`;
        body += `${indentation.get()}${endpointParamName}.query_pairs_mut().clear();\n`;
        // if there's a credential param, create the necessary auth policy
        const authPolicy = getAuthPolicy(constructor, use);
        if (authPolicy) {
          body += `${indentation.get()}${authPolicy}\n`;
        }
        body += `${indentation.get()}let options = options.unwrap_or_default();\n`;
        body += `${indentation.get()}Ok(Self {\n`;

        // propagate any client option fields to the client initializer
        indentation.push();
        for (const field of getClientOptionsFields(client.constructable.options)) {
          body += `${indentation.get()}${field.name}: options.${field.name},\n`;
        }

        body += `${indentation.get()}${endpointParamName},\n`;
        body += `${indentation.get()}pipeline: Pipeline::new(\n`;
        body += `${indentation.push().get()}option_env!("CARGO_PKG_NAME"),\n`;
        body += `${indentation.get()}option_env!("CARGO_PKG_VERSION"),\n`;
        body += `${indentation.get()}options.client_options,\n`;
        body += `${indentation.get()}Vec::default(),\n`;
        body += `${indentation.get()}${authPolicy ? 'vec![auth_policy]' : 'Vec::default()'},\n`;
        body += `${indentation.pop().get()}),\n`; // end Pipeline::new
        body += `${indentation.pop().get()}})\n`; // end Ok
        body += `${indentation.pop().get()}}\n`; // end constructor

        // ensure extra new-line between ctors and/or client methods
        if (i + 1 < client.constructable.constructors.length || client.methods.length > 0) {
          body += '\n';
        }
      }
    }

    for (let i = 0; i < client.methods.length; ++i) {
      const method = client.methods[i];
      let returnType: string;
      let async = 'async ';
      // NOTE: when methodBody is called, the starting indentation
      // will be correct for the current scope, so there's no need
      // for the callee to indent right away.
      let methodBody: (indentation: helpers.indentation) => string;
      use.addForType(method.returns);
      switch (method.kind) {
        case 'async':
          returnType = helpers.getTypeDeclaration(method.returns);
          methodBody = (indentation: helpers.indentation): string => {
            return getAsyncMethodBody(indentation, use, client, method);
          };
          break;
        case 'clientaccessor':
          async = '';
          returnType = method.returns.name;
          methodBody = (indentation: helpers.indentation): string => {
            return getClientAccessorMethodBody(indentation, method);
          };
          break;
      }
      body += `${indentation.get()}${helpers.formatDocComment(method.docs)}`;
      body += `${indentation.get()}${helpers.emitPub(method.pub)}${async}fn ${method.name}(${getMethodParamsSig(method, use)}) -> ${returnType} {\n`;
      body += `${indentation.push().get()}${methodBody(indentation)}\n`;
      body += `${indentation.pop().get()}}\n`; // end method
      if (i + 1 < client.methods.length) {
        body += '\n';
      }
    }

    body += '}\n\n'; // end client impl

    if (client.constructable) {
      // emit the builder function for client options
      const clientOptionsType = client.constructable.options;
      const builderTypeName = getOptionsBuilderTypeName(clientOptionsType, false);
      body += `impl ${clientOptionsType.type.name} {\n`;
      body += `${indentation.get()}pub fn builder() -> builders::${builderTypeName} {\n`;
      body += `${indentation.push().get()}builders::${builderTypeName}::new()\n`;
      body += `${indentation.pop().get()}}\n`;
      body += '}\n\n'; // end impl

      // emit default trait impl for client options type
      body += `impl Default for ${clientOptionsType.type.name} {\n`;
      body += `${indentation.get()}fn default() -> Self {\n`;
      body += `${indentation.push().get()}Self {\n`;
      indentation.push();
      for (const field of clientOptionsType.type.fields) {
        if (!field.defaultValue) {
          throw new Error(`missing default value for struct field ${clientOptionsType.type.name}.${field.name}`);
        }
        body += `${indentation.get()}${field.name}: ${field.defaultValue},\n`;
      }
      body += `${indentation.pop().get()}}\n`;
      body += `${indentation.pop().get()}}\n`;
      body += '}\n\n'; // end impl
      body += createClientOptionsBuilderImpl(indentation, clientOptionsType, use);
    }

    // emit method options
    for (let i = 0; i < client.methods.length; ++i) {
      const method = client.methods[i];
      if (method.kind === 'clientaccessor') {
        continue;
      }

      body += '#[derive(Clone, Debug, Default)]\n';
      body += `${helpers.emitPub(method.pub)}struct ${helpers.getTypeDeclaration(method.options.type)} {\n`;
      for (const field of method.options.type.fields) {
        use.addForType(field.type);
        body += `${indentation.get()}${field.name}: ${helpers.getTypeDeclaration(field.type)},\n`;
      }
      body += '}\n\n'; // end options

      body += `impl${getLifetimeAnnotation(method.options.type)}${helpers.getTypeDeclaration(method.options.type)} {\n`;
      body += `${indentation.get()}pub fn builder() -> builders::${getOptionsBuilderTypeName(method.options, true)} {\n`;
      body += `${indentation.push().get()}builders::${getOptionsBuilderTypeName(method.options, false)}::new()\n`;
      body += `${indentation.pop().get()}}\n`; // end builder()
      body += '}\n'; // end options impl

      if (i + 1 < client.methods.length) {
        body += '\n';
      }
    }

    body += '\n';
    body += createPubModBuilders(client, use);

    let content = helpers.contentPreamble();
    content += use.text();
    content += body;

    const clientMod = codegen.deconstruct(client.name).join('_');
    clientFiles.push({name: `${clientMod}.rs`, content: content});
    clientMods.push(clientMod);
  }

  // now emit the mod.rs file for the clients
  let content = helpers.contentPreamble();
  const sortedMods = clientMods.sort((a: string, b: string) => { return helpers.sortAscending(a, b); });
  for (const clientMod of sortedMods) {
    content += `pub mod ${clientMod};\n`;
  }
  clientFiles.push({name: 'mod.rs', content: content});

  return clientFiles;
}

function getConstructorParamsSig(params: Array<rust.ClientParameter>, options: rust.ClientOptions, use: Use): string {
  const paramsSig = new Array<string>();
  for (const param of params) {
    use.addForType(param.type);
    paramsSig.push(`${param.name}: ${helpers.getTypeDeclaration(param.type)}`);
  }
  paramsSig.push(`options: ${helpers.getTypeDeclaration(options)}`);
  return paramsSig.join(', ');
}

function getMethodParamsSig(method: rust.MethodType, use: Use): string {
  const paramsSig = new Array<string>();
  paramsSig.push(formatParamTypeName(method.self));
  for (const param of method.params) {
    if (param.type.kind === 'literal') {
      // literal params are embedded directly in the code (e.g. accept header param)
      continue;
    }

    // don't add client params to the method param sig
    if (method.kind !== 'clientaccessor' && (<rust.MethodParameter>param).location === 'method') {
      use.addForType(param.type);
      paramsSig.push(`${param.name}: ${formatParamTypeName(param)}`);
    }
  }
  if (method.kind !== 'clientaccessor') {
    paramsSig.push(`options: ${helpers.getTypeDeclaration(method.options, true)}`);
  }
  return paramsSig.join(', ');
}

// creates the auth policy if the ctor contains a credential param.
// the policy will be named auth_policy.
function getAuthPolicy(ctor: rust.Constructor, use: Use): string | undefined {
  for (const param of ctor.parameters) {
    if (param.type.kind === 'arc' && param.type.type.kind === 'tokenCredential') {
      use.addType('azure_core', 'BearerTokenCredentialPolicy');
      const scopes = new Array<string>();
      for (const scope of param.type.type.scopes) {
        scopes.push(`"${scope}"`);
      }
      return `let auth_policy: Arc<dyn Policy> = Arc::new(BearerTokenCredentialPolicy::new(credential, vec![${scopes.join(', ')}]));`;
    }
  }
  return undefined;
}

function formatParamTypeName(param: rust.Parameter | rust.Self): string {
  let format = '';
  if (param.ref) {
    format = '&';
  }
  if (param.mut) {
    format += 'mut ';
  }
  if ((<rust.Parameter>param).type) {
    format += helpers.getTypeDeclaration((<rust.Parameter>param).type);
  } else {
    format += param.name;
  }
  return format;
}

function createClientOptionsBuilderImpl(indentation: helpers.indentation, options: rust.ClientOptions, use: Use): string {
  use.addType('azure_core::builders', 'ClientOptionsBuilder');
  use.addTypes('azure_core', ['Policy', 'RetryOptions', /*'TelemetryOptions', */'TransportOptions']);
  use.addType('std::sync', 'Arc');

  const emitWithMethod = function(indentation: helpers.indentation, name: string, into: string): string {
    let withPolicies = `${indentation.get()}fn with_${name}<P>(mut self, ${name}: P) -> Self `;
    withPolicies += `where P: Into<${into}>, Self: Sized {\n`;
    withPolicies += `${indentation.push().get()}self.client_options.set_${name}(${name});\n`;
    withPolicies += `${indentation.get()}self\n`;
    withPolicies += `${indentation.pop().get()}}\n\n`;
    return withPolicies;
  };

  let implBuilder = `impl ClientOptionsBuilder for ${options.type.name} {\n`;
  implBuilder += emitWithMethod(indentation, 'per_call_policies', 'Vec<Arc<dyn Policy>>');
  implBuilder += emitWithMethod(indentation, 'per_try_policies', 'Vec<Arc<dyn Policy>>');
  implBuilder += emitWithMethod(indentation, 'retry', 'RetryOptions');
  // TODO: https://github.com/Azure/azure-sdk-for-rust/issues/1753
  //implBuilder += emitWithMethod(indentation, 'telemetry', 'TelemetryOptions');
  implBuilder += emitWithMethod(indentation, 'transport', 'TransportOptions');
  implBuilder += '}\n\n';
  return implBuilder;
}

function createPubModBuilders(client: rust.Client, use: Use): string {
  const indentation = new helpers.indentation();

  let body = 'pub mod builders {\n';
  body += `${indentation.get()}use super::*;\n\n`;

  const emitNewFunction = function(indentation: helpers.indentation, options: rust.ClientOptions | rust.MethodOptions): string {
    let newFunction = `${indentation.push().get()}pub(super) fn new() -> Self {\n`;
    newFunction += `${indentation.push().get()}Self {\n`;
    newFunction += `${indentation.push().get()}options: ${options.type.name}::default(),\n`;
    newFunction += `${indentation.pop().get()}}\n`;
    newFunction += `${indentation.pop().get()}}\n\n`;
    return newFunction;
  };

  const emitBuildMethod = function(indentation: helpers.indentation, options: rust.ClientOptions | rust.MethodOptions): string {
    let buildMethod = `${indentation.get()}pub fn build(&self) -> ${options.type.name} {\n`;
    buildMethod += `${indentation.push().get()}self.options.clone()\n`;
    buildMethod += `${indentation.pop().get()}}\n`;
    return buildMethod;
  };

  if (client.constructable) {
    // emit the builder type for client options
    const clientOptionsType = client.constructable.options;
    const builderTypeName = getOptionsBuilderTypeName(clientOptionsType, false);
    body += `${indentation.get()}pub struct ${builderTypeName} {\n`;
    body += `${indentation.push().get()}options: ${clientOptionsType.type.name},\n`;
    body += `${indentation.pop().get()}}\n\n`; // end struct
    body += `${indentation.get()}impl ${builderTypeName} {\n`;
    body += emitNewFunction(indentation, clientOptionsType);
    body += emitBuildMethod(indentation, clientOptionsType);
    for (const field of getClientOptionsFields(client.constructable.options)) {
      let typeName = helpers.getTypeDeclaration(field.type);
      let into = '';
      if (field.type.kind === 'String') {
        // for Strings, we define these as "impl Into<String>" so that passing a str will just work
        typeName = `impl Into<${typeName}>`;
        into = '.into()';
      }
      body += `\n${indentation.get()}pub fn with_${field.name}(mut self, ${field.name}: ${typeName}) -> Self {\n`;
      body += `${indentation.push().get()}self.options.${field.name} = ${field.name}${into};\n`;
      body += `${indentation.get()}self\n`;
      body += `${indentation.pop().get()}}\n`;
    }
    body += `${indentation.pop().get()}}\n\n`; // end impl
  }

  // emit the client method options builders
  for (let i = 0; i < client.methods.length; ++i) {
    const method = client.methods[i];
    if (method.kind === 'clientaccessor') {
      continue;
    }

    use.addType('azure_core', 'Context');
    use.addType('azure_core::builders', 'ClientMethodOptionsBuilder');

    const optionsBuilderTypeName = getOptionsBuilderTypeName(method.options, true);

    body += `${indentation.get()}pub struct ${optionsBuilderTypeName} {\n`;
    body += `${indentation.push().get()}options: ${helpers.getTypeDeclaration(method.options.type)},\n`;
    body += `${indentation.pop().get()}}\n\n`; // end struct

    body += `${indentation.get()}impl ${getOptionsBuilderTypeName(method.options, 'anonymous')} {\n`;
    body += emitNewFunction(indentation, method.options);
    body += emitBuildMethod(indentation, method.options);
    body += `${indentation.pop().get()}}\n\n`; // end impl

    body += `${indentation.get()}impl${getLifetimeAnnotation(method.options.type)}ClientMethodOptionsBuilder${getLifetimeAnnotation(method.options.type)}for ${optionsBuilderTypeName} {\n`;

    body += `${indentation.push().get()}fn with_context(mut self, context: &${getLifetimeName(method.options.type)}Context) -> Self {\n`;
    body += `${indentation.push().get()}self.options.${getClientMethodOptionsFieldName(method.options)}.set_context(context);\n`;
    body += `${indentation.get()}self\n`;
    body += `${indentation.pop().get()}}\n`; // end with_context

    body += `${indentation.pop().get()}}\n`; // end ClientMethodOptionsBuilder impl

    if (i + 1 < client.methods.length) {
      body += '\n';
    }
  }

  body += '}\n'; // end pub mod builders
  return body;
}

function getOptionsBuilderTypeName(option: rust.ClientOptions | rust.MethodOptions, withLifetime: true | false | 'anonymous'): string {
  if (!withLifetime || !option.type.lifetime) {
    return `${option.type.name}Builder`;
  } else if (withLifetime === 'anonymous') {
    return `${option.type.name}Builder${helpers.AnonymousLifetimeAnnotation}`;  
  }
  return `${option.type.name}Builder${helpers.getGenericLifetimeAnnotation(option.type.lifetime)}`;
}

// returns a filtered array of struct fields from the client options types
function getClientOptionsFields(option: rust.ClientOptions): Array<rust.StructField> {
  const fields = new Array<rust.StructField>();
  for (const field of option.type.fields) {
    if (helpers.getTypeDeclaration(field.type) === 'ClientOptions') {
      // azure_core::ClientOptions is passed to Pipeline::new so skip it
      continue;
    }
    fields.push(field);
  }
  return fields;
}

function getClientMethodOptionsFieldName(option: rust.MethodOptions): string {
  for (const field of option.type.fields) {
    // startsWith to ignore possible lifetime annotation suffix
    if (helpers.getTypeDeclaration(field.type).startsWith('ClientMethodOptions')) {
      return field.name;
    }
  }
  throw new Error(`didn't find ClientMethodOptions field in ${option.type.name}`);
}

function getLifetimeAnnotation(type: rust.Struct): string {
  if (type.lifetime) {
    return `${helpers.getGenericLifetimeAnnotation(type.lifetime)} `;
  }
  return ' ';
}

function getLifetimeName(type: rust.Struct): string {
  if (type.lifetime) {
    return `${type.lifetime.name} `;
  }
  return ' ';
}

function getEndpointFieldName(client: rust.Client): string {
  // find the endpoint field. it's the only one that's
  // a Url. the name will be uniform across clients
  let endpointFieldName: string | undefined;
  for (const field of client.fields) {
    if (field.type.kind === 'Url' ) {
      endpointFieldName = field.name;
    } else if (endpointFieldName) {
      throw new Error(`found multiple URL fields in client ${client.name} which is unexpected`);
    }
  }
  if (!endpointFieldName) {
    throw new Error(`didn't find URI field for client ${client.name}`);
  }
  return endpointFieldName;
}

function getClientAccessorMethodBody(indentation: helpers.indentation, clientAccessor: rust.ClientAccessor): string {
  let body = `${clientAccessor.returns.name} {\n`;
  const endpointFieldName = getEndpointFieldName(clientAccessor.returns);
  body += `${indentation.push().get()}${endpointFieldName}: self.${endpointFieldName}.clone(),\n`;
  body += `${indentation.get()}pipeline: self.pipeline.clone(),\n`;
  body += `${indentation.pop().get()}}`;
  return body;
}

type HeaderParamType = rust.HeaderCollectionParameter | rust.HeaderParameter;
type QueryParamType = rust.QueryCollectionParameter | rust.QueryParameter;

function getAsyncMethodBody(indentation: helpers.indentation, use: Use, client: rust.Client, method: rust.AsyncMethod): string {
  use.addTypes('azure_core', ['AsClientMethodOptions', 'Method', 'Request']);
  let body = 'let options = options.unwrap_or_default();\n';
  body += `${indentation.get()}let mut ctx = options.method_options.context();\n`;
  body += `${indentation.get()}let mut url = self.${getEndpointFieldName(client)}.clone();\n`;

  // collect and sort all the header/path/query params
  const headerParams = new Array<HeaderParamType>();
  const pathParams = new Array<rust.PathParameter>();
  const queryParams = new Array<QueryParamType>();
  for (const param of method.params) {
    switch (param.kind) {
      case 'header':
      case 'headerCollection':
        headerParams.push(param);
        break;
      case 'path':
        pathParams.push(param);
        break;
      case 'query':
      case 'queryCollection':
        queryParams.push(param);
        break;
    }
  }
  headerParams.sort((a: HeaderParamType, b: HeaderParamType) => { return helpers.sortAscending(a.header, b.header); });
  pathParams.sort((a: rust.PathParameter, b: rust.PathParameter) => { return helpers.sortAscending(a.segment, b.segment); });
  queryParams.sort((a: QueryParamType, b: QueryParamType) => { return helpers.sortAscending(a.key, b.key); });

  let path = `"${method.httpPath}"`;
  if (pathParams.length > 0) {
    // we have path params that need to have their segments replaced with the param values
    body += `${indentation.get()}let mut path = String::from(${path});\n`;
    for (const pathParam of pathParams) {
      body += `${indentation.get()}path = path.replace("{${pathParam.segment}}", &${getHeaderPathQueryParamValue(pathParam)});\n`;
    }
    path = '&path';
  }

  body += `${indentation.get()}url.set_path(${path});\n`;

  for (const queryParam of queryParams) {
    if (queryParam.kind === 'queryCollection' && queryParam.format === 'multi') {
      const valueVar = queryParam.name[0];
      body += `${indentation.get()}for ${valueVar} in ${queryParam.name}.iter() {\n`;
      body += `${indentation.push().get()}url.query_pairs_mut().append_pair("${queryParam.key}", ${valueVar});\n`;
      body += `${indentation.pop().get()}}\n`;
    } else {
      body += `${indentation.get()}url.query_pairs_mut().append_pair("${queryParam.key}", &${getHeaderPathQueryParamValue(queryParam)});\n`;
    }
  }

  body += `${indentation.get()}let mut request = Request::new(url, Method::${codegen.capitalize(method.httpMethod)});\n`;

  for (const headerParam of headerParams) {
    body += `${indentation.get()}request.insert_header("${headerParam.header.toLowerCase()}", ${getHeaderPathQueryParamValue(headerParam)});\n`;
  }

  const bodyParam = getBodyParameter(method);
  if (bodyParam) {
    body += `${indentation.get()}request.set_body(${bodyParam.name});\n`;
  }

  body += `${indentation.get()}self.pipeline.send(&mut ctx, &mut request).await\n`;
  return body;
}

function getBodyParameter(method: rust.AsyncMethod): rust.BodyParameter | undefined {
  let bodyParam: rust.BodyParameter | undefined;
  for (const param of method.params) {
    if (param.kind === 'body') {
      if (bodyParam) {
        throw new Error(`method ${method.name} has multiple body parameters`);
      }
      bodyParam = param;
    }
  }
  return bodyParam;
}

function getHeaderPathQueryParamValue(param: HeaderParamType | rust.PathParameter | QueryParamType): string {
  let paramName = '';
  if (param.location === 'client') {
    paramName = 'self.';
  }

  if (param.kind === 'headerCollection' || param.kind === 'queryCollection') {
    if (param.format === 'multi') {
      throw new Error('multi should have been handled outside getHeaderPathQueryParamValue');
    }
    return `${param.name}.join("${getCollectionDelimiter(param.format)}")`;
  }

  switch (param.type.kind) {
    case 'String':
      paramName += param.name;
      break;
    case 'implTrait':
      // only done for method params so no need to include paramName prefix
      return `${param.name}.into()`;
    case 'literal':
      return `"${param.type.value}"`;
    default:
      throw new Error(`unhandled ${param.kind} param type kind ${param.type.kind}`);
  }

  return paramName;
}

function getCollectionDelimiter(format: rust.CollectionFormat): string {
  switch (format) {
    case 'csv':
      return ',';
    case 'pipes':
      return '|';
    case 'ssv':
      return ' ';
    case 'tsv':
      return '\t';
    default:
      throw new Error(`unhandled collection format ${format}`);
  }
}
