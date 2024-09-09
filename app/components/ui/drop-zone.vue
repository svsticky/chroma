<script lang="ts">
export default defineComponent({
  emits: ["update:inDropZone", "update:files"],
  data() {
    return {
      inDropZone: false,
    };
  },
  methods: {
    openFileDialog() {
      (
        this.$refs
          .dropzoneFileInput as ComponentPublicInstance<HTMLInputElement>
      ).click();
    },
    dropZoneEnter() {
      this.$emit("update:inDropZone", (this.inDropZone = true));
    },
    dropZoneLeave() {
      this.$emit("update:inDropZone", (this.inDropZone = false));
    },
    dropHandler(e: DragEvent) {
      this.inDropZone = false;

      if (!e.dataTransfer) return;

      if (e.dataTransfer.items) {
        return this.uploadHandler(
          [...e.dataTransfer.items]
            .filter((item, i) => item.kind === "file")
            .map((item) => item.getAsFile()!),
        );
      } else {
        return this.uploadHandler([...e.dataTransfer.files]);
      }
    },
    fileHandler(e: Event) {
      const files = (e.target as HTMLInputElement).files;

      if (files) {
        return this.uploadHandler([...files]);
      }
    },
    uploadHandler(files: File[]) {
      this.$emit("update:files", files);
    },
  },
});
</script>

<template>
  <div class="flex items-center justify-center">
    <label
      for="dropzone-file"
      class="drop-zone"
      :class="{ 'drop-zone-ready': inDropZone }"
      @drop.prevent="dropHandler"
      @dragover.prevent
      @dragenter="dropZoneEnter"
      @dragleave="dropZoneLeave"
    >
      <span v-if="!inDropZone" class="drop-zone-inner">
        <slot name="icon">
          <svg
            class="drop-zone-icon"
            aria-hidden="true"
            xmlns="http://www.w3.org/2000/svg"
            fill="none"
            viewBox="0 0 20 16"
          >
            <path
              stroke="currentColor"
              stroke-linecap="round"
              stroke-linejoin="round"
              stroke-width="2"
              d="M13 13h3a3 3 0 0 0 0-6h-.025A5.56 5.56 0 0 0 16 6.5 5.5 5.5 0 0 0 5.207 5.021C5.137 5.017 5.071 5 5 5a4 4 0 0 0 0 8h2.167M10 15V6m0 0L8 8m2-2 2 2"
            />
          </svg>
        </slot>
        <span class="drop-zone-caption">
          <slot name="caption">
            <span class="font-semibold">Click to upload</span> or drag and drop
          </slot>
        </span>
        <span class="drop-zone-type-hint"><slot name="type-hint" /></span>
      </span>
      <span v-else class="drop-zone-inner">
        <slot name="drop-icon">
          <font-awesome icon="check" class="drop-zone-icon" />
        </slot>
        <span class="drop-zone-caption font-semibold">
          <slot name="drop-caption">Release files to upload</slot>
        </span>
      </span>
      <input
        ref="dropzoneFileInput"
        id="dropzone-file"
        type="file"
        class="hidden"
        multiple
        @change.prevent="fileHandler"
      />
    </label>
  </div>
</template>

<style scoped lang="postcss">
.drop-zone {
  @apply flex flex-col items-center justify-center;
  @apply w-full h-full;
  @apply rounded-lg;
  @apply border-2 border-dashed border-gray-300 dark:border-gray-600;
  @apply cursor-pointer;
  @apply bg-gray-50 dark:bg-gray-800;

  &:hover {
    @apply bg-gray-100 dark:bg-gray-900;
    @apply dark:border-gray-500;
  }

  &.drop-zone-ready {
    @apply border-red-500;
  }

  .drop-zone-inner {
    @apply pt-5 pb-6;
    @apply flex flex-col items-center justify-center;
    @apply pointer-events-none;

    .drop-zone-icon {
      @apply mb-4;
      @apply w-8 h-8;
      @apply text-gray-500 dark:text-gray-400;
    }

    .drop-zone-caption {
      @apply mb-2;
      @apply text-sm text-gray-500 dark:text-gray-400;
    }

    .drop-zone-type-hint {
      @apply text-xs text-gray-500 dark:text-gray-400;
    }
  }
}
</style>
