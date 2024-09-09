<script lang="ts">
export default defineComponent({
  props: {
    show: Boolean,
    albumId: { type: String, required: true },
  },
  emits: ["update:show"],
  setup() {
    const models = useModels();

    return { models };
  },
  data() {
    return { loading: false };
  },
  methods: {
    closeModal() {
      this.$emit("update:show", false);
    },
    async deleteAlbum() {
      this.loading = true;

      try {
        await this.models.album.delete(this.albumId);

        navigateTo(`/`);
      } catch (e) {
        console.error(e);
      }

      this.loading = false;
    },
  },
});
</script>

<template>
  <ui-modal
    v-bind:show="show"
    @update:show="$emit('update:show', $event)"
    class="delete-album-modal"
  >
    <template #header>Delete album</template>
    <template #default
      >Are you sure you want to delete the album? This cannot be undone.
    </template>
    <template #footer>
      <div class="button-row">
        <button
          type="submit"
          form="new-album-form"
          class="button button-error button-sm"
          @click="deleteAlbum"
          :disabled="loading"
        >
          <template v-if="loading">
            <font-awesome icon="circle-notch" spin />
          </template>
          <template v-else>Delete</template>
        </button>
        <button
          class="button button-sm"
          :disabled="loading"
          @click="closeModal"
        >
          Cancel
        </button>
      </div>
    </template>
  </ui-modal>
</template>

<style scoped lang="postcss">
.delete-album-modal {
  .button-row {
    @apply flex gap-x-2 justify-end w-full;
  }
}
</style>
