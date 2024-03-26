<script lang="ts">
export default defineComponent({
  async setup() {
    definePageMeta({
      parent: () => {
        const route = useRoute()
        const albumId = route.params['album'] as string

        return `/album/${albumId}`
      },
      showFavicon: false,
      showTitle: false,
      showSearchBar: false,
      fixed: true,
      dark: true
    })


    const route = useRoute()
    const photoId = route.params['photo'] as string

    return {photoId}
  }
})
</script>

<template>
  <photo-preview :photo-id="photoId"/>
</template>

<style scoped>

</style>