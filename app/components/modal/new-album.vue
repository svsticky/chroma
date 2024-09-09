<script lang="ts">
export default defineComponent({
  props: {
    show: Boolean,
  },
  emits: ["update:show", "upload"],
  setup() {
    const models = useModels();

    return { models };
  },
  data() {
    return {
      loading: false,
      form: {
        name: "",
      },
    };
  },
  methods: {
    resetForm() {
      for (const key of Object.keys(this.form)) {
        if (typeof this.form[key as keyof typeof this.form] === "string") {
          this.form[key as keyof typeof this.form] = "";
        } else {
          console.warn(
            "Trying to reset invalid type:",
            typeof this.form[key as keyof typeof this.form],
          );
        }
      }
    },
    closeModal() {
      this.$emit("update:show", false);
    },
    async createAlbum() {
      this.loading = true;

      try {
        const album = await this.models.album.create(this.form.name);

        navigateTo(`/album/${album.id}`);
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
    class="new-album-modal"
    @after-leave="resetForm"
  >
    <template #header>Create album</template>
    <template #default>
      <form
        @submit.prevent="createAlbum"
        id="new-album-form"
        class="w-96 max-w-full"
      >
        <label class="form-group">
          <span class="form-label">Album name:</span>
          <input
            v-model="form.name"
            type="text"
            class="form-input"
            placeholder="Untitled album"
          />
        </label>
      </form>
    </template>
    <template #footer>
      <div class="button-row">
        <button
          type="submit"
          form="new-album-form"
          class="button button-primary button-sm"
          :disabled="loading"
        >
          <template v-if="loading">
            <font-awesome icon="circle-notch" spin />
          </template>
          <template v-else>Create</template>
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
.new-album-modal {
  .button-row {
    @apply flex gap-x-2 justify-end w-full;
  }
}
</style>
