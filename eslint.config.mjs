// eslint.config.mjs
import antfu from '@antfu/eslint-config'

export default antfu({
  stylistic: {
    indent: 2,
    quotes: 'single',
  },
  typescript: true,
  svelte: true,
  ignores: [
    'node_modules',
    '.svelte-kit',
    'rust',
    'static',
  ],
})
