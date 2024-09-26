import is from '@magic/types'

import paint from './paint.mjs'
import stringify from './stringify.mjs'

export const colorize = (...args) => {
  if (is.empty(args)) {
    return ''
  } else if (args.length === 1) {
    const arg = args[0]
    if (is.array(arg)) {
      return colorize(...arg)
    }

    return paint.red(arg)
  }

  if (!args.some(arg => is.fn(paint[arg]))) {
    args = ['red', ...args]
  }

  return args
    .map((arg, i) => {
      if (is.function(paint[arg])) {
        return ''
      }

      if (!arg) {
        return ''
      }

      if (is.array(arg)) {
        arg = colorize(...arg)
      } else {
        arg = stringify(arg)
      }

      let colorFn
      if (i > 0) {
        const color = args[i - 1]
        colorFn = paint[color]
      }

      if (is.function(colorFn)) {
        return colorFn(arg)
      } else {
        return arg
      }
    })
    .filter(t => t !== '')
    .join(' ')
    .trim()
}

export default colorize
