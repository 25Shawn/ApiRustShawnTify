import is from '@magic/types'

import { paint } from './lib/index.mjs'

const log = (...args) => console.log(...args)
log.levels = ['all', 'warn', 'error']

log.resetLevel = (env = process.env.NODE_ENV) => {
  if (env === 'production') {
    log.level = 1
  } else {
    log.level = 0
  }
  return log.level
}

/* initialize loglevel */
log.resetLevel()

log.getLevel = () => (is.number(log.level) ? log.level : log.resetLevel())

log.setLevel = (lvl, env = process.env.NODE_ENV) => {
  if (is.undefinedOrNull(lvl)) {
    return log.resetLevel()
  }
  if (is.string(lvl)) {
    const idx = log.levels.indexOf(lvl)
    if (idx > -1) {
      lvl = idx
    }
  }

  if (!is.number(lvl) || lvl < 0) {
    if (env === 'production') {
      lvl = 1
    } else {
      lvl = 0
    }
  }

  if (lvl > log.levels.length - 1) {
    lvl = log.levels.length - 1
  }

  return (log.level = lvl)
}

log.info = (...msg) => {
  if (log.getLevel() > 0) {
    return false
  }

  console.log(...msg)

  return true
}

log.success = (a, ...b) => log.info(paint('green', a), ...b)

log.error = (...args) => {
  const [a, ...b] = args
  if (is.error(a)) {
    console.error(paint('red', a.message), `\n${a.stack}\n`, ...b)
    return true
  }
  const msg = [paint('red', a), ...b]

  console.error(...msg)

  return true
}

log.warn = (...args) => {
  if (log.getLevel() > 1) {
    return false
  }

  const [a, ...b] = args
  const msg = [paint('yellow', a), ...b]

  console.warn(...msg)

  return true
}

log.annotate = (...a) => log.info(paint('grey', ...a))

log.log = log

log.color = paint
log.paint = paint

log.time = a => {
  if (log.getLevel() > 1) {
    return false
  }
  console.time(a)
  return true
}

log.timeEnd = a => {
  if (log.getLevel() > 1) {
    return false
  }

  console.timeEnd(a)
  return true
}

log.hrtime = a => process.hrtime(a)

log.timeTaken = (a, pre = '', post = '') => {
  const [s, ns] = process.hrtime(a)
  let span = s * 1000000 + ns / 1000
  let unit = 'ns'

  if (span > 1500000) {
    unit = 's'
    span = span / 1000000
  } else if (ns > 1500) {
    unit = 'ms'
    span = span / 1000
  }

  span = span.toFixed(1)

  let res = `${span}${unit}`

  if (pre) {
    // do not add a space if this is part of a string concat
    if (pre.endsWith('"') || pre.endsWith("'")) {
      res = pre + res
    } else {
      res = `${pre} ${res}`
    }
  }
  if (post) {
    // do not add a space if this is part of a string concat
    if (post.startsWith('"') || post.startsWith("'")) {
      res += post
    } else {
      res = `${res} ${post}`
    }
  }

  log(res)

  return res
}

export default log
