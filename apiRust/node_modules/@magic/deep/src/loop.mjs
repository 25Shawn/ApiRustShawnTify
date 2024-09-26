import is from '@magic/types'

export const loop = (fn, ...items) => {
  if (is.empty(items)) {
    if (is.fn(fn)) {
      return fn(...items)
    }

    return
  } else if (items.length === 1) {
    items = items[0]
  }

  if (!is.function(fn) && is.function(items)) {
    const oldFn = fn
    fn = items
    items = oldFn
  }

  if (!is.function(fn)) {
    return [fn, items]
  }

  if (!items) {
    return fn(items)
  }

  if (!is.function(items.map)) {
    return fn(items)
  }

  return items.map(item => loop(fn, item))
}

export default loop
