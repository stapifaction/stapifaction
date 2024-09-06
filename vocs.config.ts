import { defineConfig } from 'vocs'

export default defineConfig({
  title: 'Stapifaction',
  logoUrl: {
    light: '/logo-dark.png',
    dark: '/logo-light.png'
  },
  topNav: [
    {
      text: 'Learn',
      link: '/learn'
    },
    {
      text: 'API',
      link: 'https://docs.rs/stapifaction'
    },
  ],
  sidebar: [
    {
      text: 'Getting Started',
      link: '/learn',
    },
    {
      text: 'Persist collections',
      link: '/learn/persist-collections',
    },
  ],
})
