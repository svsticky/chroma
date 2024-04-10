<script lang="ts">
import {DataType, getPhoto, Quality} from '~/models/photo'
import {ArrowBackOutline as BackIcon, CloseCircleOutline as ErrorIcon} from '@vicons/ionicons5'
import {darkTheme} from 'naive-ui'

type Data = {
  loading: boolean,
  error?: string,
  photoData?: {
    url: string,
    albumId: string,
    isObjectURL: boolean
  }
}

export default defineComponent({
  components: {
    BackIcon,
    ErrorIcon
  },
  props: {
    photoId: {type: String, required: true}
  },
  setup() {
    return {darkTheme}
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

        // *** Get the parent album id from the response
        const albumId = photo.albumId

        // *** Check the data type
        if (photo.dataType == DataType.Url) {
          this.photoData = {
            url: photo.data.url,
            albumId,
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
            albumId,
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
  <n-config-provider :theme="darkTheme">
    <n-layout class="photo-preview">
      <n-layout-header position="absolute">
        <n-grid :cols="2" layout-shift-disabled style="height: 100%">
          <n-gi>
            <n-space align="center" style="font-size: 0; height: 100%;">
              <nuxt-link v-if="photoData?.albumId" :to="`/album/${photoData.albumId}`">
                <n-button text>
                  <n-icon size="22">
                    <back-icon/>
                  </n-icon>
                </n-button>
              </nuxt-link>
            </n-space>
          </n-gi>
          <n-gi>
            <n-space align="center" justify="end" style="font-size: 0; height: 100%;">
            </n-space>
          </n-gi>
        </n-grid>
      </n-layout-header>
      <n-layout-content position="absolute">
        <n-space class="photo-container" justify="center" align="center" style="height: 100%">
          <transition mode="out-in">
            <n-empty v-if="!loading && error" description="Failed to load image">
              <template #icon>
                <n-icon>
                  <error-icon/>
                </n-icon>
              </template>
            </n-empty>
            <div v-else-if="!loading && photoData" class="photo">
              <img class="photo" :src="photoURL"/>
            </div>
            <n-spin v-else/>
          </transition>
        </n-space>
      </n-layout-content>
    </n-layout>
  </n-config-provider>
</template>

<style scoped>
.photo-preview {
  position: fixed;
  left: 0;
  top: 0;
  right: 0;
  bottom: 0;
  z-index: 1;
}

.photo-container > * {
  position: absolute;
  left: 0;
  top: 0;
  right: 0;
  bottom: 0;
}

.n-layout-header {
  padding: 10px;
  height: 54px;
  background: linear-gradient(0deg,transparent,rgba(0,0,0,0.38));
  z-index: 1;
}

.photo {
  width: 100vw;
  height: 100vh;
}

.photo > img {
  position: absolute;
  width: 100%;
  height: 100%;
  object-fit: contain;
}

.v-enter-active,
.v-leave-active {
  transition: opacity ease-in-out 200ms;
}

.v-enter-from,
.v-leave-to {
  opacity: 0;
}
</style>
