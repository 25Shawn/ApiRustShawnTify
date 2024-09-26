export const error = (err, name = 'Unknown', type = 'E') => {
  if (Array.isArray(err)) {
    name = err[1] || name
    type = err[2] || type

    err = new Error(err[0])
  } else if (err instanceof Error) {
    if (name === 'Unknown' && typeof err.name === 'string') {
      name = err.name
    }
  } else {
    err = new Error(err)

    err.stack = err.stack
      .split('\n')
      .filter(c => !c.includes('/error/src/index.mjs'))
      .join('\n')
  }

  if (typeof name !== 'string') {
    name = 'Unknown'
  }

  err.code = name
  err.name = name
  err.type = type
  err.msg = err.message

  if (!err.code.startsWith(type)) {
    err.code = `${type}_${err.code}`.toUpperCase().replace(/ /g, '_')
  }

  if (err.name === 'Error') {
    err.code = 'E_UNKNOWN'
  }

  const nameRegExp = new RegExp(`${err.name}:? `, 'g')
  const msgRegExp = new RegExp(`${err.message}:? `, 'g')

  // clean stack
  // remove name and message
  err.stack = err.stack
    .replace(nameRegExp, '')
    .replace(msgRegExp, '')
    .replace('Error:', '')
    .split('\n')
    .map(t => t.trim())
    .join('\n')

  return err
}

export default error
