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
        { text: "Getting Started", link: "/book/getting-started.md" },
        
        { 
            text: "Simple Wallet",
            collapsed: false,
            items: [
                { text: "The Wallet Struct", link: "/book/wallet.md" },
                { text: "Staying in Sync", link: "/book/sync.md" },
            ]
        },

        { text: "More Advanced Features", link: "/book/advanced-features.md" },
        { text: "Extras", link: "/book/extras.md" },
    ]
  },
})
