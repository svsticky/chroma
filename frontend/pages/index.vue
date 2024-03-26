<script lang="ts">
import {listAlbums} from '~/models/album'
import {NButton} from 'naive-ui'
import {AddOutline as AddIcon, ImageOutline as ImageIcon, CloseCircleOutline as ErrorIcon} from '@vicons/ionicons5'
import {DataType, getPhoto, Quality} from '~/models/photo'

type Data = {
  loadedPhotos: {
    [key: string]: {
      url: string,
      isObjectURL: boolean
    }
  }
}

export default defineComponent({
  components: {
    ImageIcon,
    ErrorIcon
  },
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
      const albums = await listAlbums({
        includeCoverPhotos: false
      })

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
  },
  data(): Data {
    return {
      loadedPhotos: {}
    }
  },
  unmounted() {
    for (const id in this.loadedPhotos) {
      this.unloadPhoto(id)
    }
  },
  methods: {
    unloadPhoto(id: string) {
      // *** Check if the photo is loaded, otherwise there is nothing to do
      if (!(id in this.loadedPhotos)) {
        return
      }

      // *** Check if it is an object url
      if (this.loadedPhotos[id].isObjectURL) {
        // *** Revoke the object url and thus unload it from memory
        URL.revokeObjectURL(this.loadedPhotos[id].url)
      }

      // *** Delete the entry from the object
      delete this.loadedPhotos[id]
    },
    async loadPhoto(id: string) {
      // *** Check if the photo is already loaded
      if (id in this.loadedPhotos) {
        return true
      }

      try {
        // *** Load the photo
        const photo = await getPhoto(id, {
          quality: Quality.Thumbnail
        })

        // *** Check the data type
        if (photo.dataType == DataType.Url) {
          this.loadedPhotos[id] = {
            url: photo.data.url,
            isObjectURL: false
          }
        } else {
          // *** Check if bytes have been loaded
          if (!photo.data.bytes) {
            return false
          }

          // *** Create the object url
          const url = URL.createObjectURL(new Blob([photo.data.bytes]))

          // *** Add the url to the registry
          this.loadedPhotos[id] = {
            url,
            isObjectURL: true
          }
        }
      } catch (e: any) {
        // *** Load failed
        return false
      }

      // *** Load successful
      return true
    },
    async getPhotoUrl(id: string) {
      // *** Load the photo, will not load if
      await this.loadPhoto(id)

      return this.loadedPhotos[id].url
    }
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
        <Suspense>
          <n-card :title="album.name || 'Untitled album'" class="album-card" size="small" :bordered="false">
            <template #cover>
              <div class="album-cover-photo">
                <n-empty v-if="album.coverPhotoId?.length == 0" :show-description="false">
                  <template #icon>
                    <n-icon>
                      <image-icon/>
                    </n-icon>
                  </template>
                </n-empty>
                <img v-else-if="album.coverPhotoId && loadPhoto(album.coverPhotoId)"
                     :src="getPhotoUrl(album.coverPhotoId)"/>
                <n-empty v-else description="Failed to load image">
                  <template #icon>
                    <n-icon>
                      <error-icon/>
                    </n-icon>
                  </template>
                </n-empty>
              </div>
            </template>
          </n-card>

          <template #fallback>
            Loading...
          </template>
        </Suspense>
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

.album-cover-photo > .n-empty {
  position: absolute;
  top: 50%;
  left: 0;
  width: 100%;
  transform: translateY(-50%);
}

.album-cover-photo > img {
  position: absolute;
  top: 0;
  bottom: 0;
  left: 0;
  right: 0;
}
</style>
