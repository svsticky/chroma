<script lang="ts">
import type {UploadCustomRequestOptions, UploadFileInfo} from 'naive-ui'
import {createPhoto, getPhoto, Quality} from '~/models/photo'
import {ArchiveOutline as ArchiveIcon} from '@vicons/ionicons5'

export default defineComponent({
  components: {
    ArchiveIcon
  },
  props: {
    albumId: {type: String, required: true},
    open: Boolean
  },
  emits: ['update:open', 'uploadsDone'],
  data() {
    return {
      uploadList: [] as UploadFileInfo[],
      uploadsPending: 0,
      uploadsDone: false
    }
  },
  methods: {
    uploadRequest({
                    file,
                    onFinish,
                    onError,
                    onProgress
                  }: UploadCustomRequestOptions) {
      (async () => {
        this.uploadsDone = false
        this.uploadsPending++

        const photoBytes = new Uint8Array(await file.file?.arrayBuffer()!)
        onProgress({percent: 10})
        await createPhoto(this.albumId, photoBytes)

        // todo: Extract to separate function and handle failures/addition of extra uploads
        this.uploadsPending--

        if (!this.uploadsPending) {
          this.uploadsDone = true
          this.$emit('uploadsDone')
        }

        onFinish()
      })().catch(() => {
        this.uploadsPending--

        if (!this.uploadsPending) {
          this.uploadsDone = true
          this.$emit('uploadsDone')
        }

        onError()
      })
    }
  }
})
</script>

<template>
  <modal
      :show="open"
      @update:open="$emit('update:open', $event)"
      @close="uploadList = []; uploadsDone = false"
      title="Upload photos"
      style="width: 75%">
    <n-upload
        v-if="uploadList.length == 0"
        multiple
        directory-dnd
        accept=".png,.jpeg,.jpg"
        :show-file-list="false"
        :default-upload="false"
        @change="({ file }) => uploadList.push(file)"
    >
      <n-upload-dragger>
        <div style="margin-bottom: 12px">
          <n-icon size="48" :depth="3">
            <archive-icon/>
          </n-icon>
        </div>
        <n-text style="font-size: 16px">
          Click or drag an image to this area to upload
        </n-text>
        <n-p depth="3" style="margin: 8px 0 0 0">
          Drag a folder to upload a whole album
        </n-p>
      </n-upload-dragger>
    </n-upload>
    <n-upload
        v-else
        ref="uploadForm"
        abstract
        multiple
        accept=".png,.jpeg,.jpg"
        list-type="image"
        :default-upload="false"
        :custom-request="uploadRequest"
        :show-cancel-button="false"
        :show-remove-button="false"
        v-model:file-list="uploadList"
    >
      <n-space justify="space-between">
        <n-upload-trigger #="{handleClick}" abstract>
          <n-button @click="handleClick" :disabled="!!uploadsPending">
            Upload more photos
          </n-button>
        </n-upload-trigger>
        <n-button @click="uploadList = []" :disabled="!!uploadsPending">
          Clear all
        </n-button>
      </n-space>
      <n-hr></n-hr>
      <n-scrollbar style="max-height: 250px;">
        <n-upload-file-list/>
      </n-scrollbar>
    </n-upload>
    <template #buttons>
      <n-button
          v-if="!uploadsDone"
          :disabled="!!uploadsPending"
          secondary
          size="small"
          icon-placement="left"
          style="width: 100px;"
          @click="$emit('update:open', false)">
        Cancel
      </n-button>
      <n-button
          :disabled="(!uploadList.length || !!uploadsPending) && !uploadsDone"
          :loading="!!uploadsPending"
          size="small"
          type="primary"
          icon-placement="left"
          style="width: 100px;"
          @click="uploadsDone ? $emit('update:open', false) : $refs.uploadForm?.submit()">
        {{ uploadsDone ? "Close" : "Upload" }}
      </n-button>
    </template>
  </modal>
</template>
