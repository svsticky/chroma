<script lang="ts">
import {listAlbums} from '~/models/album'

export default defineComponent({
  setup() {
    const auth = useAuth()

    const {pending: loading, data: albums} = useAsyncData('albums', async () => {
      const albums = await listAlbums() || []

      albums.sort(({album: a}, {album: b}) => {
        if (a.publishedAt != null && b.publishedAt != null) {
          return a.publishedAt - b.publishedAt
        } else {
          return a.createdAt - b.createdAt
        }
      })

      return albums.map(album => album.toObject())
    })

    return {auth, loading, albums}
  }
})
</script>

<template>
  <div>
    <n-h1>Albums</n-h1>
    <div class="album-list">
      <n-card v-if="loading" v-for="n in 10" class="album-card" size="small">
        <template #header>
          <n-skeleton text :sharp="false"/>
        </template>
        <template #cover>
          <n-skeleton class="image-cover"/>
        </template>
      </n-card>
      <n-card v-else v-for="{album} in albums" :title="album.name || 'Untitled album'" class="album-card" size="small"
              @click="navigateTo(`/album/${album.id}`)">
        <template #cover>
          <div class="image-cover">
            <!--            Add cover -->
          </div>
        </template>
      </n-card>
    </div>
  </div>
</template>

<style scoped>
.album-list {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(200px, 1fr));
  gap: 8px 12px;
}

.album-card {
  cursor: pointer;
}

.image-cover {
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
