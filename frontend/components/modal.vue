<script lang="ts">

export default defineComponent({
  props: {
    show: Boolean,
    title: String
  },
  emits: ['update:show', 'close']
})
</script>

<template>
  <n-modal
      :show="show"
      @update:show="$emit('update:show', $event)"
      @after-leave="$emit('close')"
      preset="card"
      :title="title"
      transform-origin="center"
      :bordered="false"
      :mask-closable="false"
      :closable="false"
      size="small"
      role="dialog"
      aria-modal="true"
  >
    <slot/>
    <template #action>
      <n-space justify="end">
        <slot name="buttons">
          <n-button
              size="small"
              type="primary"
              icon-placement="left"
              style="width: 100px;"
              @click="$emit('update:show', false)">
            Ok
          </n-button>
        </slot>
      </n-space>
    </template>
  </n-modal>
</template>

<style scoped>

</style>
