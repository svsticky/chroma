<script lang="ts">
import {createAlbum} from '~/models/album'

export default defineComponent({
  data() {
    return {
      name: '',
      isDraft: false
    }
  },
  methods: {
    async onAlbumCreate() {
      const albumId = await createAlbum(this.name, this.isDraft)

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
        <n-button secondary @click="this.$router.go(-1)">
          Cancel
        </n-button>
        <n-button type="primary" attr-type="submit">
          Create
        </n-button>
      </n-space>
    </n-form>
  </div>
</template>

<style scoped>
.n-button {
  width: 150px;
}
</style>
