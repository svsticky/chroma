// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  compatibilityDate: "2024-08-09",
  devtools: {
    enabled: true,
  },
  modules: ["@nuxtjs/tailwindcss", "@vesp/nuxt-fontawesome"],
  fontawesome: {
    icons: {
      solid: [
        "arrow-left",
        "check",
        "chevron-left",
        "chevron-right",
        "circle",
        "circle-check",
        "circle-notch",
        "circle-user",
        "download",
        "image",
        "images",
        "pencil",
        "plus",
        "right-from-bracket",
        "trash",
        "xmark",
      ],
      regular: ["circle-check", "image"],
    },
  },
  runtimeConfig: {
    public: {
      apiBase: "http://localhost:8000",
    },
  },
  build: {
    transpile: ["@egjs/vue3-infinitegrid "],
  },
});
