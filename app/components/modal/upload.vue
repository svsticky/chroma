<script lang="ts">
import type { UiModal } from "#components";
import type { Upload } from "~/components/modal/ui/upload-item.vue";

export default defineComponent({
  props: {
    show: Boolean,
  },
  emits: ["update:show", "upload"],
  data() {
    return {
      uploads: [] as Upload[],
      status: "ready" as "ready" | "pending" | "done",
    };
  },
  methods: {
    closeModal() {
      this.$emit("update:show", false);
    },
    openFileDialog() {
      return (
        this.$refs.dropZone as ComponentPublicInstance<typeof UiModal>
      ).openFileDialog();
    },
    filesHandler(files: File[]) {
      this.status = "ready";
      this.uploads = this.uploads.concat(
        files.map((file) => ({ file, status: "idle", progress: 0 })),
      );
    },
    async uploadFiles() {
      this.status = "pending";

      await new Promise((resolve, reject) =>
        this.$emit("upload", this.uploads, resolve, reject),
      );

      this.status = "done";
    },
  },
  computed: {
    hasUploads() {
      return this.uploads.length > 0;
    },
  },
});
</script>

<template>
  <ui-modal
    v-bind:show="show"
    @update:show="$emit('update:show', $event)"
    class="upload-modal"
    @after-leave="uploads = []"
  >
    <template #header>Upload photos</template>
    <template #default>
      <ui-drop-zone
        v-show="uploads.length === 0"
        ref="dropZone"
        @update:files="filesHandler"
        class="w-screen max-w-4xl h-64"
      />
      <div v-show="uploads.length > 0">
        <div class="upload-actions mb-2 flex">
          <button
            class="button ml-auto"
            @click="openFileDialog"
            :disabled="status === 'pending'"
          >
            Add more
          </button>
          <button
            class="button ml-2"
            @click="uploads = []"
            :disabled="status === 'pending'"
          >
            Clear
          </button>
        </div>
        <ul
          class="upload-list w-screen h-full max-w-4xl min-h-64 max-h-96 overflow-scroll"
        >
          <modal-ui-upload-item
            v-for="upload in uploads"
            class="upload-item"
            :upload="upload"
          />
        </ul>
      </div>
    </template>
    <template #footer>
      <div class="button-row">
        <button
          v-if="hasUploads"
          class="button button-primary button-sm"
          @click="uploadFiles"
          :disabled="status !== 'ready'"
        >
          Upload
        </button>
        <button
          class="button button-sm"
          @click="closeModal"
          :disabled="status === 'pending'"
        >
          {{ hasUploads && status != "done" ? "Cancel" : "Close" }}
        </button>
      </div>
    </template>
  </ui-modal>
</template>

<style scoped lang="postcss">
.upload-modal {
  .upload-list {
    @apply flex flex-col gap-y-2;

    .upload-item {
      @apply p-1.5;
      @apply flex gap-x-2 items-center;
      @apply dark:bg-gray-800;
      @apply rounded;
    }
  }

  .button-row {
    @apply flex gap-x-2 justify-end w-full;
  }
}
</style>
