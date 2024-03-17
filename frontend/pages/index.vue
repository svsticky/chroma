<script lang="ts">
import {listAlbums} from '~/models/album'
import {NButton} from 'naive-ui'
import {AddOutline as AddIcon} from '@vicons/ionicons5'

export default defineComponent({
  setup() {
    definePageMeta({
      actionComponents: [
        () => h(NButton, {
          'onClick': () => navigateTo(`/album/new`),
          'quaternary': true,
          'render-icon': () => h(AddIcon)
        }, () => 'Add album')
      ]
    })

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
    <div class="album-grid">
      <n-card v-if="loading" v-for="n in 10" class="album-card" size="small">
        <template #header>
          <n-skeleton text :sharp="false"/>
        </template>
        <template #cover>
          <n-skeleton class="album-cover-photo"/>
        </template>
      </n-card>
      <nuxt-link v-else v-for="{album} in albums" :to="`/album/${album.id}`" class="album-link">
        <n-card :title="album.name || 'Untitled album'" class="album-card" size="small" :bordered="false">
          <template #cover>
            <div class="album-cover-photo">
              <!--            Add cover -->
            </div>
          </template>
        </n-card>
      </nuxt-link>
    </div>
  </div>
</template>

<style scoped>
.album-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(200px, 1fr));
  gap: 8px 12px;
}

.album-link {
  text-decoration: none;
}

.album-cover-photo {
  width: 100%;
  padding-top: 75%; /* 1:1 Aspect Ratio */
  position: relative;
  background-color: #cccccc;
}

.album-cover-photo > img {
  position: absolute;
  top: 0;
  bottom: 0;
  left: 0;
  right: 0;
}
</style>
