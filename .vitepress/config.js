import { defineConfig } from 'vitepress'

export default defineConfig({
  title: "The Book of BDK",
  description: "",
  base: "/book-of-bdk/",
  themeConfig: {
    search: {
      provider: "local"
    },
    
    nav: [
      { text: "Book", link: "/book/introduction.md", activeMatch: "/book/" },
    ],

    sidebar: [
        { text: "Introduction", link: "/book/introduction.md" },
        { text: "Project Organization", link: "/book/organization.md" },
        { text: "Getting Started", link: "/book/getting-started.md" },
        
        // Async Esplora
        { 
            text: "Wallet with async Esplora",
            collapsed: false,
            items: [
                { text: "Simple Wallet with Esplora", link: "/book/wallet.md" },
            ]
        },
    ]
  },
})
