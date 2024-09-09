<script lang="ts">
import type { Album } from "~/proto/entity/album";
import { Photo } from "~/proto/entity/photo";
import { CreatePhotoRequest } from "~/proto/payload/v2/photos/create";
import type { Upload } from "~/components/modal/ui/upload-item.vue";
import { AlbumInfo } from "#components";
import SelectionToolbar from "~/components/album/selection-toolbar.vue";

export default defineComponent({
  components: { SelectionToolbar },
  emits: ["update:photos"],
  props: {
    albumId: { type: String, required: true },
  },
  setup({ albumId }, { emit }) {
    const models = useModels();
    const localAlbum = ref(null as ReturnType<Album["toObject"]> | null);

    const {
      data: album,
      status: albumStatus,
      error: albumError,
      refresh: refreshAlbum,
    } = useAsyncData(albumId, () =>
      localAlbum.value
        ? (async () => localAlbum.value)()
        : models.album.get(albumId),
    );

    const {
      data: photos,
      status: photosStatus,
      error: photosError,
    } = useAsyncData(`${albumId}_photos`, () => models.photo.search(albumId));

    emit("update:photos", photos);

    return {
      models,
      albumId,
      album,
      localAlbum,
      albumStatus,
      albumError,
      refreshAlbum,
      photos,
      photosStatus,
      photosError,
    };
  },
  data() {
    return {
      showUploadModal: false,
      showDeleteAlbumModal: false,
      albumInfo: {
        editingTitle: false,
        savingTitle: false,
        savingPublished: false,
        savingCoverPhoto: false,
      },
      deletingAlbum: false,
      selectedPhotos: new Set<string>(),
      deletingPhotos: false,
    };
  },
  methods: {
    async onAlbumUpdate(newAlbum: ReturnType<Album["toObject"]>) {
      this.localAlbum = newAlbum;
      await this.refreshAlbum();
      this.localAlbum = null;
    },
    async fileUpload(uploads: Upload[], resolve: () => void) {
      for (const upload of uploads) {
        if (upload.status != "idle") continue;

        upload.status = "pending";

        try {
          const { response, progress } = net.upload<Photo>(
            Photo,
            "/api/v2/photos",
            {
              body: CreatePhotoRequest.fromObject({
                albumId: this.albumId,
                data: new Uint8Array(await upload.file.arrayBuffer()),
              }).serializeBinary(),
            },
          );

          (upload.progress as unknown as Ref<number>) = progress;

          const newPhoto = (await response).toObject();

          if (!this.photos) {
            this.photos = [newPhoto];
          } else {
            this.photos.push(newPhoto);
          }

          upload.status = "done";
        } catch (error) {
          upload.status = "error";
          console.error(error);
        }
      }

      return resolve();
    },
    setAlbumCoverPhoto() {
      // Get the photo id
      const photoId = this.selectedPhotos.values().next().value;

      // Start update
      (
        this.$refs.albumInfo as ComponentPublicInstance<typeof AlbumInfo>
      ).saveAlbumCoverPhoto(photoId);

      // Deselect the photo
      this.selectedPhotos.clear();
    },
    async deletePhotos() {
      this.deletingPhotos = true;

      try {
        if (!this.selectedPhotos.size) {
          throw new Error("Cannot delete a selection of 0 photos");
        }

        // Delete the photos
        await this.models.photo.batchDelete(Array.from(this.selectedPhotos));

        // Unset the cover photo
        if (
          this.album?.coverPhoto?.id &&
          this.selectedPhotos.has(this.album.coverPhoto.id)
        ) {
          this.album.coverPhoto = undefined;
        }

        // Remove the photos from the list
        this.photos =
          this.photos?.filter(
            (photo) => !photo.id || !this.selectedPhotos.has(photo.id),
          ) || null;

        // Clear the selection (as photos have been deleted)
        this.selectedPhotos.clear();
      } catch (e) {
        // Todo send toast
        console.error(e);
      }

      this.deletingPhotos = false;
    },
  },
  computed: {
    albumErrorMessage() {
      switch (this.albumError?.statusCode) {
        case 404:
          return "This album does not exist";
        case 500:
          return "Whoops, crashed the server ;(";
        case undefined:
          return `Error: ${this.albumError?.toString()}`;
        default:
          return this.albumError?.statusMessage;
      }
    },
    selectionMode() {
      return this.selectedPhotos.size > 0;
    },
  },
  watch: {
    photos(newValue: ReturnType<Photo["toObject"]>[]) {
      this.$emit("update:photos", newValue);
    },
  },
});
</script>

<template>
  <Head>
    <Title v-if="albumStatus === 'success'"
      >Chroma - {{ album?.name || "Untitled album" }}</Title
    >
    <Title v-else>Chroma</Title>
  </Head>

  <article class="album-page" :class="{ 'album-error': albumError }">
    <template v-if="!albumError">
      <header>
        <album-info
          ref="albumInfo"
          class="album-info"
          :class="{ 'selection-mode': selectionMode }"
          :album="album"
          :album-status="albumStatus"
          @update:album="onAlbumUpdate"
          v-model:editing-title="albumInfo.editingTitle"
          v-model:saving-title="albumInfo.savingTitle"
          v-model:saving-published="albumInfo.savingPublished"
          v-model:saving-cover-photo="albumInfo.savingCoverPhoto"
        >
          <template #actions>
            <button
              id="upload-photos-button"
              class="button"
              :disabled="albumInfo.editingTitle"
              @click="showUploadModal = true"
            >
              <font-awesome icon="image" class="icon" />
              Add photos
            </button>
            <button
              id="delete-album-button"
              class="button"
              :disabled="albumInfo.editingTitle"
              @click="showDeleteAlbumModal = true"
            >
              <font-awesome icon="trash" class="icon" />
              Delete album
            </button>
          </template>
        </album-info>
        <album-selection-toolbar
          v-model:selected-photos="selectedPhotos"
          :show="selectionMode"
        >
          <button
            v-show="selectedPhotos.size === 1"
            class="selection-set-cover-photo"
            @click="setAlbumCoverPhoto"
          >
            <font-awesome icon="images" />
          </button>
          <button class="selection-add-to-album">
            <font-awesome icon="plus" />
          </button>
          <button class="selection-download">
            <font-awesome icon="download" />
          </button>
          <button class="selection-delete" @click="deletePhotos">
            <font-awesome icon="trash" />
          </button>
        </album-selection-toolbar>
      </header>
      <main>
        <album-photo-grid
          v-if="!photosError"
          v-model:selected-photos="selectedPhotos"
          :album-id="albumId"
          :photos="photos"
          :photos-status="photosStatus"
          :class="{ 'edit-mode': albumInfo.editingTitle }"
          class="album-photo-grid"
        />
        <div v-else>Oh no! Failed to obtain images: {{ photosError }}</div>
      </main>
    </template>
    <template v-else>
      <main>
        <h1 class="status-message">
          {{ albumError.statusCode || "Oh no!" }} {{ albumErrorMessage }}
        </h1>
        <p class="default-text">That isn't supposed to happen!</p>
        <nav class="links">
          <!-- @formatter:off !-->
          Go back <nuxt-link to="/">Home</nuxt-link>.
          <!--  @formatter:on !-->
        </nav>
      </main>
      <aside>
        <img src="/img/error-page.png" alt="Broken camera clipart" />
      </aside>
    </template>
  </article>

  <modal-upload v-model:show="showUploadModal" @upload="fileUpload" />
  <modal-delete-album v-model:show="showDeleteAlbumModal" :album-id="albumId" />
</template>

<style scoped lang="postcss">
article.album-page {
  & > header {
    @apply relative;

    .album-info {
      @apply transition-opacity;

      &.selection-mode {
        /* Todo: Properly disable the element, rather than ignore pointer events */
        @apply opacity-50;
        @apply pointer-events-none select-none;
      }

      #delete-album-button:hover {
        @apply bg-red-500 text-white;
      }
    }
  }

  & > main {
    .album-photo-grid {
      @apply transition-opacity;

      &.edit-mode {
        /* Todo: Properly disable the element, rather than ignore pointer events */
        @apply opacity-50;
        @apply pointer-events-none select-none;
      }
    }
  }
}

article.album-page.album-error {
  @apply flex flex-1 items-center justify-around;

  & > main {
    .status-message {
      @apply my-2.5;
      @apply text-2xl font-bold text-blue-500;
    }

    .default-text {
      @apply my-2.5;
      @apply text-3xl font-bold;
    }

    .links {
      @apply my-2.5;
      @apply text-gray-400;

      a {
        @apply underline;
      }
    }
  }

  aside img {
    @apply max-w-96;
  }
}

.selection-toolbar-k17wdx8a-enter-active,
.selection-toolbar-k17wdx8a-leave-active {
  @apply transition-all;
}

.selection-toolbar-k17wdx8a-enter-from,
.selection-toolbar-k17wdx8a-leave-to {
  @apply opacity-0;
  @apply translate-y-[10px];
}
</style>