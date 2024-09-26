# @magic/types

typechecking utilities

[![NPM version][npm-image]][npm-url]
[![Linux Build Status][travis-image]][travis-url]
[![Windows Build Status][appveyor-image]][appveyor-url]
[![Coverage Status][coveralls-image]][coveralls-url]
[![Greenkeeper badge][greenkeeper-image]][greenkeeper-url]
[![Known Vulnerabilities][snyk-image]][snyk-url]

[npm-image]: https://img.shields.io/npm/v/@magic/types.svg
[npm-url]: https://www.npmjs.com/package/@magic/types
[travis-image]: https://img.shields.io/travis/com/magic/types/master
[travis-url]: https://travis-ci.com/magic/types
[appveyor-image]: https://img.shields.io/appveyor/ci/magic/types/master.svg
[appveyor-url]: https://ci.appveyor.com/project/magic/types/branch/master
[coveralls-image]: https://coveralls.io/repos/github/magic/types/badge.svg
[coveralls-url]: https://coveralls.io/github/magic/types
[greenkeeper-image]: https://badges.greenkeeper.io/magic/types.svg
[greenkeeper-url]: https://badges.greenkeeper.io/magic/types.svg
[snyk-image]: https://snyk.io/test/github/magic/types/badge.svg
[snyk-url]: https://snyk.io/test/github/magic/types

##### install
```javascript
npm install @magic/types
```

##### import single function
```js
// single function import
import { isArray } from '@magic/types'

isArray([]) // true
```

##### import all functions
```javascript
import is from '@magic/types'

is.array([]) // true
```

##### functions
```javascript

// test a value for multiple types
is(ele, ...types)
// alias is.eq, isEq, test

// test if a value is not of a type
not(ele, ...types)
// alias is.neq, isNeq, isNot

type comparisons:
isArray([]) // true
// alias isArr, is.array, is.arr

isBoolean(true) // true
// alias isBool, is.boolean, is.bool

isDefined(undefined) //false
// alias isDef, is.defined, is.def

isUndefined(undefined) //false
// alias isUndef, is.undefined, is.undef

isFunction(() => {}) // true
// alias isFunc, isFn, is.function, is.func, is.fn

isAsyncFunction(async () => {}) // true
// alias isAsyncFunc, isAsyncFn, is.asyncFunction, is.asyncFunc, is.asyncFn

isGeneratorFunction(* () => {}) // true
// alias isGeneratorFunc, isGeneratorFunc, is.generatorFunction, is.generatorFunc, is.generatorFn

isNumber(1) // true
// alias isNum, is.number, is.num

isInteger(1) // true
// alias isInt, is.integer, is.int

isFloat(1.1) // true
// alias is.float

isObject({}) // true
// alias isObj, is.object, is.obj

isString('') // true
// alias isStr, is.string, is.str

isRGBAObject({ r: 1, g: 1, b: 1, a: 1 }) // true
// alias isRGBA, is.rgbaObject, is.rgba

isRGBObject
// alias isRGB, is.rgbObject, is.rgb

isHexColor('#333') // true
// alias isHex, is.hex, is.hexColor

isHexAlphaColor('#3333') // true
// alias isHexa, is.hexa, is.hexAlphaColor

isColor('#444') // true
// alias isCol, is.color, is.col

isDate(new Date()) // true
// alias isTime, is.date, is.time

isRegExp(/regexp/) // true
// alias isRegex, is.regexp, is.regExp, is.regex

isTruthy('true') // true
// alias is.truthy

isFalsy(0) // true
// alias is.falsy

isEmpty('') // true
// alias is.empty

isError(new Error('')) // true
// alias isErr, is.error, is.err

isIterable([]) // true
// alias is.iterable

isEmail('a@b.c') // true
// alias isMail, is.email, is.mail

isNull(null) // true
// alias isNil, is.nil, is.null

isUndefinedOrNull(undefined || null) // true
// alias is.undefinedOrNull, is.undefinedOrNil, is.undefOrNull, is.undefOrNil

isBuffer(new Buffer('test')) // true
// alias isBuff, is.buffer, is.buff

isPromise(new Promise()) // true
// alias is.promise, isThenable, isThen, is.thenable, is.then

// removed (for now?)!
isArguments(() => return arguments) // true
// alias isArgs, is.arguments, is.args

isUUID(uuid) // true
// alias is.uuid

testType(42, 'number') // true
// alias is.type

test(42, ['string', 'object']) // false
// alias is.types

isEq(42, 'number') // true
// alias is.eq

isNot = isNeq = is.not(42, 'number') // true
// alias is.neq

isDeepEqual([1, 2, 3], [1, 2, 3]) // true
// alias is.deep.eq, is.deep.equal
isDeepDifferent([1, 2, 3], [1, 2, 3]) // false
// alias is.deep.diff, is.deep.different

isEvery([1, 2, 3], 'number') // true
isEvery([1, 2, 3], is.number) // true
// alias is.every, is.all

isSome([1, 'str', {}], 'number') // true
isSome([1, 'str', {}], is.number) // true
// alias is.some

isNone([1, 2, 3], 'string') // true
isNone([1, 2, 3], is.number) // false
// alias is.none

isInstanceOf(new Date(), Date) // true
// alias is.instance, is.instanceof, is.instanceOf

isCase('UPPERCASE', 'up') // true
isCase('lowercase', 'low') // true
// alias is.case

isUpperCase('UPPERCASE') // true
// alias is.case.upper, is.isCase.upper

isLowerCase('lowercase') // true
// alias is.case.lower, is.isCase.lower

```


#### Changelog

##### 0.0.5
added Map, WeakMap, Set and WeakSet

##### 0.1.0
use es6 modules

##### 0.1.1
FIX: add module field to package.json

##### 0.1.2
FIX: is.number no longer errors on node es6 modules and other weird objects

##### 0.1.3
use @magic/deep for is.deep.equal and is.deep.different

##### 0.1.4
is.deep uses @magic/deep now.

this means that is.deep.equal(null, undefined) is returning a function now,
because it expects currying.

##### 0.1.5
minimum node version is 13.5.0

##### 0.1.6
remove @magic/deep dependency

##### 0.1.7
fix erroneous '@magic/types' import in src/deep/equal.mjs

##### 0.1.8
add
* is.every
* is.some
* is.none

##### 0.1.9
add is.instanceOf

##### 0.1.10
add isCase, isUpperCase, isLowerCase

##### 0.1.11
add isObjectNative

##### 0.1.12
bump required node version to 14.2.0

##### 0.1.13
* add isAsyncFunction
* add isGeneratorFunction

##### 0.1.14
* bump required node version to 14.15.4
* update dependencies

##### 0.1.15 - unreleased
...
