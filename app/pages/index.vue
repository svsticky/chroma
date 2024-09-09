<script lang="ts">
export default defineComponent({
  setup() {
    definePageMeta({
      key: "index",
    });

    useHead({
      title: "Chroma",
    });

    const models = useModels();

    const {
      data: albums,
      status,
      error,
    } = useAsyncData("albums", () => models.album.list());

    return { albums, status, error };
  },
  data() {
    return {
      showNewAlbumModal: false,
    };
  },
});
</script>

<template>
  <div class="container mx-auto">
    <header>
      <h1>Albums</h1>
      <button
        id="create-album-button"
        class="button"
        @click="showNewAlbumModal = true"
      >
        <font-awesome icon="plus" class="icon" />
        Create album
      </button>
    </header>
    <main>
      <section v-if="!error" class="album-grid">
        <div
          v-if="status === 'pending'"
          v-for="n in 10"
          class="album-card loading skeleton"
        >
          <div class="cover-photo skeleton-block"></div>
          <div class="title skeleton-text w-64 max-w-[calc(100%-2rem)]">
            Loading
          </div>
        </div>
        <nuxt-link
          v-else
          v-for="album in albums"
          :key="album.id"
          :to="`/album/${album.id}`"
          class="album-card"
        >
          <div class="cover-photo">
            <img
              v-if="album.coverPhoto?.media?.urls?.at(0)?.url"
              :src="album.coverPhoto?.media?.urls?.at(0)?.url"
              :alt="album.name || 'Untitled album'"
            />
            <div v-else class="cover-photo-placeholder">
              <font-awesome icon="image" />
            </div>
          </div>
          <div class="title">
            {{ album.name || "Untitled album" }}
          </div>
        </nuxt-link>
      </section>
      <section v-else>Oh no! Failed to obtain albums: {{ error }}</section>
    </main>
  </div>

  <modal-new-album v-model:show="showNewAlbumModal" />
</template>

<style scoped lang="postcss">
header,
main {
  @apply m-auto;
  @apply max-w-6xl;
}

header {
  @apply mb-5;
  @apply w-full flex justify-between items-center;

  h1 {
    @apply text-5xl font-extrabold dark:text-white;
  }
}

main .album-grid {
  @apply grid gap-4;
  grid-template-columns: repeat(auto-fill, minmax(250px, 1fr));

  .album-card {
    @apply block rounded-lg;

    .cover-photo {
      @apply aspect-[4/3];
      @apply bg-gray-400 dark:bg-gray-800;
      @apply rounded-lg overflow-hidden shadow;

      img {
        @apply w-full h-full object-cover;
      }

      .cover-photo-placeholder {
        @apply h-full w-full;
        @apply flex justify-center items-center;
        @apply text-5xl text-gray-500 dark:text-gray-700;
      }
    }

    .title {
      @apply m-2;
      @apply font-medium text-gray-900 dark:text-white text-ellipsis;
      @apply overflow-hidden;
    }
  }
}
</style>
