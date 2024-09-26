## @magic/transmute
transmutes html and markdown to @magic view fragments

[![NPM version][npm-image]][npm-url]
[![Linux Build Status][travis-image]][travis-url]
[![Windows Build Status][appveyor-image]][appveyor-url]
[![Coverage Status][coveralls-image]][coveralls-url]
[![Greenkeeper badge][greenkeeper-image]][greenkeeper-url]
[![Known Vulnerabilities][snyk-image]][snyk-url]


### installation

#### cli:
```bash
npm i -g @magic/transmute

// executable as magic-transmute now
magic-transmute
```

#### javascript api
```bash
npm i --save @magic/transmute
```

### usage

#### cli:

`magic-transmute --help`

#### commands:
* markdown - convert markdown to magic functions
* html     - convert html to magic functions
* file     - convert file to magic functions

#### flags:
* --add-wrapper - add export default[] to the returned string. - alias: ["--addWrapper"]
* --no-pretty   - do not run prettier. - alias: ["--noPretty"]
* --markdown    - force markdown parser to run. - alias: ["--mark", "-m"]
* --output      - output file path - alias: ["--out", "-o"]
* --input       - input file path - alias: ["--in", "-i"]
* --help        - this help text - alias: ["-help", "help", "--h", "-h"]
* --str         - an input string of either html or markdown, depending on running command

##### transpile html string:
`magic-transmute html --str '<a href="https://magic.github.io">magic!</a>'`

##### transpile markdown string:
`magic-transmute markdown --str '[magic!](https://magic.github.io)'`

##### html file:
`magic-transmute file --input input.html --output output.mjs`

##### markdown file
(.markdown and .md get recognized):

`magic-transmute file --input input.md --output output.mjs`

##### force markdown
(arbitrary file extensions)

`magic-transmute file --input input.txt --output output.mjs --markdown`


#### api examples
```javascript
import transmute from '@magic/transmute'

// html to magic
transmute.html('<a href="https://magic.github.io">magic!</a>')
// returns: `Link({ to: 'https://magic.github.io' }, 'magic!')`

// markdown to magic
transmute.markdown('[magic!](https://magic.github.io)')
// returns: `Link({ to: 'https://magic.github.io' }, 'magic!')`

const magic = transmute.markdown('[magic!](https://magic.github.io)')
console.log(magic)
// logs: `Link({ to: 'https://magic.github.io' }, 'magic!')`

// that's it.
```

#### changelog

##### 0.0.1
first release

##### 0.0.2
update dependencies

##### 0.0.3
fix cli

##### 0.0.4
update @magic dependencies to use npm packages instead of github

##### 0.0.5
* extract state from html and markdown files
* allow usage of state variables in markdown and html

##### 0.0.6
* pass options from LexLex to marked.Lexer
* only implant state in html if it has not been passed as argument
* correctly camelCase object keys for html tags
* correctly merge passed state with in-file state for markdown and html

##### 0.0.7 - broken
remove commonjs support, node >= 13.5.0 required

##### 0.0.8
export correct cli file

##### 0.0.9
do not lowercase @magic-module names, some other minor changes

##### 0.0.10
do not wrap @magic-modules in p tags

##### 0.0.11
* remove eval from state injection function.
* correctly handle both ${} and {{}} variables in markdown and html files.

##### 0.0.12
state keys in ${} and {{ }} get trimmed... doh.

##### 0.0.13
return originalState, which is the state derived from the markdown/html file.
this allows @magic to use this state for individual pages when building them.

##### 0.0.14

##### FULL @magic-module api available from markdown.

* nested state variables in template strings eg {{state.test.deep.var}} now work.
* replace parse5 with posthtml.
* custom markdown and custom html renderers added.
* @magic-modules in markdown and html can have keys that are arrays or objects now.

##### 0.0.15
if the value of a ${state.variable} in a template string is an array, this array gets join(' ')ed

##### 0.0.16
render/html: empty arrays and objects return an empty string.

##### 0.0.17
render/markdown.codespan: do not show linenumbers for codespans.

##### 0.0.18
* handle &quot; escapes.
* string delimiters ', " AND ` in a string do not error, get escaped instead.

##### 0.0.19
* actually escape multiple html entities in a string, not just the first occurrence

##### 0.0.20
update dependencies

##### 0.0.21
update dependencies

##### 0.0.22
update prettier, @magic/cli

##### 0.0.23
Link and Img do not get wrapped in a paragraph (anymore)

##### 0.0.24
update marked and prettier

##### 0.0.25
update dependencies

##### 0.0.26
* bump required node version to 14.2.0
* update dependencies

##### 0.0.27
* update dependencies
* FIX: command line interface was broken.

##### 0.0.28
* update dependencies
* move magic-modules/no-spy to devDependencies

##### 0.0.29 
update dependencies

##### 0.0.30 
update dependencies

##### 0.0.31
update dependencies

##### 0.0.32 
update dependencies

##### 0.0.33
* update dependencies
* fix markdown html handling changes

##### 0.0.34 
* update dependencies

##### 0.0.35 
update dependencies

##### 0.0.36 
update dependencies

##### 0.0.37
* bump required node version to 14.15.4
* update dependencies

##### 0.0.38 - unreleased
...

[npm-image]: https://img.shields.io/npm/v/@magic/transmute.svg
[npm-url]: https://www.npmjs.com/package/@magic/transmute
[travis-image]: https://img.shields.io/travis/com/magic/transmute/master
[travis-url]: https://travis-ci.com/magic/transmute
[appveyor-image]: https://img.shields.io/appveyor/ci/magic/transmute/master.svg
[appveyor-url]: https://ci.appveyor.com/project/magic/transmute/branch/master
[coveralls-image]: https://coveralls.io/repos/github/magic/transmute/badge.svg
[coveralls-url]: https://coveralls.io/github/magic/transmute
[greenkeeper-image]: https://badges.greenkeeper.io/magic/transmute.svg
[greenkeeper-url]: https://badges.greenkeeper.io/magic/transmute.svg
[snyk-image]: https://snyk.io/test/github/magic/transmute/badge.svg
[snyk-url]: https://snyk.io/test/github/magic/transmute

