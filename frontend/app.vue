<template>
  <NuxtLayout>
    <NuxtPage :transition="{
        name: 'page',
        mode: 'out-in'
      }"/>
  </NuxtLayout>
</template>

<style>
html, body, #__nuxt {
  padding: 0;
  margin: 0;

  width: 100%;
  height: 100%;
}

.page-enter-active,
.page-leave-active {
  transition: opacity ease-in-out 200ms;
}

.page-enter-from,
.page-leave-to {
  opacity: 0;
}
</style>
