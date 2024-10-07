/*---------------------------------------------------------------------------------------------
*  Copyright (c) Microsoft Corporation. All rights reserved.
*  Licensed under the MIT License. See License.txt in the project root for license information.
*--------------------------------------------------------------------------------------------*/

import * as method from './method.js';
import * as types from './types.js';

// Client is a SDK client
export interface Client {
  kind: 'client';

  // the name of the client
  name: string;

  // any docs for the client
  docs: types.Docs;

  // contains info for instantiable clients
  constructable?: ClientConstruction;

  // fields contains the ctor parameters that are
  // persisted as fields on the client type. note that
  // not all ctor params might be persisted, and not
  // all fields are ctor params.
  fields: Array<ClientParameter>;

  // all the methods for this client
  methods: Array<MethodType>;

  // the parent client in a hierarchical client
  parent?: Client;
}

// ClientConstruction contains data for instantiable clients.
export interface ClientConstruction {
  // the client options type used in the constructors
  options: ClientOptions;

  // the constructor functions for a client.
  constructors: Array<Constructor>;
}

// ClientOptions is the struct containing optional client params
export interface ClientOptions extends types.Option {
  type: types.Struct;
}

// represents a client constructor function
export interface Constructor {
  name: string;

  // the modeled parameters. at minimum, an endpoint param
  parameters: Array<ClientParameter>;
}

// ClientParameter is a Rust client parameter
export interface ClientParameter {
  // the name of the parameter
  name: string;

  // the type of the client parameter
  type: types.Type;
}

///////////////////////////////////////////////////////////////////////////////////////////////////
// methods
///////////////////////////////////////////////////////////////////////////////////////////////////

// HTTPMethod defines the possible HTTP verbs in a request
export type HTTPMethod = 'delete' | 'get' | 'head' | 'patch' | 'post' | 'put';

// Method defines the possible method types
export type MethodType = AsyncMethod | ClientAccessor;

// AsyncMethod is an async Rust method
export interface AsyncMethod extends HTTPMethodBase {
  kind: 'async';

  // the params passed to the method (excluding self). can be empty
  params: Array<MethodParameter>;
}

// ClientAccessor is a method that returns a sub-client instance.
export interface ClientAccessor extends method.Method<Client> {
  kind: 'clientaccessor';

  // the client returned by the accessor method
  returns: Client;
}

///////////////////////////////////////////////////////////////////////////////////////////////////
// parameters
///////////////////////////////////////////////////////////////////////////////////////////////////

// CollectionFormat indicates how a collection is formatted on the wire
export type CollectionFormat = 'csv' | 'ssv' | 'tsv' | 'pipes';

// ExtendedCollectionFormat includes additional formats
export type ExtendedCollectionFormat = CollectionFormat | 'multi';

// ParameterLocation indicates where the value of the param originates
export type ParameterLocation = 'client' | 'method';

// MethodParameter defines the possible method parameter types
export type MethodParameter = BodyParameter | HeaderCollectionParameter | HeaderParameter | PathParameter | QueryCollectionParameter | QueryParameter;

// BodyParameter is a param that's passed via the HTTP request body
export interface BodyParameter extends HTTPParameterBase {
  kind: 'body';

  // the type of the body param
  type: types.RequestContent;
}

// HeaderCollectionParameter is a param that goes in a HTTP header
export interface HeaderCollectionParameter extends HTTPParameterBase {
  kind: 'headerCollection';

  // the header in the HTTP request
  header: string;

  // the collection of header param values
  type: types.Vector;

  // the format of the collection
  format: CollectionFormat;
}

// HeaderParameter is a param that goes in a HTTP header
export interface HeaderParameter extends HTTPParameterBase {
  kind: 'header';

  // the header in the HTTP request
  header: string;

  // the type of the header param
  // note that not all types are applicable
  type: types.Type;
}

// MethodOptions is the struct containing optional method params
export interface MethodOptions extends types.Option {
  type: types.Struct;
}

// PathParameter is a param that goes in the HTTP path
export interface PathParameter extends HTTPParameterBase {
  kind: 'path';

  // the segment name to be replaced with the param's value
  segment: string;

  // the type of the path param
  // note that not all types are applicable
  type: types.Type;

  // indicates if the path parameter should be URL encoded
  encoded: boolean;
}

// QueryCollectionParameter is a param that goes in the HTTP query string
export interface QueryCollectionParameter extends HTTPParameterBase {
  kind: 'queryCollection';

  // key is the query param's key name
  key: string;

  // the collection of query param values
  type: types.Vector;

  // indicates if the query parameter should be URL encoded
  encoded: boolean;

  // the format of the collection
  format: ExtendedCollectionFormat;
}

// QueryParameter is a param that goes in the HTTP query string
export interface QueryParameter extends HTTPParameterBase {
  kind: 'query';

  // key is the query param's key name
  key: string;

  // the type of the query param
  // note that not all types are applicable
  type: types.Type;

  // indicates if the query parameter should be URL encoded
  encoded: boolean;
}

///////////////////////////////////////////////////////////////////////////////////////////////////
// base types
///////////////////////////////////////////////////////////////////////////////////////////////////

interface HTTPMethodBase extends method.Method<types.Type> {
  // the params passed to the method (excluding self). can be empty
  params: Array<HTTPParameterBase>;

  // the method options type for this method
  options: MethodOptions;

  // the type returned by the method
  returns: types.Result;

  // the HTTP verb used for the request
  httpMethod: HTTPMethod;

  // the HTTP path for the request
  httpPath: string;
}

interface HTTPParameterBase extends method.Parameter {
  location: ParameterLocation;
}

class HTTPMethodBase extends method.Method<types.Type> implements HTTPMethodBase {
  constructor(name: string, httpMethod: HTTPMethod, httpPath: string, pub: boolean, impl: string, self: method.Self) {
    super(name, pub, impl, self);
    this.httpMethod = httpMethod;
    this.httpPath = httpPath;
    this.docs = {};
  }
}

class HTTPParameterBase extends method.Parameter {
  constructor(name: string, location: ParameterLocation, type: types.Type) {
    super(name, type);
    this.location = location;
    this.docs = {};
  }
}

///////////////////////////////////////////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////////////////

export class AsyncMethod extends HTTPMethodBase implements AsyncMethod {
  constructor(name: string, client: Client, pub: boolean, options: MethodOptions, httpMethod: HTTPMethod, httpPath: string) {
    super(name, httpMethod, httpPath, pub, client.name, new method.Self(false, true));
    this.kind = 'async';
    this.params = new Array<MethodParameter>();
    this.options = options;
  }
}

export class BodyParameter extends HTTPParameterBase implements BodyParameter {
  constructor(name: string, location: ParameterLocation, type: types.RequestContent) {
    super(name, location, type);
    this.kind = 'body';
  }
}

export class Client implements Client {
  constructor(name: string) {
    this.kind = 'client';
    this.name = name;
    this.fields = new Array<ClientParameter>();
    this.methods = new Array<MethodType>();
    this.docs = {};
  }
}

export class ClientAccessor extends method.Method<Client> implements ClientAccessor {
  constructor(name: string, client: Client, returns: Client) {
    super(name, true, client.name, new method.Self(false, true));
    this.kind = 'clientaccessor';
    this.params = new Array<MethodParameter>();
    this.returns = returns;
  }
}

export class ClientConstruction implements ClientConstruction {
  constructor(options: ClientOptions) {
    this.options = options;
    this.constructors = new Array<Constructor>();
  }
}

export class ClientOptions extends types.Option implements ClientOptions {
  constructor(type: types.Struct) {
    super(type, false);
  }
}

export class ClientParameter implements ClientParameter {
  constructor(name: string, type: types.Type) {
    this.name = name;
    this.type = type;
  }
}

export class Constructor implements Constructor {
  constructor(name: string) {
    this.name = name;
    this.parameters = new Array<ClientParameter>();
  }
}

export class HeaderCollectionParameter extends HTTPParameterBase implements HeaderCollectionParameter {
  constructor(name: string, header: string, location: ParameterLocation, type: types.Vector, format: CollectionFormat) {
    validateHeaderPathQueryParamKind(type, 'headerCollection');
    super(name, location, type);
    this.kind = 'headerCollection';
    this.header = header;
    this.format = format;
  }
}

export class HeaderParameter extends HTTPParameterBase implements HeaderParameter {
  constructor(name: string, header: string, location: ParameterLocation, type: types.Type) {
    validateHeaderPathQueryParamKind(type, 'header');
    super(name, location, type);
    this.kind = 'header';
    this.header = header;
  }
}

export class MethodOptions extends types.Option implements MethodOptions {
  constructor(type: types.Struct, ref: boolean) {
    super(type, ref);
  }
}

export class PathParameter extends HTTPParameterBase implements PathParameter {
  constructor(name: string, segment: string, location: ParameterLocation, type: types.Type, encoded: boolean) {
    validateHeaderPathQueryParamKind(type, 'path');
    super(name, location, type);
    this.kind = 'path';
    this.segment = segment;
    this.encoded = encoded;
  }
}

export class QueryCollectionParameter extends HTTPParameterBase implements QueryCollectionParameter {
  constructor(name: string, key: string, location: ParameterLocation, type: types.Vector, encoded: boolean, format: ExtendedCollectionFormat) {
    validateHeaderPathQueryParamKind(type.type, 'queryCollection');
    super(name, location, type);
    this.kind = 'queryCollection';
    this.key = key;
    this.encoded = encoded;
    this.format = format;
  }
}

export class QueryParameter extends HTTPParameterBase implements QueryParameter {
  constructor(name: string, key: string, location: ParameterLocation, type: types.Type, encoded: boolean) {
    validateHeaderPathQueryParamKind(type, 'query');
    super(name, location, type);
    this.kind = 'query';
    this.key = key;
    this.encoded = encoded;
  }
}

function validateHeaderPathQueryParamKind(type: types.Type, paramKind: string) {
  switch (type.kind) {
    case 'String':
    case 'enum':
    case 'literal':
    case 'offsetDateTime':
    case 'scalar':
      return;
    case 'implTrait':
      validateHeaderPathQueryParamKind(type.type, paramKind);
      return;
    case 'vector':
      if (paramKind.endsWith('Collection')) {
        return;
      }
  }
  throw new Error(`unsupported ${paramKind} paramter type kind ${type.kind}`);
}
