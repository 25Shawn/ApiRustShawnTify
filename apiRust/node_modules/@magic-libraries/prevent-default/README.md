## @magic-libraries/prevent-default

an effect that calls event.prevent-default for any event.

this library gets imported into @magic apps automagically.

[html-docs](https://magic-libraries.github.io/prevent-default)

[@magic](https://magic.github.io/core)

[![NPM version][npm-image]][npm-url]
[![Linux Build Status][travis-image]][travis-url]
[![Windows Build Status][appveyor-image]][appveyor-url]
[![Coverage Status][coveralls-image]][coveralls-url]
[![Greenkeeper badge][greenkeeper-image]][greenkeeper-url]
[![Known Vulnerabilities][snyk-image]][snyk-url]

[npm-image]: https://img.shields.io/npm/v/@magic-libraries/prevent-default.svg
[npm-url]: https://www.npmjs.com/package/@magic-libraries/prevent-default
[travis-image]: https://img.shields.io/travis/com/magic-libraries/prevent-default/master
[travis-url]: https://travis-ci.com/magic-libraries/prevent-default
[appveyor-image]: https://img.shields.io/appveyor/ci/magiclibraries/prevent-default/master.svg
[appveyor-url]: https://ci.appveyor.com/project/magiclibraries/prevent-default/branch/master
[coveralls-image]: https://coveralls.io/repos/github/magic-libraries/prevent-default/badge.svg
[coveralls-url]: https://coveralls.io/github/magic-libraries/prevent-default
[greenkeeper-image]: https://badges.greenkeeper.io/magic-libraries/prevent-default.svg
[greenkeeper-url]: https://badges.greenkeeper.io/magic-libraries/prevent-default.svg
[snyk-image]: https://snyk.io/test/github/magic-libraries/prevent-default/badge.svg
[snyk-url]: https://snyk.io/test/github/magic-libraries/prevent-default

* [usage](#usage)

#### <a name="usage"></a>usage
in a page/component, just use the lib.preventDefault effect

```javascript
a({
  to: '/somewhere',
  onclick: [actions.component.click, lib.preventDefault],
})

form({
  name: 'form-name',
  onsubmit: [actions.form.submit, lib.preventDefault],
})
```

#### changelog

##### 0.0.1
first commit

##### 0.0.2
fix readme

##### 0.0.3
require node 13.5.0

##### 0.0.4
bump required node version

##### 0.0.5
bump required node version to 14.15.4

##### 0.0.6 - unreleased
...
