<script lang="ts">
export default defineComponent({});
</script>

<template>
  <label class="switch">
    <input v-bind="$attrs" type="checkbox" class="switch-checkbox" />
    <span class="switch-button"></span>
    <span class="switch-label"><slot /></span>
  </label>
</template>

<style scoped lang="postcss">
.switch {
  @apply inline-flex items-center cursor-pointer ml-auto;

  & &-checkbox {
    @apply sr-only;

    & ~ .switch-button {
      @apply w-11 h-6;
      @apply relative;
      @apply rounded-full;
      @apply bg-gray-200 dark:bg-gray-700;
      @apply dark:border-gray-600;

      &::after {
        @apply content-[''];
        @apply top-[2px] start-[2px] h-5 w-5;
        @apply absolute;
        @apply rounded-full;
        @apply bg-white;
        @apply border border-gray-300;
        @apply transition-all;
      }
    }

    &:focus ~ .switch-button {
      @apply outline outline-4 outline-blue-300 dark:outline-blue-900;
    }

    &:checked ~ .switch-button {
      @apply bg-blue-600;

      &::after {
        @apply translate-x-full rtl:-translate-x-full;
        @apply border-white;
      }
    }

    &:disabled ~ .switch-button {
      &::after {
        @apply dark:bg-gray-500;
        @apply dark:border-gray-500;
      }
    }

    &:disabled:checked ~ .switch-button {
      @apply bg-blue-200 dark:bg-blue-900;
    }

    &:disabled ~ .switch-label {
      @apply text-gray-400 dark:text-gray-500;
    }
  }

  & &-label {
    @apply ms-3;
    @apply text-sm font-medium text-gray-900 dark:text-gray-300;
  }
}
</style>
