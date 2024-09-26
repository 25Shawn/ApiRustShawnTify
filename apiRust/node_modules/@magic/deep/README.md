### @magic/deep

Work with deeply nested objects and arrays.

[![NPM version][npm-image]][npm-url]
[![Linux Build Status][travis-image]][travis-url]
[![Windows Build Status][appveyor-image]][appveyor-url]
[![Coverage Status][coveralls-image]][coveralls-url]
[![Greenkeeper badge][greenkeeper-image]][greenkeeper-url]
[![Known Vulnerabilities][snyk-image]][snyk-url]

[npm-image]: https://img.shields.io/npm/v/@magic/deep.svg
[npm-url]: https://www.npmjs.com/package/@magic/deep
[travis-image]: https://api.travis-ci.com/magic/deep.svg?branch=master
[travis-url]: https://travis-ci.com/magic/deep
[appveyor-image]: https://img.shields.io/appveyor/ci/magic/deep/master.svg
[appveyor-url]: https://ci.appveyor.com/project/magic/deep/branch/master
[coveralls-image]: https://coveralls.io/repos/github/magic/deep/badge.svg
[coveralls-url]: https://coveralls.io/github/magic/deep
[greenkeeper-image]: https://badges.greenkeeper.io/magic/deep.svg
[greenkeeper-url]: https://badges.greenkeeper.io/magic/deep.svg
[snyk-image]: https://snyk.io/test/github/magic/deep/badge.svg
[snyk-url]: https://snyk.io/test/github/magic/deep

##### install
```bash
npm i --save --save-exact @magic/deep
```

##### import
```javascript
// single function
import { equal, flatten, loop, merge } from '@magic/deep'

// object with all functions
import deep from '@magic/deep'
```

Currently implemented:

##### deep.equal
```javascript
// test equality
deep.equal(['shallow', ['deep']], ['shallow', ['deep']])
// true

// alias
deep.equals, deep.eq
```

##### deep.different
```javascript
// test difference
deep.different(['shallow', ['deep']], ['shallow', ['deep']])
// false

// alias
deep.diff
```


##### deep.flatten
```javascript
// flatten a deeply nested array
deep.flatten(['shallow', ['deep']])
// ['shallow', 'deep']
```

##### deep.loop
```javascript
// apply function add
const add = e => e + 1

// for each item
const items = [1, 2, [3]]

// using a deeply nested array
deep.loop(add, items)
// or
deep.loop(items, add)
// returns [2, 3, [4]]
```

##### deep.merge
```javascript
// merge objects and arrays, with infinite recursion if needed.
// this can be slow...

deep.merge({ obj1Key: { val: 1 } }, { obj2Key: { val: 2 } } )

// { obj1Key: { val: 1}, obj2Key: { val: 2 } }

deep.merge({ key: { val: 1, str: 'test' } }, { key: { val: 2, str: 'overwritten' } })

// { key: { val: 2, str: 'overwritten' } }
```

### Changelog

#### 0.1.0
use ecmascript modules instead of commonjs.

#### 0.1.1
* update readme
* also export deep.eq alias for deep.equal

#### 0.1.2
* require node 13.5.0
* use deep.equal and deep.different from @magic/types

#### 0.1.3
bump required node version to 14.2.0

#### 0.1.4 
update dependencies

#### 0.1.5
* bump required node version to 14.15.4
* update dependencies

##### 0.1.6 - unreleased
...
