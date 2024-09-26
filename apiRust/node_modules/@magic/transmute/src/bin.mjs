#!/usr/bin/env node

import { cli } from '@magic/cli'

import { handleArgv, runCli } from './bin/index.mjs'

const args = {
  options: [
    ['--help', '-help', 'help', '--h', '-h'],
    '--str',
    ['--input', '--in', '-i'],
    ['--output', '--out', '-o'],
    ['--addWrapper', '--add-wrapper'],
    ['--noPretty', '--no-pretty'],
    ['--markdown', '--mark', '-m'],
  ],
  commands: ['markdown', 'html', 'file'],
  help: {
    name: '@magic/transmute',
    header: 'generates magic functions from html and markdown',
    commands: {
      html: 'convert html to magic functions',
      markdown: 'convert markdown to magic functions',
      file: 'convert file to magic functions',
    },
    options: {
      '--output': 'output file to write to',
      '--add-wrapper': 'add export default[] to the returned string.',
      '--no-pretty': 'do not run prettier.',
      '--markdown': 'force markdown parser to run.',
      '--input': 'input file path',
      '--output': 'output file path',
      '--help': 'this help text',
      '--str': 'an input string of either html or markdown, depending on running command',
    },
    example: `
transpile html string:
magic-transmute html --str '<a href="https://example.com">a link</a>'

transpile markdown string:
magic-transmute markdown --str '[a link](https://example.com)'',

transpile html file:
magic-transmute file --input input.html --output output.mjs

transpile markdown file (.markdown and .md get recogniced):
magic-transmute file --input input.md --output output.mjs

transpile file as markdown (arbitrary file extensions)
magic-transmute file --input input.txt --output output.mjs --markdown
`,
  },
}

const res = cli(args)

const [html, isMarkdown] = handleArgv(res)

runCli(html, isMarkdown, res)
