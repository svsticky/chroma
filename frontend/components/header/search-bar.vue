<script lang="ts">
import {SearchOutline as SearchIcon} from '@vicons/ionicons5'

export default defineComponent({
  components: {
    SearchIcon
  }
})
</script>

<template>
  <n-input round placeholder="Search" clearable>
    <template #prefix>
      <n-icon>
        <search-icon/>
      </n-icon>
    </template>
  </n-input>
</template>

<style scoped>

</style>
