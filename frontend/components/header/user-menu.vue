<script lang="ts">
import {NIcon} from 'naive-ui'
import {
  CreateOutline as EditIcon,
  LogOutOutline as LogoutIcon,
  PersonCircleOutline as ProfileIcon
} from '@vicons/ionicons5'

export default defineComponent({
  setup() {
    const renderIcon = (icon: Component) => {
      return () => {
        return h(NIcon, null, {
          default: () => h(icon)
        })
      }
    }

    return {
      options: [
        {
          label: 'Profile',
          key: 'profile',
          icon: renderIcon(ProfileIcon)
        },
        {
          label: 'Edit Profile',
          key: 'editProfile',
          icon: renderIcon(EditIcon)
        },
        {
          label: 'Logout',
          key: 'logout',
          icon: renderIcon(LogoutIcon)
        }
      ]
    }
  },
  data() {
    return {
      firstName: 'Mervin',
      infix: 'de',
      lastName: 'Jong'
    }
  },
  computed: {
    name() {
      return [this.firstName, this.infix, this.lastName].filter(v => v).join(' ')
    },
    initials() {
      return this.firstName.charAt(0) + this.lastName.charAt(0)
    },
    avatarColor() {
      let hash = 0
      for (let i = 0; i < this.name.length; i++) {
        hash = this.name.charCodeAt(i) + ((hash << 5) - hash)
      }

      return `hsl(${hash % 360}, 70%, 50%)`
    }
  }
})
</script>

<template>
  <n-dropdown size="large" trigger="click" placement="bottom-end" :options="options">
    <n-button text round>
      <n-avatar round :style="`background-color: ${avatarColor}`">
        {{ initials }}
      </n-avatar>
    </n-button>
  </n-dropdown>
</template>

<style scoped>

</style>
