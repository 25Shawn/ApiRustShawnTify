import is from '@magic/types'

export const merge = (o1, o2) => {
  if (typeof o1 === 'undefined') {
    return o2
  } else if (typeof o2 === 'undefined') {
    return o1
  }

  if (is.array(o1)) {
    return o1.concat(o2)
  } else if (is.array(o2)) {
    return [].concat(o1, ...o2)
  }

  if (is.mergeable(o1) && is.mergeable(o2)) {
    const keys = Object.keys({ ...o1, ...o2 })
    const final = {}
    keys.forEach(key => {
      if (!o2.hasOwnProperty(key)) {
        final[key] = o1[key]
      } else {
        final[key] = merge(o1[key], o2[key])
      }
    })

    return final
  }

  if (!is.mergeable(o1) && !is.mergeable(o2)) {
    return o2
  }

  return [o1, o2]
}

export default merge
