<template>
  <NuxtLayout>
    <NuxtPage :keepalive="{ include: 'index' }" />
  </NuxtLayout>
</template>

<style lang="postcss">
html,
body {
  @apply h-full;
}

body {
  @apply bg-gray-100 dark:bg-gray-900 dark:text-white;
}

#__nuxt {
  @apply min-h-full flex flex-col justify-stretch;
}
</style>
