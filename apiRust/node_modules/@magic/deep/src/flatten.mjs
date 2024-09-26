export const shallow = flat => (Array.isArray(flat) ? flatten(...flat) : flat)

export const concat = (flat, deep) => flat.concat(shallow(deep))

export const flatten = (...arr) => arr.reduce(concat, [])

export default flatten
