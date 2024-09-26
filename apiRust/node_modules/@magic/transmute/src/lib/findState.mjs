// find and extract the page state from both markdown and html vars.
export const findState = input => {
  if (input.trim().startsWith('---')) {
    const splinters = input.split('---')

    // bail early, not a state declaration.
    if (splinters.length < 3) {
      return { input }
    }

    const stateJson = splinters[1]
    input = splinters[2]

    if (stateJson.trim().startsWith('@state')) {
      const state = JSON.parse(stateJson.replace('@state', ''))
      return {
        state,
        input,
      }
    }
  }

  return {
    input,
  }
}
