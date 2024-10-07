/*---------------------------------------------------------------------------------------------
*  Copyright (c) Microsoft Corporation. All rights reserved.
*  Licensed under the MIT License. See License.txt in the project root for license information.
*--------------------------------------------------------------------------------------------*/

// if name is a reserved word, append the suffix and return the result, else return name
// the suffix indicates the context in which name appears
//    fn - function name
// param - param name
//  prop - struct field
export function getEscapedReservedName(name: string, suffix: 'fn' | 'param' | 'prop'): string {
  if (reservedWords.has(name)) {
    name = `${name}_${suffix}`;
  }
  return name;
}

// https://doc.rust-lang.org/reference/keywords.html
const reservedWords = new Set<string>(
  [
    // strict keywords
    'as', 'async', 'await', 'break', 'const', 'continue', 'crate', 'dyn', 'else', 'enum', 'extern', 'false', 'fn',
    'for', 'if', 'impl', 'in', 'let', 'loop', 'match', 'mod', 'move', 'mut', 'pub', 'ref', 'return', 'self',
    'Self', 'static', 'struct', 'super', 'trait', 'true', 'type', 'unsafe', 'use', 'where', 'while',

    // reserved keywords
    'abstract', 'become', 'box', 'do', 'final', 'macro', 'override', 'priv', 'try', 'typeof', 'unsized', 'virtual', 'yield',

    // weak keywords
    'macro_rules', 'union', '\'static',
  ]
);
