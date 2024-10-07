// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License. See License.txt in the project root for license information.
import { exec, execSync } from 'child_process';
import { semaphore } from './semaphore.js';

// limit to 8 concurrent builds
const sem = semaphore(8);

const pkgRoot = execSync('git rev-parse --show-toplevel').toString().trim() + '/eng/packages/http-client-rust/';

const tspRoot = pkgRoot + 'node_modules/@azure-tools/cadl-ranch-specs/http/';

// the format is as follows
// 'crateName': [ 'input', 'additional arg 1', 'additional arg N...' ]
// if no .tsp file is specified in input, it's assumed to be main.tsp
const cadlRanch = {
  //'cadl_apikey': ['authentication/api-key'],
  //'cadl_custom': ['authentication/http/custom'],
  //'cadl_oauth2': ['authentication/oauth2'],
  //'cadl_unionauth': ['authentication/union'],
  //'cadl_access': ['azure/client-generator-core/access'],
  'cadl_flattenproperty': ['azure/client-generator-core/flatten-property'],
  //'cadl_coreusage': ['azure/client-generator-core/usage'],
  //'cadl_basic': ['azure/core/basic'],
  //'cadl_lrorpc': ['azure/core/lro/rpc'],
  //'cadl_lrolegacy': ['azure/core/lro/rpc-legacy'],
  //'cadl_lrostd': ['azure/core/lro/standard'],
  //'cadl_corescalar': ['azure/core/scalar'],
  //'cadl_traits': ['azure/core/traits'],
  //'cadl_naming': ['client/naming'],
  //'cadl_default': ['client/structure/default/client.tsp'],
  //'cadl_multiclient': ['client/structure/multi-client/client.tsp'],
  //'cadl_renamedop': ['client/structure/renamed-operation/client.tsp'],
  //'cadl_twoop': ['client/structure/two-operation-group/client.tsp'],
  //'cadl_bytes': ['encode/bytes'],
  //'cadl_datetime': ['encode/datetime'],
  //'cadl_duration': ['encode/duration'],
  //'cadl_bodyoptional': ['parameters/body-optionality'],
  'cadl_collectionfmt': ['parameters/collection-format'],
  //'cadl_spread': ['parameters/spread'],
  //'cadl_contentneg': ['payload/content-negotiation'],
  //'cadl_jmergepatch': ['payload/json-merge-patch'],
  //'cadl_mediatype': ['payload/media-type'],
  //'cadl_multipart': ['payload/multipart'],
  //'cadl_pageable': ['payload/pageable'],
  //'cadl_srvdrivenold': ['resiliency/srv-driven/old.tsp'],
  //'cadl_srvdrivennew': ['resiliency/srv-driven'],
  'cadl_jsonencodedname': ['serialization/encoded-name/json'],
  //'cadl_multiple': ['server/path/multiple'],
  //'cadl_single': ['server/path/single'],
  //'cadl_unversioned': ['server/versions/not-versioned'],
  //'cadl_versioned': ['server/versions/versioned'],
  //'cadl_clientreqid': ['special-headers/client-request-id'],
  //'cadl_condreq': ['special-headers/conditional-request'],
  //'cadl_repeatability': ['special-headers/repeatability'],
  'cadl_specialwords': ['special-words'],
  'cadl_array': ['type/array'],           // needs additional codegen work before we can add tests
  'cadl_dictionary': ['type/dictionary'], // needs additional codegen work before we can add tests
  'cadl_extensible': ['type/enum/extensible'],
  'cadl_fixed': ['type/enum/fixed'],
  'cadl_empty': ['type/model/empty'],
  //'cadl_enumdisc': ['type/model/inheritance/enum-discriminator'],
  //'cadl_nodisc': ['type/model/inheritance/not-discriminated'],
  //'cadl_recursive': ['type/model/inheritance/recursive'],
  //'cadl_singledisc': ['type/model/inheritance/single-discriminator'],
  'cadl_usage': ['type/model/usage'],
  //'cadl_visibility': ['type/model/visibility'],
  //'cadl_addlprops': ['type/property/additional-properties'],
  //'cadl_nullable': ['type/property/nullable'],
  //'cadl_optionality': ['type/property/optionality'],
  //'cadl_valuetypes': ['type/property/value-types'],
  //'cadl_scalar': ['type/scalar'],
  //'cadl_union': ['type/union'],
};

const args = process.argv.slice(2);
var filter = undefined;
const switches = [];
for (var i = 0 ; i < args.length; i += 1) {
  const filterArg = args[i].match(/--filter=(?<filter>\w+)/);
  if (filterArg) {
    filter = filterArg.groups['filter'];
    continue;
  }
  switch (args[i]) {
    case '--verbose':
      switches.push('--verbose');
      break;
    default:
      break;
  }
}

if (filter !== undefined) {
  console.log("Using filter: " + filter)
}

function should_generate(name) {
  if (filter !== undefined) {
    const re = new RegExp(filter);
    return re.test(name)
  }
  return true
}

const keyvault_secrets = pkgRoot + 'test/tsp/Security.KeyVault.Secrests';
generate('keyvault_secrets', keyvault_secrets, 'test/sdk/keyvault_secrets');

for (const crate in cadlRanch) {
  const values = cadlRanch[crate];
  let additionalArgs;
  if (values.length > 1) {
    additionalArgs = values.slice(1);
  }
  // make the output directory structure the same as the cadl input directory.
  // if the input specifies a .tsp file, remove that first.
  let outDir = values[0];
  if (outDir.lastIndexOf('.tsp') > -1) {
    outDir = outDir.substring(0, outDir.lastIndexOf('/'));
  }
  generate(crate, tspRoot + values[0], `test/cadlranch/${outDir}`, additionalArgs);
}

function generate(crate, input, outputDir, additionalArgs) {
  if (!should_generate(crate)) {
    return
  }
  if (additionalArgs === undefined) {
    additionalArgs = [];
  } else {
    for (let i = 0; i < additionalArgs.length; ++i) {
      additionalArgs[i] = `--option="@azure-typespec/http-client-rust.${additionalArgs[i]}"`;
    }
  }
  sem.take(function() {
    // default to main.tsp if a .tsp file isn't specified in the input
    if (input.lastIndexOf('.tsp') === -1) {
      input += '/main.tsp';
    }
    console.log('generating ' + input);
    const fullOutputDir = pkgRoot + outputDir;
    try {
      const options = [];
      options.push(`--option="@azure-typespec/http-client-rust.crate-name=${crate}"`);
      options.push(`--option="@azure-typespec/http-client-rust.crate-version=0.1.0"`);
      options.push(`--option="@azure-typespec/http-client-rust.emitter-output-dir=${fullOutputDir}"`);
      const command = `npx tsp compile ${input} --emit=${pkgRoot} ${options.join(' ')} ${additionalArgs.join(' ')}`;
      if (switches.includes('--verbose')) {
        console.log(command);
      }
      exec(command, function(error, stdout, stderr) {
        // print any output or error from the tsp compile command
        logResult(error, stdout, stderr);
        // format on success
        if (error === null && stderr === '') {
          execSync('cargo fmt --all -- --emit files', { cwd: fullOutputDir, encoding: 'ascii' });
        }
      });
    } catch (err) {
      console.error(err.output.toString());
    } finally {
      sem.leave();
    }
  });
}

function logResult(error, stdout, stderr) {
  if (stdout !== '') {
    console.log('stdout: ' + stdout);
  }
  if (stderr !== '') {
    console.error('\x1b[91m%s\x1b[0m', 'stderr: ' + stderr);
  }
  if (error !== null) {
    console.error('\x1b[91m%s\x1b[0m', 'exec error: ' + error);
  }
}
