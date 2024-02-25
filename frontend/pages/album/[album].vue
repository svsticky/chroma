<script lang="ts">
import {getAlbum} from '~/models/album'

export default defineComponent({
  setup() {
    const editing = useEditing()
    const route = useRoute()

    const albumId = route.params['album'] as string

    const {pending: loading, data: album} = useAsyncData(`album:${albumId}`, async () => {
      return (await getAlbum(albumId)).toObject()
    })

    return {editing, loading, album}
  },
  computed: {
    albumName() {
      return this.album?.album?.name ? this.album.album.name : 'Untitled album'
    }
  }
})
</script>

<template>
  <div>
    <n-space>
      <n-card :bordered="false" style="width: 200px">
        <template #cover>
          <div class="image-cover">

          </div>
        </template>
      </n-card>
      <n-h1 class="title">
        <n-skeleton v-if="loading" text :sharp="false"/>
        <div v-else @focus="editing.setEditing(true)" contenteditable>{{ albumName }}</div>
      </n-h1>
    </n-space>
    <n-image-group class="photo-list">
      <n-image v-if="loading" v-for="n in 10" class="album-card" size="small">
        <template #header>
          <n-skeleton text :sharp="false"/>
        </template>
        <template #cover>
          <n-skeleton class="image-cover"/>
        </template>
      </n-image>
      <n-image v-else v-for="{album} in []">
      </n-image>
    </n-image-group>
  </div>
</template>

<style scoped>
.title {
  flex: 1;
  cursor: text;
}

.image-cover {
  border-radius: 3px;
  width: 100%;
  padding-top: 100%; /* 1:1 Aspect Ratio */
  position: relative;
  background-color: #cccccc;
}

.image-cover > img {
  position: absolute;
  top: 0;
  bottom: 0;
  left: 0;
  right: 0;
}
</style>
