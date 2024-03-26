<script lang="ts">
import {createAlbum} from '~/models/album'

export default defineComponent({
  setup() {
    definePageMeta({
      parent: '/',
      showFavicon: false,
      showTitle: false
    })
  },
  data() {
    return {
      name: '',
      isDraft: false
    }
  },
  methods: {
    async onAlbumCreate() {
      const albumId = await createAlbum(this.name, {
        isDraft: this.isDraft
      })

      navigateTo(`/album/${albumId}`)
    }
  }
})
</script>

<template>
  <div>
    <n-h1>Create new album</n-h1>
    <n-form @submit.prevent="onAlbumCreate">
      <n-form-item label="Album name">
        <n-input v-model:value="name" placeholder="Album name"/>
      </n-form-item>
      <n-space>
        <n-button type="primary" attr-type="submit">
          Create
        </n-button>
        <nuxt-link to="/">
          <n-button secondary>
            Cancel
          </n-button>
        </nuxt-link>
      </n-space>
    </n-form>
  </div>
</template>

<style scoped>
.n-button {
  width: 150px;
}
</style>
