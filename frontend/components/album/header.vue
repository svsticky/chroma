<script lang="ts">
import {defineComponent} from 'vue'
import {getAlbum} from '~/models/album'

export default defineComponent({
  props: {
    albumId: {type: String, required: true}
  },
  setup(props) {
    const {pending: loading, data: album} = useAsyncData(`album:${props.albumId}`, async () => {
      return (await getAlbum(props.albumId as string, true, false)).toObject()
    })

    return {loading, album}
  },
  computed: {
    albumName() {
      return this.album?.album?.name || 'Untitled album'
    }
  }
})
</script>

<template>
  <n-space>
    <n-card :bordered="false" style="width: 250px">
      <template #cover>
        <div class="album-cover-photo">
        </div>
      </template>
    </n-card>
    <n-space vertical justify="space-between" style="height: 100%">
      <n-h1 class="album-title" style="padding: 0 14px;">
        <n-skeleton v-if="loading" text :sharp="false"/>
        <div v-else>{{ albumName }}</div>
      </n-h1>
      <n-space>
        <slot name="extra"/>
      </n-space>
    </n-space>
  </n-space>
</template>

<style scoped>
.album-title {
  flex: 1;
}

.album-cover-photo {
  border-radius: 3px;
  width: 100%;
  padding-top: 75%;
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
