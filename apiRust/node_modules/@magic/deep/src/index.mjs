import is from '@magic/types'

import { loop as lo } from './loop.mjs'
import { merge as me } from './merge.mjs'
import { flatten as fl } from './flatten.mjs'

export const equal = is.deep.equal
export const equals = is.deep.equal
export const eq = is.deep.equal
export const different = is.deep.different
export const diff = is.deep.different

export const flatten = fl
export const loop = lo
export const merge = me

export default {
  equal,
  equals,
  eq,
  different,
  diff,
  flatten,
  loop,
  merge,
}
