import entities from '@magic/entities'

export const stringDecode = str => {
  entities.forEach(([num, symbol, name]) => {
    if (symbol) {
      const nameRegExp = new RegExp(`&${name};`, 'gim')
      const numRegExp = new RegExp(`&#${num};`, 'gim')

      str = str.replace(nameRegExp, symbol).replace(numRegExp, symbol)
    }
  })

  return str
}
