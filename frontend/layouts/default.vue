<template>
  <n-space vertical>
    <n-layout>
      <n-layout-header>
        <the-header/>
      </n-layout-header>
      <n-layout-content>
        <n-card size="small" :bordered="false">
          <keep-alive>
            <slot/>
          </keep-alive>
        </n-card>
      </n-layout-content>
      <!--      <the-footer/>-->
    </n-layout>
  </n-space>
</template>

<style scoped>
.n-layout-header {
  padding: 10px;
}
</style>
<script setup lang="ts">
</script>
