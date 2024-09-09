<script lang="ts">
export default defineComponent({
  props: {
    border: { type: Boolean, default: true },
    rounded: Boolean,
  },
});
</script>

<template>
  <label>
    <input
      type="checkbox"
      v-bind="$attrs"
      class="checkbox-checkbox"
      :class="{
        border,
        rounded,
      }"
    />
    <span class="checkbox-button"></span>
    <span class="checkbox-label"><slot /></span>
  </label>
</template>

<style scoped lang="postcss">
.checkbox-checkbox {
  @apply appearance-none;
  @apply w-4 h-4 mt-1;
  @apply relative shrink-0;
  @apply bg-gray-500;
  @apply opacity-40;

  &.border {
    @apply border-2 border-blue-500 rounded-sm;
  }

  &.rounded {
    @apply rounded-full;
  }

  &:checked {
    @apply bg-blue-800 border-0;
  }
}
</style>
