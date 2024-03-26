<script lang="ts">
import {defineComponent} from 'vue'
import {DataType, getPhoto, Quality} from "~/models/photo"
import {CloseCircleOutline as ErrorIcon} from "@vicons/ionicons5";

type Data = {
  loading: boolean,
  error?: string,
  photoData?: {
    url: string,
    isObjectURL: boolean
  }
}

export default defineComponent({
  components: {
    ErrorIcon
  },
  props: {
    photoId: {type: String, required: true}
  },
  data(): Data {
    return {
      loading: false,
      error: undefined,
      photoData: undefined
    }
  },
  async mounted() {
    await this.loadPhoto()
  },
  unmounted() {
    this.unloadPhoto()
  },
  methods: {
    unloadPhoto() {
      // *** Check if the photo is loaded, otherwise there is nothing to do
      if (!this.photoData) {
        return
      }

      // *** Check if it is an object url
      if (this.photoData.isObjectURL) {
        // *** Revoke the object url and thus unload it from memory
        URL.revokeObjectURL(this.photoData.url)
      }

      // *** Delete the entry from the object
      delete this.photoData
    },
    async loadPhoto() {
      // *** Check if the photo is already loaded
      if (this.photoData) {
        return
      }

      this.loading = true
      this.error = undefined

      try {
        // *** Load the photo
        const photo = await getPhoto(this.photoId, {
          quality: Quality.Preview
        })

        // *** Check the data type
        if (photo.dataType == DataType.Url) {
          this.photoData = {
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
          this.photoData = {
            url,
            isObjectURL: true
          }
        }
      } catch (e: any) {
        this.error = e
        this.loading = false
      }

      this.loading = false
    }
  },
  computed: {
    photoURL() {
      return this.photoData?.url || ''
    }
  }
})
</script>

<template>
  <div v-if="loading || !photoData">
    Loading...
  </div>
  <div v-else class="photo">
    <n-empty v-if="error" description="Failed to load image">
      <template #icon>
        <n-icon>
          <error-icon/>
        </n-icon>
      </template>
    </n-empty>
    <img v-else :src="photoURL"/>
  </div>
</template>

<style scoped>
.photo {
  position: fixed;
  left: 0;
  top: 0;
  right: 0;
  bottom: 0;
}

.photo > img {
  width: 100%;
  height: 100%;
  object-fit: contain;
}
</style>