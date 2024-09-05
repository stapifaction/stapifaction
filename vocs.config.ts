import { defineConfig } from 'vocs'

export default defineConfig({
  title: 'Docs',
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
  ],
})
