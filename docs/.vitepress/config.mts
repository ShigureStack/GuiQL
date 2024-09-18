import { defineConfig } from 'vitepress'

// https://vitepress.dev/reference/site-config
export default defineConfig({
  title: "GuiQL Documentation",
  description: "A query language for building UIs",
  themeConfig: {
    // https://vitepress.dev/reference/default-theme-config
    nav: [
      { text: 'Home', link: '/' },
      { text: 'Guide', link: '/guide/' },
      { text: 'Reference', link: '/reference/' },
    ],

    sidebar: [
      {
        text: 'Guide',
        link: '/guide/',
        items: [
          {
            text: 'Getting started',
            link: '/guide/getting-started/',
            items: [
              { text: 'Create a project', link: '/guide/getting-started/create-a-project' },
              { text: 'Counter', link: '/guide/getting-started/counter' },
            ],
          },
          { text: 'Context', link: '/guide/context' },
          { text: 'Custom element', link: '/guide/custom-element'},
          { text: 'Main loop', link: '/guide/main-loop'},
        ]
      },
      {
        text: 'Reference',
        link: '/reference/',
        items: [
          {
            text: 'Statements',
            link: '/reference/statements/',
            items: [
              { text: 'CREATE', link: '/reference/statements/create' },
              { text: 'IMPORT', link: '/reference/statements/import' },
              { text: 'REGISTER VIEW', link: '/reference/statements/register-view' },
              { text: 'REPLACE', link: '/reference/statements/replace' },
              { text: 'SELECT', link: '/reference/statements/select' },
              { text: 'SUBSCRIBE', link: '/reference/statements/subscribe' },
            ],
          },
          {
            text: 'Elements',
            link: '/reference/elements/',
            items: [
              { text: 'Div', link: '/reference/elements/div' },
              { text: 'Input', link: '/reference/elements/input' },
              { text: 'Window', link: '/reference/elements/window' },
            ],
          },
        ],
      },
    ],

    socialLinks: [
      { icon: 'github', link: 'https://github.com/shigurers/GuiQL' }
    ],

    outline: {
        level: 'deep',
    },
  }
})
