<script lang="ts">
import {NButton} from 'naive-ui'
import {CloudUploadOutline as UploadIcon, CreateOutline as EditIcon, ImageOutline as PhotoIcon} from '@vicons/ionicons5'

export default defineComponent({
  components: {
    UploadIcon,
    PhotoIcon
  },
  setup() {
    definePageMeta({
      parent: '/',
      showFavicon: false,
      showTitle: false,
      actionComponents: [
        () => h(NButton, {
          'onClick': (e) => {
            const route = useRoute()
            const albumId = route.params['album'] as string
            return navigateTo(`${albumId}/edit`)
          },
          'quaternary': true,
          'render-icon': () => h(EditIcon)
        }, () => 'Edit album')
      ]
    })

    const route = useRoute()
    const albumId = route.params['album'] as string

    return {albumId}
  },
  data() {
    return {
      showUpload: false
    }
  },
  methods: {
    openUpload() {
      this.showUpload = true
    },
    closeUpload() {
      this.showUpload = false
    }
  }
})
</script>

<template>
  <div>
    <album-upload-modal
        v-model:open="showUpload"
        @uploadsDone="() => $refs.photoGrid.loadPhotos()"
        :album-id="albumId"
    />

    <n-space vertical>
      <album-header :album-id="albumId">
        <n-button quaternary @click="openUpload">
          <template #icon>
            <upload-icon/>
          </template>
          Upload photos
        </n-button>
      </album-header>
      <album-photo-grid ref="photoGrid" :album-id="albumId">
        <template #empty>
          <n-empty>
            <template #icon>
              <n-icon>
                <photo-icon/>
              </n-icon>
            </template>
            Album has no photos
            <template #extra>
              <n-button @click="openUpload">Upload photos</n-button>
            </template>
          </n-empty>
        </template>
      </album-photo-grid>
    </n-space>
  </div>
</template>

<style scoped>
</style>
