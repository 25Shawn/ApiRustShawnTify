import is from '@magic/types'

export const stringifyByType = a => {
  if (is.array(a) && a.length === 1) {
    return stringifyByType(a[0])
  }

  if (is.string(a)) {
    return a
  } else if (is.number(a)) {
    return a
  } else if (is.array(a)) {
    return stringify(...a)
  } else if (is.object(a)) {
    if (is.function(a) || is.date(a) || is.regexp(a)) {
      return a.toString()
    }

    return JSON.stringify(a)
  }

  return '' + a
}

export const byEmptyString = t => t !== ''

export const stringify = (...str) =>
  str
    .map(stringifyByType)
    .filter(byEmptyString)
    .join(' ')

export default stringify
