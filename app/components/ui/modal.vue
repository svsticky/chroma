<script lang="ts">
export default defineComponent({
  props: {
    show: Boolean,
  },
  emits: ["update:show"],
  methods: {
    closeModal() {
      this.$emit("update:show", false);
    },
  },
});
</script>

<template>
  <!--  Use tab navigation fix: https://dev.to/bogdanfromkyiv/enhancing-web-accessibility-locking-the-tab-button-within-modals-and-menus-3774 -->
  <transition name="modal-94e1ef14">
    <div v-show="show" tabindex="-1" class="modal">
      <div class="modal-window">
        <header>
          <slot name="header">Modal</slot>
        </header>
        <main>
          <slot>
            <p class="w-[500px] h-32">Place your content here!</p>
          </slot>
        </main>
        <footer>
          <slot name="footer">
            <button
              class="button button-primary button-sm ml-auto"
              @click="closeModal"
            >
              Close
            </button>
          </slot>
        </footer>
      </div>
    </div>
  </transition>
</template>

<style scoped lang="postcss">
.modal {
  @apply fixed;
  @apply left-0 right-0 top-[-10px] bottom-0 z-50;
  @apply flex items-center justify-center;
  @apply backdrop-blur-sm bg-gray-300/75 dark:bg-gray-900/75;

  .modal-window {
    @apply m-4;
    @apply max-w-full;
    @apply bg-white rounded shadow dark:bg-gray-700;

    header,
    main,
    footer {
      @apply p-4;
    }

    header {
      @apply flex items-center justify-between;
      @apply border-b rounded-t dark:border-gray-600;
      @apply text-lg font-semibold text-gray-900 dark:text-white;
    }

    footer {
      @apply flex;
      @apply bg-gray-50 dark:bg-gray-700;
      @apply dark:border-t dark:border-gray-600;
      @apply rounded-b;
    }
  }
}

.modal-94e1ef14-enter-active,
.modal-94e1ef14-leave-active {
  @apply transition-all;
}

.modal-94e1ef14-enter-from,
.modal-94e1ef14-leave-to {
  @apply opacity-0;
  @apply translate-y-[10px];
}
</style>
