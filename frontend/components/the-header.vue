<script lang="ts">
import {AddOutline, CheckmarkOutline, CloudUploadOutline, TrashSharp} from '@vicons/ionicons5'

export default defineComponent({
  components: {
    AddOutline,
    CheckmarkOutline,
    CloudUploadOutline,
    TrashSharp
  },
  setup() {
    const auth = useAuth()
    const editing = useEditing()
    const themeVars = useThemeVars()

    return {auth, editing, themeVars}
  },
  data() {
    return {
      options: []
    }
  },
  computed: {
    isHome() {
      return this.$route.name == 'index'
    }
  },
  methods: {
    handleBack() {
      navigateTo('/')
    },
    handleOk() {
      this.editing.setEditing(false)
    }
  }
})
</script>

<template>
    <transition mode="out-in">
      <n-page-header v-if="!editing.isEditing()" :on-back="isHome ? undefined : handleBack">
        <template #title>
          <transition>
            <span v-if="isHome">Chroma</span>
          </transition>
        </template>
        <template #avatar>
          <transition>
            <n-avatar
                v-if="isHome"
                color="transparent"
                src="/icon.svg"
            />
          </transition>
        </template>
        <template #extra>
          <n-button
              v-if="auth.isAdmin()"
              @click="navigateTo('/album/new')">
            <template #icon>
              <add-outline/>
            </template>
            Create album
          </n-button>
        </template>
      </n-page-header>
      <n-page-header v-else :on-back="handleOk">
        <template #title>
          Edit album
        </template>
        <template #back>
          <n-icon>
            <checkmark-outline/>
          </n-icon>
        </template>
        <template #extra>
          <n-space>
            <n-button
                :bordered="false"
            >
              <template #icon>
                <cloud-upload-outline/>
              </template>
              Upload photos
            </n-button>
            <n-button
                type="error"
            >
              <template #icon>
                <trash-sharp/>
              </template>
              Delete album
            </n-button>
          </n-space>
        </template>
      </n-page-header>
    </transition>
</template>

<style scoped>
.v-enter-active,
.v-leave-active {
  transition: opacity 200ms ease;
}

.v-enter-from,
.v-leave-to {
  opacity: 0;
}
</style>
