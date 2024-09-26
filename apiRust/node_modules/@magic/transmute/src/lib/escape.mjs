// escape string delimiters in the string
export const escape = str => str.replace(/'/g, "\\'").replace(/`/g, '\\`').replace(/"/g, '\\"')
