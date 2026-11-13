// @ts-check
import withNuxt from './.nuxt/eslint.config.mjs'

export default withNuxt(
  // Your custom configs here
  {
    rules: {
      // Prettier (vue-плагин) печатает void-элементы как <input />,
      // поэтому разрешаем self-closing, чтобы линтер и форматтер не спорили.
      'vue/html-self-closing': [
        'error',
        {
          html: { void: 'always', normal: 'never', component: 'always' },
          svg: 'always',
          math: 'always',
        },
      ],
    },
  },
)
