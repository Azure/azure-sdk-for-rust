/*---------------------------------------------------------------------------------------------
*  Copyright (c) Microsoft Corporation. All rights reserved.
*  Licensed under the MIT License. See License.txt in the project root for license information.
*--------------------------------------------------------------------------------------------*/

import * as types from './types.js';

// Method is a Rust method
export interface Method<T> {
  // the name of the method
  name: string;

  // any docs for the method
  docs: types.Docs;

  // indicates if the method should be public
  pub: boolean;

  // the name of the type on which the method is implemented
  impl: string;

  // self contains info about the self param
  self: Self;

  // the params passed to the method (excluding self). can be empty
  params: Array<Parameter>;

  // the method's return type
  returns?: T;
}

// Parameter is a Rust function or method parameter
export interface Parameter {
  // the name of the parameter
  name: string;

  // any docs for the parameter
  docs: types.Docs;

  // the parameter's type
  type: types.Type;

  // indicates if the parameter is mutable. defaults to false
  mut: boolean;

  // indicates if the parameter is a reference. defaults to false
  ref: boolean;
}

// Self is a method's self parameter
export interface Self {
  name: 'self';

  // indicates if self is mutable
  mut: boolean;

  // indicates if self is a reference
  ref: boolean;
}

///////////////////////////////////////////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////////////////

export class Method<T> implements Method<T> {
  constructor(name: string, pub: boolean, impl: string, self: Self) {
    this.name = name;
    this.pub = pub;
    this.impl = impl;
    this.self = self;
    this.params = new Array<Parameter>();
    this.docs = {};
  }
}

export class Parameter implements Parameter {
  constructor(name: string, type: types.Type) {
    this.name = name;
    this.type = type;
    this.mut = false;
    this.ref = false;
    this.docs = {};
  }
}

export class Self implements Self {
  constructor(mut: boolean, ref: boolean) {
    this.name = 'self';
    this.mut = mut;
    this.ref = ref;
  }
}
