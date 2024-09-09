<script lang="ts">
import { Photo, type PhotoUrl } from "~/proto/entity/photo";
import { Album } from "~/proto/entity/album";
import type { AsyncDataRequestStatus } from "#app";

type QualityPreference = {
  [key: string]: number;
};

const qualityPreference: QualityPreference = {
  Thumbnail: 0,
  Preview: 1,
  Original: 2,
};

export default defineComponent({
  emits: [
    "update:album",
    "update:editingTitle",
    "update:savingTitle",
    "update:savingPublished",
    "update:savingCoverPhoto",
  ],
  props: {
    album: {
      type: Object as PropType<ReturnType<Album["toObject"]> | null>,
      required: true,
    },
    albumStatus: {
      type: String as PropType<AsyncDataRequestStatus>,
      required: true,
    },
    editingTitle: Boolean,
    savingTitle: Boolean,
    savingPublished: Boolean,
    savingCoverPhoto: Boolean,
  },
  setup() {
    const models = useModels();

    return { models };
  },
  data() {
    return {
      titleValue: "",
      publishStatus: false,
    };
  },
  methods: {
    editTitle() {
      this.$emit("update:editingTitle", true);
      this.titleValue = this.albumName;
      this.$nextTick(() => {
        (
          this.$refs.titleInput as ComponentPublicInstance<HTMLInputElement>
        ).focus();
      });
    },
    async saveTitle() {
      this.$emit("update:savingTitle", true);

      try {
        if (!this.album?.id) {
          throw new Error("Cannot update title of undefined album");
        }

        this.$emit(
          "update:album",
          await this.models.album.update(this.album.id, {
            name: this.titleValue,
          }),
        );

        this.$emit("update:editingTitle", false);
      } catch (e) {
        // Todo send toast
        console.error(e);
      }

      this.$emit("update:savingTitle", false);
    },
    discardTitle() {
      this.$emit("update:editingTitle", false);
    },
    async updatePublished() {
      this.$emit("update:savingPublished", true);

      try {
        if (!this.album?.id) {
          throw new Error("Cannot update published status of undefined album");
        }

        this.$emit(
          "update:album",
          await this.models.album.update(this.album.id, {
            published: !this.album.published,
          }),
        );
      } catch (e) {
        this.albumIsPublished = this.album?.published || false;
        console.error(e);
      }

      this.$emit("update:savingPublished", false);
    },
    async saveAlbumCoverPhoto(photoId: string) {
      this.$emit("update:savingCoverPhoto", true);

      try {
        if (!this.album?.id) {
          throw new Error("Cannot update title of undefined album");
        }

        this.$emit(
          "update:album",
          await this.models.album.update(this.album.id, {
            coverPhoto: {
              id: photoId,
            },
          }),
        );
      } catch (e) {
        // Todo send toast
        console.error(e);
      }

      this.$emit("update:savingCoverPhoto", false);
    },
    onPhotoLoaded(event: Event) {
      (event.target as ComponentPublicInstance<HTMLImageElement>).classList.add(
        "loaded",
      );
    },
  },
  computed: {
    albumName() {
      return this.album?.name || "Untitled album";
    },
    coverPhotoUrl() {
      const bestUrl = {
        score: null as number | null,
        url: null as string | null,
      };

      this.album?.coverPhoto?.media?.urls?.forEach(
        (url: ReturnType<PhotoUrl["toObject"]>) => {
          const size = url.size || "";

          if (!qualityPreference.hasOwnProperty(size)) return;

          if (
            bestUrl.score === null ||
            qualityPreference[size] < bestUrl.score
          ) {
            bestUrl.score = qualityPreference[size];
            bestUrl.url = url.url || null;
          }
        },
      );

      return bestUrl.url;
    },
    albumCreator() {
      return this.album?.createdBy?.name || "Unknown";
    },
    albumCreated() {
      return this.album?.createdAt
        ? new Date(this.album.createdAt * 1000).toLocaleDateString(undefined, {
            year: "numeric",
            month: "2-digit",
            day: "2-digit",
          })
        : "Unknown";
    },
    albumPublisher() {
      return this.album?.publishedBy?.name || "Unknown";
    },
    albumPublished() {
      return this.album?.publishedAt
        ? new Date(this.album.publishedAt * 1000).toLocaleDateString(
            undefined,
            {
              year: "numeric",
              month: "2-digit",
              day: "2-digit",
            },
          )
        : "Unknown";
    },
    albumIsPublished: {
      get() {
        return this.album?.published || this.publishStatus;
      },
      set(v: boolean) {
        this.publishStatus = v;
      },
    },
  },
});
</script>

<template>
  <article class="album-info" :class="{ skeleton: albumStatus === 'pending' }">
    <template v-if="albumStatus === 'pending'">
      <figure class="cover-photo skeleton-block"></figure>
      <div class="album-info-container">
        <div class="album-title">
          <h1 class="skeleton-text w-96">Loading...</h1>
        </div>
        <p>
          <span class="skeleton-text w-64 mr-2">Loading...</span>
          <span class="skeleton-text w-64">Loading...</span>
        </p>
      </div>
    </template>
    <template v-else-if="albumStatus === 'success'">
      <figure class="cover-photo">
        <transition name="cover-photo-433a9abd">
          <div v-if="savingCoverPhoto" class="cover-photo-saving">
            <font-awesome icon="circle-notch" spin />
          </div>
          <div v-else-if="!coverPhotoUrl" class="cover-photo-placeholder">
            <font-awesome icon="image" />
          </div>
          <img
            v-else
            :src="coverPhotoUrl!"
            :alt="albumName"
            @load="onPhotoLoaded"
          />
        </transition>
      </figure>
      <div class="album-info-container">
        <div class="album-title">
          <nuxt-link
            v-if="!editingTitle"
            tabindex="0"
            @click.prevent="editTitle"
            @keydown.enter.prevent="editTitle"
          >
            <h1>{{ albumName }}</h1>
            <font-awesome icon="pencil" class="icon" />
          </nuxt-link>
          <form
            v-else
            class="album-title-edit"
            @submit.prevent="saveTitle"
            @keydown.esc="discardTitle"
          >
            <input
              type="text"
              ref="titleInput"
              class="form-input"
              v-model="titleValue"
              :disabled="savingTitle"
            />
            <button
              type="submit"
              class="button button-primary button-sm w-24"
              :disabled="savingTitle"
            >
              <font-awesome v-if="savingTitle" icon="circle-notch" spin />
              <span v-else>Save</span>
            </button>
            <button
              class="button button-sm w-24"
              :disabled="savingTitle"
              @click="discardTitle"
            >
              Cancel
            </button>
          </form>
        </div>
        <p>
          Created by {{ albumCreator }} at {{ albumCreated }}
          <span v-if="album?.published">
            &#x2022; Published by {{ albumPublisher }} at {{ albumPublished }}
          </span>
        </p>
        <nav class="actions">
          <slot name="actions" />

          <ui-switch
            v-model="albumIsPublished"
            :disabled="editingTitle || savingPublished"
            @change="updatePublished"
            >Published
          </ui-switch>
        </nav>
      </div>
    </template>
  </article>
</template>

<style scoped lang="postcss">
article.album-info {
  @apply mb-5;
  @apply flex gap-x-4;

  .cover-photo {
    @apply w-64;
    @apply aspect-[4/3];
    @apply bg-gray-400 dark:bg-gray-800;
    @apply rounded-lg overflow-hidden shadow;
    @apply shrink-0;

    img {
      @apply w-full h-full object-cover opacity-0 transition-opacity;

      &.loaded {
        @apply opacity-100;
      }
    }

    .cover-photo-saving,
    .cover-photo-placeholder {
      @apply h-full w-full;
      @apply flex justify-center items-center;
      @apply text-5xl text-gray-500 dark:text-gray-700;
    }
  }

  .album-info-container {
    @apply p-5 -m-5;
    @apply overflow-hidden;
    @apply flex-grow flex flex-col;

    .album-title {
      & > a {
        @apply inline-flex items-start gap-x-2 rounded-lg;
        @apply max-w-full;

        &:hover .icon {
          @apply text-gray-900 dark:text-white;
        }
      }

      h1 {
        @apply text-4xl font-bold dark:text-white text-ellipsis;
        @apply overflow-hidden;
      }

      .icon {
        @apply text-sm text-gray-500 dark:text-gray-400;
      }
    }

    .album-title-edit {
      @apply flex gap-x-2;
    }

    p {
      @apply text-gray-500 italic;
    }

    nav {
      @apply mt-auto flex gap-x-2;
    }
  }
}

.cover-photo-433a9abd-enter-active,
.cover-photo-433a9abd-leave-active {
  @apply transition-opacity;
}

.cover-photo-433a9abd-enter-from,
.cover-photo-433a9abd-leave-to {
  @apply opacity-0;
}
</style>
