<script lang="ts">
import {defineComponent} from 'vue'
import {DataType, listPhotosInAlbum, Quality} from '~/models/photo'
import {FetchError} from 'ofetch'
import {ImageOutline as PhotoIcon} from '@vicons/ionicons5'

export default defineComponent({
  components: {
    PhotoIcon
  },
  props: {
    albumId: {type: String, required: true}
  },
  data() {
    return {
      loading: true,
      error: '',
      photoUrls: [] as string[]
    }
  },
  async mounted() {
    await this.loadPhotos()
  },
  methods: {
    unloadPhotos() {
      // *** Loop over all the photo urls
      for (const photoUrl of this.photoUrls) {
        try {
          // *** Try to remove the photo URL, will throw if it was not an object URL but that doesn't matter
          URL.revokeObjectURL(photoUrl)
        } catch (e) {
        }
      }

      // *** Empty the array
      this.photoUrls = []
    },
    async loadPhotos() {
      this.loading = true

      // *** Unload existing photos by revoking the object URLs
      this.unloadPhotos()
      this.error = ''

      try {
        // *** Transform the photo list to (object) urls
        this.photoUrls = (await listPhotosInAlbum(this.albumId, Quality.Thumbnail)).map(photo => photo.toObject()).map(photo =>
            photo.dataType == DataType.Bytes && photo.data?.bytes ? URL.createObjectURL(new Blob([photo.data.bytes])) : photo.data?.url
        ).filter(photo => photo) as string[]
      } catch (e: any) {
        // **+* Unload any loaded photos on error
        this.unloadPhotos()

        // *** Grab the error message
        this.error = e instanceof FetchError ? e.statusMessage || e.message : (e as Error).message
      }

      this.loading = false
    }
  }
})
</script>

<template>
  <div>
    <div v-if="loading || photoUrls.length > 0" class="photo-grid">
      <n-skeleton v-if="loading" v-for="_ in 10" class="image-skeleton" :sharp="false" />
      <nuxt-link v-else v-for="photoUrl in photoUrls" class="photo-thumbnail">
        <div class="image-container">
          <img :src="photoUrl"/>
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
