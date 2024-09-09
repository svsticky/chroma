<script lang="ts">
export default defineComponent({
  props: {
    show: Boolean,
    selectedPhotos: { type: Set<string>, default: new Set<string>() },
  },
  emits: ["update:selectedPhotos"],
  methods: {
    clearSelection() {
      this.selectedPhotos.clear();
      this.$emit("update:selectedPhotos", this.selectedPhotos);
    },
  },
});
</script>

<template>
  <transition name="selection-toolbar-k17wdx8a">
    <div v-show="show" class="selection-toolbar">
      <section class="selection-toolbar-left">
        <button
          class="selection-toolbar-button selection-discard"
          @click="clearSelection"
        >
          <font-awesome icon="xmark" />
        </button>
        <span class="selection-toolbar-label"
          >{{ selectedPhotos.size }} selected</span
        >
      </section>
      <section class="selection-toolbar-right">
        <slot />
      </section>
    </div>
  </transition>
</template>

<style scoped lang="postcss">
.selection-toolbar {
  @apply absolute;
  @apply p-2.5;
  @apply left-0 bottom-0 right-0;
  @apply flex justify-between;
  @apply shadow shadow-blue-200/75 dark:shadow-blue-900/75;
  @apply bg-blue-200 dark:bg-blue-900;
  @apply rounded-lg;

  & > section {
    @apply flex gap-x-2 items-center;

    .selection-toolbar-label {
      @apply font-medium;
    }

    & > :deep(button) {
      @apply w-8 h-8 flex justify-center items-center;
      @apply text-blue-950 dark:text-blue-50;
      @apply rounded-full;

      &:hover,
      &:focus {
        @apply bg-blue-100 dark:bg-blue-800;
      }
    }
  }
}
</style>
