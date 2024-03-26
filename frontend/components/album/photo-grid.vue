<script lang="ts">
import {defineComponent} from 'vue'
import {DataType, listPhotosInAlbum, Quality} from '~/models/photo'
import {FetchError} from 'ofetch'
import {ImageOutline as PhotoIcon} from '@vicons/ionicons5'

type Data = {
  loading: boolean
  error?: string,
  loadedPhotos: {
    [photoId: string]: {
      url: string,
      isObjectURL: boolean
    }
  }
}

export default defineComponent({
  components: {
    PhotoIcon
  },
  props: {
    albumId: {type: String, required: true}
  },
  data(): Data {
    return {
      loading: true,
      error: undefined,
      loadedPhotos: {}
    }
  },
  async mounted() {
    await this.loadPhotos()
  },
  methods: {
    unloadPhotos() {
      // *** Loop over all the photo urls
      for (const photoId in this.loadedPhotos) {
        const {url, isObjectURL} = this.loadedPhotos[photoId]
        if (!isObjectURL) {
          continue
        }

        // *** Revoke the object
        URL.revokeObjectURL(url)

        // *** Remove the photo from the list
        delete this.loadedPhotos[photoId]
      }
    },
    async loadPhotos() {
      this.loading = true

      // *** Unload existing photos by revoking the object URLs
      this.unloadPhotos()
      this.error = ''

      try {
        // *** Transform the photo list to (object) urls
        const photos = await listPhotosInAlbum(this.albumId, {
          quality: Quality.Thumbnail
        })

        for (const photo of photos.map(photo => photo.toObject())) {
          if (!photo.id) {
            continue
          }

          switch (photo.dataType) {
            case DataType.Bytes:
              if (!photo.data?.bytes) {
                continue
              }

              const url = URL.createObjectURL(new Blob([photo.data.bytes]))
              this.loadedPhotos[photo.id] = {
                url,
                isObjectURL: true
              }
              break
            case DataType.Url:
              if (!photo.data?.url) {
                continue
              }

              this.loadedPhotos[photo.id] = {
                url: photo.data.url,
                isObjectURL: false
              }
          }
        }
      } catch (e: any) {
        // **+* Unload any loaded photos on error
        this.unloadPhotos()

        // *** Grab the error message
        this.error = e instanceof FetchError ? e.statusMessage || e.message : (e as Error).message
      }

      this.loading = false
    }
  },
  computed: {
    hasPhotos() {
      return Object.keys(this.loadedPhotos).length > 0
    }
  }
})
</script>

<template>
  <div>
    <div v-if="loading || hasPhotos" class="photo-grid">
      <n-skeleton v-if="loading" v-for="_ in 10" class="image-skeleton" :sharp="false"/>
      <nuxt-link v-else :to="`/album/${albumId}/photo/${id}`" v-for="(photo, id) in loadedPhotos" class="photo-thumbnail">
        <div class="image-container">
          <img :src="photo['url']"/>
        </div>
      </nuxt-link>
    </div>
    <div v-else>
      <n-result
          v-if="error"
          status="error"
          title="Failed to retrieve photos"
          :description="error"
      >
        <template #footer>
          <n-button @click="loadPhotos">Try again</n-button>
        </template>
      </n-result>
      <slot v-else name="empty">
        <n-empty>
          <template #icon>
            <n-icon>
              <photo-icon/>
            </n-icon>
          </template>
          Album has no photos
        </n-empty>
      </slot>
    </div>
  </div>
</template>

<style scoped>
.photo-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(250px, 1fr));
  gap: 8px 12px;
}

.photo-grid .image-skeleton,
.photo-grid .image-container {
  width: 100%;
  padding-top: 75%;
  position: relative;
}

.photo-grid .image-container img {
  position: absolute;
  left: 0;
  top: 0;
  width: 100%;
  height: 100%;
  object-fit: cover;
}
</style>
