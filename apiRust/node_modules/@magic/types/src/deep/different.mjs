import is from '../index.mjs'

import equal from './equal.mjs'

export const different = (a, b) => {
  if (is.undefined(b)) {
    if (is.undefined(a)) {
      // this most likely is an argument error.
      return false
    }

    return c => different(a, c)
  }

  return !equal(a, b)
}

export default different
