# @magic/error

throw errors with custom code/name.

[html-docs](https://magic.github.io/error)

[![NPM version][npm-image]][npm-url]
[![Linux Build Status][travis-image]][travis-url]
[![Windows Build Status][appveyor-image]][appveyor-url]
[![Coverage Status][coveralls-image]][coveralls-url]
[![Greenkeeper badge][greenkeeper-image]][greenkeeper-url]
[![Known Vulnerabilities][snyk-image]][snyk-url]

[npm-image]: https://img.shields.io/npm/v/@magic/error.svg
[npm-url]: https://www.npmjs.com/package/@magic/error
[travis-image]: https://img.shields.io/travis/com/magic/error/master
[travis-url]: https://travis-ci.com/magic/error
[appveyor-image]: https://img.shields.io/appveyor/ci/magic/error/master.svg
[appveyor-url]: https://ci.appveyor.com/project/magic/error/branch/master
[coveralls-image]: https://coveralls.io/repos/github/magic/error/badge.svg
[coveralls-url]: https://coveralls.io/github/magic/error
[greenkeeper-image]: https://badges.greenkeeper.io/magic/error.svg
[greenkeeper-url]: https://badges.greenkeeper.io/magic/error.svg
[snyk-image]: https://snyk.io/test/github/magic/error/badge.svg
[snyk-url]: https://snyk.io/test/github/magic/error

### installation:
```javascript
npm install @magic/error
```

### usage:
```javascript
import error from '@magic/error'

err = error('message', 'name')
// Error { message: 'message', name: 'name', code: 'E_NAME', type: 'E', msg: 'message' }

err = error('message')
// Error { message: 'message', name: 'Unknown', code: 'E_UNKNOWN', type: 'E', msg: 'message' }

err = error(new Error('message'))
// Error { message: 'message', name: 'Error', code: 'E_UNKNOWN', type: 'E', msg: 'message' }

err = error('message', 'name with spaces')
// Error {
//  message: 'message',
//  name: 'name with spaces',
//  code: 'E_NAME_WITH_SPACES',
//  type: 'E',
//  msg: 'message',
// }

err = error(['message', 'name', 'T'])
// Error { message: 'message', name: 'name', code: 'T_NAME', type: T, msg: 'message' }

err = error(['message'], 'name', 'T')
// Error { message: 'message', name: 'name', code: 'T_NAME', type: T, msg: 'message' }

err = error(['message', 'name1'], 'name2', 'T')
// Error { message: 'message', name: 'name1', code: 'T_NAME1', type: T, msg: 'message' }
```

#### error types
errors can have types

```javascript
// E is the default type
err = error('message', 'name')
// Error { message: 'message', name: 'name', code: 'E_NAME', type: 'E', msg: 'message' }

// warnings:
err = error('message', 'name', 'W')
// Error { message: 'message', name: 'name', code: 'W_NAME', type: 'W', msg: 'message' }

// debug:
err = error('message', 'name', 'D')
// Error { message: 'message', name: 'name', code: 'D_NAME', type: 'D', msg: 'message' }
```

##### changelog

##### 0.0.1
first commit

#### 0.0.2
* errors can be passed as first argument
* error.name is unchanged, error.code gets transformed to start with E_ and be uppercased.
* error.code for passed in errors is E_UNKNOWN.

#### 0.0.3
* error type (third fn argument) can now be set, default is E to keep output the same.
* error name (second fn argument) can now be a string with spaces.
  e.code is e.type + e.name, but UPPER_SNAKE_CASED.
* error stack cleanup improved.

#### 0.0.4
* err.msg is no === err.message
* first argument can be an array of [message, name, type]

#### 0.0.5
* fix replacement of : in error name and error message to only replace the ones we want to replace

#### 0.0.6
bump required node version to 14.2.0

#### 0.0.7 
update depdendencies

#### 0.0.8
* bump required node version to 14.15.4
* update dependencies

##### 0.0.9 - unreleased
...
