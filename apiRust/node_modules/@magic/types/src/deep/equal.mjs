// Written by Substack <3

import is from '../index.mjs'

export const equal = (a, b) => {
  // curry
  if (is.undefined(b)) {
    if (is.undefined(a)) {
      // this is not true, but it has to be assumed this is an unintentional comparation
      return false
    }

    return c => equal(a, c)
  }

  if (is.null(b)) {
    return a === b
  }

  // types must match
  if (typeof a !== typeof b) {
    return false
  }

  // bool, string, number, falsy values
  if (is.comparable(a) || is.comparable(b)) {
    return a === b
  }

  // if (is.<arguments(a)) {
  //  return is.length.eq(a, b)
  // }

  // identical 'prototype' property.
  if (a.prototype !== b.prototype) {
    return false
  }

  // real types must match too
  if (Object.prototype.toString.call(a) !== Object.prototype.toString.call(b)) {
    return false
  }

  // dates
  if (is.date(a)) {
    return a === b
  }

  // functions
  if (is.function(a)) {
    return a.toString() === b.toString()
  }

  // buffers
  if (is.buffer(a)) {
    if (a.length !== b.length) {
      return false
    }

    for (let i = 0; i < a.length; i++) {
      if (a[i] !== b[i]) {
        return false
      }
    }

    return true
  }

  // objects
  const ka = Object.keys(a)
  const kb = Object.keys(b)

  // having the same number of owned properties (keys incorporates
  // hasOwnProperty)
  if (ka.length !== kb.length) {
    return false
  }
  // the same set of keys (although not necessarily the same order),
  ka.sort()
  kb.sort()
  // ~~~cheap key test
  for (let i = ka.length - 1; i >= 0; i--) {
    if (ka[i] !== kb[i]) {
      return false
    }
  }

  // equivalent values for every corresponding key, and
  // ~~~possibly expensive deep test
  let key
  for (let i = ka.length - 1; i >= 0; i--) {
    key = ka[i]
    if (!equal(a[key], b[key])) {
      return false
    }
  }

  return typeof a === typeof b
}

export default equal
