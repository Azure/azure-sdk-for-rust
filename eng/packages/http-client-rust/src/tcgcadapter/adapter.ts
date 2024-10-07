/*---------------------------------------------------------------------------------------------
*  Copyright (c) Microsoft Corporation. All rights reserved.
*  Licensed under the MIT License. See License.txt in the project root for license information.
*--------------------------------------------------------------------------------------------*/

import * as codegen from '@azure-tools/codegen';
import { values } from '@azure-tools/linq';
import { EmitContext } from '@typespec/compiler';
import * as helpers from './helpers.js';
import * as naming from './naming.js';
import { RustEmitterOptions } from '../lib.js';
import * as tcgc from '@azure-tools/typespec-client-generator-core';
import * as rust from '../codemodel/index.js';

// Adapter converts the tcgc code model to a Rust Crate
export class Adapter {
  static async create(context: EmitContext<RustEmitterOptions>): Promise<Adapter> {
    const ctx = await tcgc.createSdkContext(context);
    return new Adapter(ctx, context.options);
  }

  private readonly crate: rust.Crate;
  private readonly ctx: tcgc.SdkContext;

  // cache of adapted types
  private readonly types: Map<string, rust.Type>;

  // cache of adapted client method params
  private readonly clientMethodParams: Map<string, rust.MethodParameter>;

  private constructor(ctx: tcgc.SdkContext, options: RustEmitterOptions) {
    this.types = new Map<string, rust.Type>();
    this.clientMethodParams = new Map<string, rust.MethodParameter>();
    this.ctx = ctx;

    let serviceType: rust.ServiceType = 'data-plane';
    if (this.ctx.arm === true) {
      serviceType = 'azure-arm';
    }

    this.crate = new rust.Crate(options['crate-name'], options['crate-version'], serviceType);
  }

  // performs all the steps to convert tcgc to a crate
  tcgcToCrate(): rust.Crate {
    this.adaptTypes();
    this.adaptClients();

    if (this.crate.enums.length > 0 || this.crate.models.length > 0) {
      this.crate.addDependency(new rust.CrateDependency('serde'));
    }

    if (this.crate.clients.length > 0) {
      this.crate.addDependency(new rust.CrateDependency('async-std'));
    }

    this.crate.sortContent();
    return this.crate;
  }

  // converts all tcgc types to their Rust type equivalent
  private adaptTypes(): void {
    for (const sdkEnum of this.ctx.sdkPackage.enums) {
      if ((sdkEnum.usage & tcgc.UsageFlags.ApiVersionEnum) === tcgc.UsageFlags.ApiVersionEnum) {
        // we skip generating the enums for API
        // versions as we expose it as a String
        continue;
      }
      const rustEnum = this.getEnum(sdkEnum);
      this.crate.enums.push(rustEnum);
    }

    for (const model of this.ctx.sdkPackage.models) {
      if (model.usage === tcgc.UsageFlags.Error) {
        // if the model is purely used for errors then
        // skip it as we use azure_core::Error.
        continue;
      }
      const rustModel = this.getModel(model);
      this.crate.models.push(rustModel);
    }
  }

  // converts a tcgc enum to a Rust enum
  private getEnum(sdkEnum: tcgc.SdkEnumType): rust.Enum {
    const enumName = codegen.capitalize(sdkEnum.name);
    let rustEnum = this.types.get(enumName);
    if (rustEnum) {
      return <rust.Enum>rustEnum;
    }

    // first create all of the enum values
    const values = new Array<rust.EnumValue>();
    for (const value of sdkEnum.values) {
      const rustEnumValue = new rust.EnumValue(helpers.fixUpEnumValueName(value.name), value.value);
      values.push(rustEnumValue);
    }

    rustEnum = new rust.Enum(enumName, isPub(sdkEnum.access), values, !sdkEnum.isFixed);
    this.types.set(enumName, rustEnum);

    return rustEnum;
  }

  // converts a tcgc model to a Rust model
  private getModel(model: tcgc.SdkModelType): rust.Model {
    if (model.name.length === 0) {
      throw new Error('unnamed model'); // TODO: this might no longer be an issue
    }
    const modelName = codegen.capitalize(model.name);
    let rustModel = this.types.get(modelName);
    if (rustModel) {
      return <rust.Model>rustModel;
    }
    rustModel = new rust.Model(modelName, isPub(model.access));
    rustModel.docs.summary = model.summary;
    rustModel.docs.description = model.doc;
    this.types.set(modelName, rustModel);

    for (const property of model.properties) {
      if (property.kind !== 'property') {
        // TODO: https://github.com/Azure/autorest.rust/issues/96
        throw new Error(`model property kind ${property.kind} NYI`);
      }
      const structField = this.getModelField(property);
      rustModel.fields.push(structField);
    }

    return rustModel;
  }

  // converts a tcgc model property to a model field
  private getModelField(property: tcgc.SdkBodyModelPropertyType): rust.ModelField {
    const fieldType = new rust.Option(this.getType(property.type), false);
    const modelField = new rust.ModelField(naming.getEscapedReservedName(snakeCaseName(property.name), 'prop'), property.serializedName, true, fieldType);
    modelField.docs.summary = property.summary;
    modelField.docs.description = property.doc;
    return modelField;
  }

  // converts a tcgc type to a Rust type
  private getType(type: tcgc.SdkType): rust.Type {
    const getScalar = (kind: 'boolean' | 'float32' | 'float64' | 'int16' | 'int32' | 'int64' | 'int8'): rust.Scalar => {
      let scalar = this.types.get(kind);
      if (scalar) {
        return <rust.Scalar>scalar;
      }

      let scalarType: rust.ScalarType;
      switch (kind) {
        case 'boolean':
          scalarType = 'bool';
          break;
        case 'float32':
          scalarType = 'f32';
          break;
        case 'float64':
          scalarType = 'f64';
          break;
        case 'int16':
          scalarType = 'i16';
          break;
        case 'int32':
          scalarType = 'i32';
          break;
        case 'int64':
          scalarType = 'i64';
          break;
        case 'int8':
          scalarType = 'i8';
          break;
      }

      scalar = new rust.Scalar(scalarType);
      this.types.set(kind, scalar);
      return scalar;
    };

    switch (type.kind) {
      case 'array': {
        const keyName = recursiveKeyName(type.kind, type.valueType);
        let vectorType = this.types.get(keyName);
        if (vectorType) {
          return vectorType;
        }
        vectorType = new rust.Vector(this.getType(type.valueType));
        this.types.set(keyName, vectorType);
        return vectorType;
      }
      case 'bytes': {
        let encoding: rust.BytesEncoding = 'std';
        if (type.encode === 'base64url') {
          encoding = 'url';
        }
        const keyName = `encodedBytes-${encoding}`;
        let encodedBytesType = this.types.get(keyName);
        if (encodedBytesType) {
          return encodedBytesType;
        }
        encodedBytesType = new rust.EncodedBytes(encoding);
        this.types.set(keyName, encodedBytesType);
        return encodedBytesType;
      }
      case 'constant':
        return this.getLiteral(type);
      case 'dict': {
        const keyName = recursiveKeyName(type.kind, type.valueType);
        let hashmapType = this.types.get(keyName);
        if (hashmapType) {
          return hashmapType;
        }
        hashmapType = new rust.HashMap(this.getType(type.valueType));
        this.types.set(keyName, hashmapType);
        return hashmapType;
      }
      case 'boolean':
      case 'float32':
      case 'float64':
      case 'int16':
      case 'int32':
      case 'int64':
      case 'int8':
        return getScalar(type.kind);
      case 'enum':
        return this.getEnum(type);
      case 'model':
        return this.getModel(type);
      case 'endpoint':
      case 'string': {
        let stringType = this.types.get(type.kind);
        if (stringType) {
          return stringType;
        }
        stringType = new rust.StringType();
        this.types.set(type.kind, stringType);
        return stringType;
      }
      case 'unknown': {
        let anyType = this.types.get(type.kind);
        if (anyType) {
          return anyType;
        }
        anyType = new rust.JsonValue(this.crate);
        this.types.set(type.kind, anyType);
        return anyType;
      }
      case 'url': {
        let urlType = this.types.get(type.kind);
        if (urlType) {
          return urlType;
        }
        urlType = new rust.Url(this.crate);
        this.types.set(type.kind, urlType);
        return urlType;
      }
      case 'utcDateTime': {
        const keyName = `${type.kind}-${type.encode}`;
        let timeType = this.types.get(keyName);
        if (timeType) {
          return timeType;
        }
        timeType = new rust.OffsetDateTime(this.crate, type.encode);
        this.types.set(keyName, timeType);
        return timeType;
      }
      default:
        throw new Error(`unhandled tcgc type ${type.kind}`);
    }
  }

  private getUnitType(): rust.Unit {
    const typeKey = 'rust-unit';
    let unitType = this.types.get(typeKey);
    if (unitType) {
      return <rust.Unit>unitType;
    }
    unitType = new rust.Unit();
    this.types.set(typeKey, unitType);
    return unitType;
  }

  private getLiteral(constType: tcgc.SdkConstantType): rust.Literal {
    const literalKey = `literal-${constType.value}`;
    let literalType = this.types.get(literalKey);
    if (literalType) {
      return <rust.Literal>literalType;
    }
    literalType = new rust.Literal(constType.value);
    this.types.set(literalKey, literalType);
    return literalType;
  }

  // converts all tcgc clients and their methods into Rust clients/methods
  private adaptClients(): void {
    for (const client of this.ctx.sdkPackage.clients) {
      if (client.methods.length === 0) {
        // skip generating empty clients
        continue;
      }

      // start with instantiable clients and recursively work down
      if (client.initialization.access === 'public') {
        this.recursiveAdaptClient(client);
      }
    }
  }

  // recursively adapts a client and its methods.
  // this simplifies the case for hierarchical clients.
  private recursiveAdaptClient(client: tcgc.SdkClientType<tcgc.SdkHttpOperation>, parent?: rust.Client): rust.Client {
    let clientName = client.name;
    if (parent) {
      // for hierarchical clients, the child client names are built
      // from the parent client name. this is because tsp allows subclients
      // with the same name. consider the following example.
      //
      // namespace Chat {
      //   interface Completions {
      //     ...
      //   }
      // }
      // interface Completions { ... }
      //
      // we want to generate two clients from this,
      // one name ChatCompletions and the other Completions

      // strip off the Client suffix from the parent client name
      clientName = parent.name.substring(0, parent.name.length - 6) + clientName;
    }

    // only add the Client suffix to instantiable clients
    if (!clientName.match(/Client$/) && !parent) {
      clientName += 'Client';
    }

    const rustClient = new rust.Client(clientName);
    rustClient.docs.summary = client.summary;
    rustClient.docs.description = client.doc;
    rustClient.parent = parent;

    // anything other than public means non-instantiable client
    if (client.initialization.access === 'public') {
      const clientOptionsStruct = new rust.Struct(`${rustClient.name}Options`, true);
      const clientOptionsField = new rust.StructField('client_options', false, new rust.ExternalType(this.crate, 'azure_core', 'ClientOptions'));
      clientOptionsField.defaultValue = 'ClientOptions::default()';
      clientOptionsStruct.fields.push(clientOptionsField);
      rustClient.constructable = new rust.ClientConstruction(new rust.ClientOptions(clientOptionsStruct));

      // NOTE: per tcgc convention, if there is no param of kind credential
      // it means that the client doesn't require any kind of authentication.
      // HOWEVER, if there *is* a credential param, then the client *does not*
      // automatically support unauthenticated requests. a credential with
      // the noAuth scheme indicates support for unauthenticated requests.

      // bit flags for auth types
      enum AuthTypes {
        Default = 0, // unspecified
        NoAuth  = 1, // explicit NoAuth
        WithAut = 2, // explicit credential
      }

      let authType = AuthTypes.Default;

      const ctorParams = new Array<rust.ClientParameter>();
      for (const param of client.initialization.properties) {
        switch (param.kind) {
          case 'credential':
            switch (param.type.kind) {
              case 'credential':
                switch (param.type.scheme.type) {
                  case 'noAuth':
                    authType |= AuthTypes.NoAuth;
                    break;
                  case 'oauth2': {
                    authType |= AuthTypes.WithAut;
                    if (param.type.scheme.flows.length === 0) {
                      throw new Error(`no flows defined for credential type ${param.type.scheme.type}`);
                    }
                    const scopes = new Array<string>();
                    for (const scope of param.type.scheme.flows[0].scopes) {
                      scopes.push(scope.value);
                    }
                    const ctorTokenCredential = new rust.Constructor('new');
                    ctorTokenCredential.parameters.push(new rust.ClientParameter('credential', new rust.Arc(new rust.TokenCredential(this.crate, scopes))));
                    rustClient.constructable.constructors.push(ctorTokenCredential);
                    break;
                  }
                  default:
                    // TODO: https://github.com/Azure/autorest.rust/issues/57
                    throw new Error(`credential scheme type ${param.type.scheme.type} NYI`);
                }
                break;
              case 'union':
                // TODO: https://github.com/Azure/autorest.rust/issues/57
                throw new Error('credential unions NYI');
            }
            break;
          case 'endpoint':
            // for Rust, we always require a complete endpoint param, templated
            // endpoints, e.g. https://{something}.contoso.com isn't supported.
            // note that the types of the param and the field are slightly different
            ctorParams.push(new rust.ClientParameter(param.name, new rust.ImplTrait('AsRef', new rust.StringSlice())));
            rustClient.fields.push(new rust.ClientParameter(param.name, new rust.Url(this.crate)));
            break;
          case 'method': {
            // this is a client param that's used in method bodies (e.g. api-version but can be others)
            if (!param.isApiVersionParam) {
              // TODO: https://github.com/Azure/autorest.rust/issues/90
              throw new Error('client method params other than api-version NYI');
            }

            if (!param.clientDefaultValue) {
              // TODO: https://github.com/Azure/autorest.rust/issues/90
              throw new Error('required client method params NYI');
            }

            // we expose the api-version param as a String
            const paramName = snakeCaseName(param.name);
            rustClient.fields.push(new rust.ClientParameter(paramName, new rust.StringType()));

            // client-side default value makes the param optional
            const apiVersionField = new rust.StructField(paramName, false, new rust.StringType());
            apiVersionField.defaultValue = `String::from("${param.clientDefaultValue}")`;
            clientOptionsStruct.fields.push(apiVersionField);
            break;
          }
        }
      }

      if (authType === AuthTypes.Default || (authType & AuthTypes.NoAuth) === AuthTypes.NoAuth) {
        const ctorWithNoCredential = new rust.Constructor('with_no_credential');
        rustClient.constructable.constructors.push(ctorWithNoCredential);
      }

      // propagate ctor params to all client ctors
      for (const constructor of rustClient.constructable.constructors) {
        constructor.parameters.push(...ctorParams);
        // ensure param order of endpoint, credential, other
        helpers.sortClientParameters(constructor.parameters);
      }
    } else if (parent) {
      // this is a sub-client. it will share the fields of the parent.
      // NOTE: we must propagate parant params before a potential recursive call
      // to create a child client that will need to inherit our client params.
      rustClient.fields = parent.fields;
    } else {
      throw new Error(`uninstantiable client ${client.name} has no parent`);
    }

    for (const method of client.methods) {
      if (method.kind === 'clientaccessor') {
        const subClient = this.recursiveAdaptClient(method.response, rustClient);
        rustClient.methods.push(new rust.ClientAccessor(`get_${snakeCaseName(subClient.name)}_client`, rustClient, subClient));
      } else {
        this.adaptMethod(method, rustClient);
      }
    }

    this.crate.clients.push(rustClient);
    return rustClient;
  }

  // converts method into a rust.Method for the specified rust.Client
  private adaptMethod(method: tcgc.SdkServiceMethod<tcgc.SdkHttpOperation>, rustClient: rust.Client): void {
    let rustMethod: rust.AsyncMethod;
    const optionsLifetime = new rust.Lifetime('a');
    const methodOptionsStruct = new rust.Struct(`${rustClient.name}${codegen.pascalCase(method.name)}Options`, true);
    methodOptionsStruct.lifetime = optionsLifetime;

    const clientMethodOptions = new rust.ExternalType(this.crate, 'azure_core', 'ClientMethodOptions');
    clientMethodOptions.lifetime = optionsLifetime;
    methodOptionsStruct.fields.push(new rust.StructField('method_options', false, clientMethodOptions));

    const httpMethod = method.operation.verb;
    const httpPath = method.operation.path;

    switch (method.kind) {
      case 'basic':
        rustMethod = new rust.AsyncMethod(naming.getEscapedReservedName(snakeCaseName(method.name), 'fn'), rustClient, isPub(method.access), new rust.MethodOptions(methodOptionsStruct, false), httpMethod, httpPath);
        break;
      case 'paging':
        // TODO: https://github.com/Azure/autorest.rust/issues/60
        return;
      default:
        throw new Error(`method kind ${method.kind} NYI`);
    }

    rustMethod.docs.summary = method.summary;
    rustMethod.docs.description = method.doc;
    rustClient.methods.push(rustMethod);

    // stuff all of the operation parameters into one array for easy traversal
    let allOpParams = new Array<OperationParamType>();
    allOpParams.push(...method.operation.parameters);
    if (method.operation.bodyParam) {
      allOpParams.push(method.operation.bodyParam);
    }

    for (const param of method.parameters) {
      // we need to translate from the method param to its underlying operation param.
      // most params have a one-to-one mapping. however, for spread params, there will
      // be a many-to-one mapping. i.e. multiple params will map to the same underlying
      // operation param. each param corresponds to a field within the operation param.
      const opParam = values(allOpParams).where((opParam: OperationParamType) => {
        return values(opParam.correspondingMethodParams).where((methodParam: tcgc.SdkModelPropertyType) => {
          return methodParam.name === param.name;
        }).any();
      }).first();

      if (!opParam) {
        throw new Error(`didn't find operation parameter for method ${method.name} parameter ${param.name}`);
      }

      let adaptedParam: rust.MethodParameter;
      if (opParam.kind === 'body' && opParam.type.kind === 'model' && opParam.type.kind !== param.type.kind) {
        throw new Error('spread params NYI');
      } else {
        adaptedParam = this.adaptMethodParameter(opParam);
      }

      adaptedParam.docs.summary = param.summary;
      adaptedParam.docs.description = param.doc;
      rustMethod.params.push(adaptedParam);

      // remove the opParam we just processed
      allOpParams = allOpParams.filter((v: OperationParamType) => {
        return v !== opParam;
      });
    }

    // client params aren't included in method.parameters so
    // look for them in the remaining operation parameters.
    for (const opParam of allOpParams) {
      if (opParam.onClient) {
        const adaptedParam = this.adaptMethodParameter(opParam);
        adaptedParam.docs.summary = opParam.summary;
        adaptedParam.docs.description = opParam.doc;
        rustMethod.params.push(adaptedParam);
      }
    }

    let returnType: rust.Type;
    if (method.response.type) {
      // search the HTTP responses for the corresponding type so we can determine the wire format
      let format: rust.SerdeFormat | undefined;
      for (const httpResp of method.operation.responses.values()) {
        if (!httpResp.type || !httpResp.defaultContentType || httpResp.type.kind !== method.response.type.kind) {
          continue;
        }
        format = this.adaptSerdeFormat(httpResp.defaultContentType);
        break;
      }
      if (!format) {
        throw new Error(`didn't find HTTP response for kind ${method.response.type.kind} in method ${method.name}`);
      }
      returnType = new rust.Response(this.crate, this.getType(method.response.type), format);
    } else {
      returnType = new rust.Response(this.crate, this.getUnitType());
    }
    rustMethod.returns = new rust.Result(this.crate, returnType);
  }

  private adaptMethodParameter(param: OperationParamType): rust.MethodParameter {
    const paramLoc = param.onClient ? 'client' : 'method';

    // if this is a client method param, check if we've already adapted it
    if (paramLoc === 'client') {
      const clientMethodParam = this.clientMethodParams.get(param.name);
      if (clientMethodParam) {
        return clientMethodParam;
      }
    }

    const paramName = naming.getEscapedReservedName(snakeCaseName(param.name), 'param');
    let paramType = this.getType(param.type);
    if (paramLoc === 'method' && paramType.kind === 'String') {
      // for Strings, we define these as "impl Into<String>" so that passing a str will just work
      paramType = new rust.ImplTrait('Into', paramType);
    }

    let adaptedParam: rust.MethodParameter;
    switch (param.kind) {
      case 'body': {
        adaptedParam = new rust.BodyParameter(paramName, paramLoc, new rust.RequestContent(this.crate, paramType, this.adaptSerdeFormat(param.defaultContentType)));
        break;
      }
      case 'header':
        if (param.collectionFormat) {
          if (paramType.kind !== 'vector') {
            throw new Error(`unexpected kind ${paramType.kind} for HeaderCollectionParameter`);
          }
          let format: rust.CollectionFormat;
          switch (param.collectionFormat) {
            case 'csv':
            case 'simple':
              format = 'csv';
              break;
            case 'pipes':
            case 'ssv':
            case 'tsv':
              format = param.collectionFormat;
              break;
            default:
              throw new Error(`unexpected format ${param.collectionFormat} for HeaderCollectionParameter`);
          }
          adaptedParam = new rust.HeaderCollectionParameter(paramName, param.serializedName, paramLoc, paramType, format);
        } else {
          adaptedParam = new rust.HeaderParameter(paramName, param.serializedName, paramLoc, paramType);
        }
        break;
      case 'path':
        adaptedParam = new rust.PathParameter(paramName, param.serializedName, paramLoc, paramType, param.urlEncode);
        break;
      case 'query':
        if (param.collectionFormat) {
          const format = param.collectionFormat === 'simple' ? 'csv' : (param.collectionFormat === 'form' ? 'multi' : param.collectionFormat);
          if (paramType.kind !== 'vector') {
            throw new Error(`unexpected kind ${paramType.kind} for QueryCollectionParameter`);
          }
          // TODO: hard-coded encoding setting, https://github.com/Azure/typespec-azure/issues/1314
          adaptedParam = new rust.QueryCollectionParameter(paramName, param.serializedName, paramLoc, paramType, true, format);
        } else {
          // TODO: hard-coded encoding setting, https://github.com/Azure/typespec-azure/issues/1314
          adaptedParam = new rust.QueryParameter(paramName, param.serializedName, paramLoc, paramType, true);
        }
        break;
    }

    adaptedParam.docs.summary = param.summary;
    adaptedParam.docs.description = param.doc;

    if (paramLoc === 'client') {
      this.clientMethodParams.set(param.name, adaptedParam);
    }

    return adaptedParam;
  }

  private adaptSerdeFormat(contentType: string): rust.SerdeFormat {
    switch (contentType) {
      case 'application/json':
        this.crate.addDependency(new rust.CrateDependency('serde_json'));
        return 'json';
      default:
        throw new Error(`unhandled contentType ${contentType}`);
    }
  }
}

function isPub(access?: tcgc.AccessFlags): boolean {
  return access !== 'internal';
}

function snakeCaseName(name: string): string {
  return codegen.deconstruct(name).join('_');
}

type OperationParamType = tcgc.SdkBodyParameter | tcgc.SdkHeaderParameter | tcgc.SdkPathParameter | tcgc.SdkQueryParameter;

function recursiveKeyName(root: string, obj: tcgc.SdkType): string {
  switch (obj.kind) {
    case 'array':
      return recursiveKeyName(`${root}-array`, obj.valueType);
    case 'enum':
      return `${root}-${obj.name}`;
    case 'enumvalue':
      return `${root}-${obj.enumType.name}-${obj.value}`;
    case 'dict':
      return recursiveKeyName(`${root}-dict`, obj.valueType);
    case 'plainDate':
      return `${root}-plainDate`;
    case 'utcDateTime':
      return `${root}-${obj.encode}`;
    case 'duration':
      // TODO: this should be: return `${root}-duration-${obj.encode}`;
      // as it is now, it treats the duration as a String
      // https://github.com/Azure/autorest.rust/issues/41
      return `${root}-${obj.wireType.kind}`;
    case 'model':
      return `${root}-${obj.name}`;
    case 'nullable':
      return recursiveKeyName(root, obj.type);
    case 'plainTime':
      return `${root}-plainTime`;
    default:
      return `${root}-${obj.kind}`;
  }
}
