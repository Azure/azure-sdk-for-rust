/*---------------------------------------------------------------------------------------------
*  Copyright (c) Microsoft Corporation. All rights reserved.
*  Licensed under the MIT License. See License.txt in the project root for license information.
*--------------------------------------------------------------------------------------------*/

import * as client from './client.js';
import * as types from './types.js';

// Crate is a Rust crate
// the Rust edition is centrally managed
export interface Crate {
  // the name of the Crate
  name: string;

  // the version of the Crate
  version: string;

  // the target service type
  type: ServiceType;

  // the Crates on which this Crate depends
  dependencies: Array<CrateDependency>;

  // enums contains all of the enums for this crate. can be empty
  enums: Array<types.Enum>;

  // models contains all of the models for this crate. can be empty
  models: Array<types.Model>;

  // clients contains all the clients for this crate. can be empty
  clients: Array<client.Client>;
}

// ServiceType defines the possible service types
export type ServiceType = 'azure-arm' | 'data-plane';

// CrateDependency is an external Crate dependency
// note that dependency versions are centralized which is
// why there's no version info specified here.
export interface CrateDependency {
  // the name of the Crate
  name: string;
}

///////////////////////////////////////////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////////////////

export class Crate implements Crate {
  constructor(name: string, version: string, type: ServiceType) {
    this.name = name;
    this.version = version;
    this.type = type;
    this.dependencies = new Array<CrateDependency>();
    this.enums = new Array<types.Enum>();
    this.models = new Array<types.Model>();
    this.clients = new Array<client.Client>();
  }

  addDependency(dependency: CrateDependency): void {
    for (const dep of this.dependencies) {
      if (dep.name === dependency.name) {
        return;
      }
    }
    this.dependencies.push(dependency);
  }

  sortContent(): void {
    const sortAscending = function(a: string, b: string): number {
      return a < b ? -1 : a > b ? 1 : 0;
    };

    this.dependencies.sort((a: CrateDependency, b: CrateDependency) => { return sortAscending(a.name, b.name); });
    this.enums.sort((a: types.Enum, b: types.Enum) => { return sortAscending(a.name, b.name); });
    for (const rustEnum of this.enums) {
      rustEnum.values.sort((a: types.EnumValue, b: types.EnumValue) => { return sortAscending(a.name, b.name); });
    }
    this.models.sort((a: types.Model, b: types.Model) => { return sortAscending(a.name, b.name); });
    for (const model of this.models) {
      model.fields.sort((a: types.ModelField, b: types.ModelField) => { return sortAscending(a.name, b.name); });
    }
    this.clients.sort((a: client.Client, b: client.Client) => { return sortAscending(a.name, b.name); });
    for (const client of this.clients) {
      client.fields.sort((a: client.ClientParameter, b: client.ClientParameter) => { return sortAscending(a.name, b.name); });
      client.methods.sort((a: client.MethodType, b: client.MethodType) => { return sortAscending(a.name, b.name); });
      if (client.constructable) {
        client.constructable.options.type.fields.sort((a: types.StructField, b: types.StructField) => { return sortAscending(a.name, b.name); });
      }
    }
  }
}

export class CrateDependency implements CrateDependency {
  constructor(name: string) {
    this.name = name;
  }
}
