<script lang="ts">
import {ArrowBackOutline as BackIcon} from '@vicons/ionicons5'

export default defineComponent({
  components: {
    BackIcon
  },
  computed: {
    parentRoute() {
      return this.$route.meta.parent || '/'
    },
    showBackButton() {
      return this.$route.meta.parent != undefined
    },
    showFavicon() {
      return this.$route.meta.showFavicon != false
    },
    showTitle() {
      return this.$route.meta.showTitle != false
    },
    showSearchBar() {
      return this.$route.meta.showSearchBar != false
    },
    showUserMenu() {
      return this.$route.meta.showUserMenu != false
    },
    actionComponents() {
      return this.$route.meta.actionComponents || []
    }
  }
})
</script>

<template>
  <n-space vertical>
    <n-layout>
      <n-layout-header>
        <n-grid :cols="3" layout-shift-disabled>
          <n-gi>
            <n-space align="center" style="font-size: 0; height: 100%;">
              <transition name="back">
                <nuxt-link v-if="showBackButton" :to="parentRoute">
                  <n-button text>
                    <n-icon size="22">
                      <back-icon/>
                    </n-icon>
                  </n-button>
                </nuxt-link>
              </transition>
              <transition>
                <nuxt-link
                    v-if="showFavicon"
                    :to="parentRoute">
                  <n-avatar
                      color="transparent"
                      src="/icon.svg"
                  />
                </nuxt-link>
              </transition>
              <transition>
                <nuxt-link
                    v-if="showTitle"
                    :to="parentRoute"
                    style="text-decoration: none; color: inherit">
                  <span class="album-title">Chroma</span>
                </nuxt-link>
              </transition>
            </n-space>
          </n-gi>
          <n-gi>
            <transition>
              <div v-if="showSearchBar">
                <header-search-bar/>
              </div>
            </transition>
          </n-gi>
          <n-gi>
            <n-space align="center" justify="end" style="font-size: 0; height: 100%;">
              <span v-for="component in actionComponents" :key="component">
                <component :is="component"/>
              </span>
              <transition>
                <div v-if="showUserMenu">
                  <header-user-menu/>
                </div>
              </transition>
            </n-space>
          </n-gi>
        </n-grid>
      </n-layout-header>
      <n-layout-content>
        <n-card size="small" :bordered="false">
          <keep-alive>
            <slot/>
          </keep-alive>
        </n-card>
      </n-layout-content>
    </n-layout>
  </n-space>
</template>

<style scoped>
.n-layout-header {
  padding: 10px;
}

.n-space > :deep(div:empty) {
  margin-left: -8px;
  margin-top: -12px;
}

.album-title {
  font-size: 18px;
  font-weight: 500;
}

.v-enter-active,
.v-leave-active {
  transition: opacity ease-in-out 200ms;
}

.v-enter-from,
.v-leave-to {
  opacity: 0;
}

.back-enter-active,
.back-leave-active {
  transition: opacity ease-in-out 200ms, margin ease-in-out 200ms;
}

.back-enter-from,
.back-leave-to {
  opacity: 0;
  margin-left: -100%;
}
</style>
