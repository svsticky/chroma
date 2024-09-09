<script lang="ts">
import type { AsyncDataRequestStatus } from "#app";
import type { Photo, PhotoUrl } from "~/proto/entity/photo";
import { JustifiedInfiniteGrid } from "@egjs/vue3-infinitegrid";

type QualityPreference = {
  [key: string]: number;
};

const qualityPreference: QualityPreference = {
  Thumbnail: 0,
  Preview: 1,
  Original: 2,
};

export default defineComponent({
  components: {
    JustifiedInfiniteGrid,
  },
  emits: ["update:photos", "update:selectedPhotos"],
  props: {
    albumId: { type: String, required: true },
    photos: {
      type: Object as PropType<ReturnType<Photo["toObject"]>[] | null>,
      required: true,
    },
    photosStatus: {
      type: String as PropType<AsyncDataRequestStatus>,
      required: true,
    },
    selectedPhotos: Set<string>,
  },
  data() {
    return {
      renderComplete: false,
      bestUrls: {} as {
        [key: string]: ReturnType<PhotoUrl["toObject"]> | null;
      },
      selectedPhotos: new Set<string>(),
    };
  },
  methods: {
    hasPhotoUrl(photo: ReturnType<Photo["toObject"]>) {
      return photo.media?.urls?.length;
    },
    getPhotoUrl(photo: ReturnType<Photo["toObject"]>) {
      const bestMatch = {
        score: null as number | null,
        photoUrl: null as ReturnType<PhotoUrl["toObject"]> | null,
      };

      if (photo.id && this.bestUrls.hasOwnProperty(photo.id)) {
        return this.bestUrls[photo.id];
      }

      photo.media?.urls?.forEach(
        (photoUrl: ReturnType<PhotoUrl["toObject"]>) => {
          const size = photoUrl.size || "";

          if (!qualityPreference.hasOwnProperty(size)) return;

          if (
            bestMatch.score === null ||
            qualityPreference[size] < bestMatch.score
          ) {
            bestMatch.score = qualityPreference[size];
            bestMatch.photoUrl = photoUrl;
          }
        },
      );

      if (photo.id) {
        this.bestUrls[photo.id] = bestMatch.photoUrl;
      }

      return bestMatch.photoUrl;
    },
    onPhotoLoaded(event: Event) {
      (event.target as ComponentPublicInstance<HTMLImageElement>).classList.add(
        "loaded",
      );
    },
  },
  computed: {
    isPending() {
      return this.photosStatus === "pending" || !this.hasRendered;
    },
    hasRendered() {
      return (
        this.photosStatus === "success" &&
        (!this.photos?.length || this.renderComplete)
      );
    },
    selectionMode() {
      return this.selectedPhotos.size > 0;
    },
  },
  watch: {
    selectedPhotos: {
      handler(newValue: Set<string>) {
        this.$emit("update:selectedPhotos", newValue);
      },
      deep: true,
    },
  },
});
</script>

<template>
  <article class="photo-grid" :class="{ skeleton: isPending }">
    <div
      v-if="isPending"
      v-for="n in 10"
      class="photo-preview skeleton-block"
    />
    <justified-infinite-grid
      v-if="photosStatus === 'success'"
      class="photo-grid"
      :class="{ invisible: !renderComplete, 'selection-mode': selectionMode }"
      :gap="5"
      :stretch="true"
      :pass-unstretch-row="true"
      :size-range="[228, 228]"
      :stretch-range="[144, 320]"
      :is-constant-size="true"
      @render-complete="renderComplete = true"
      @content-error="$event.update()"
    >
      <figure
        v-for="photo in photos"
        :key="photo.id"
        class="thumbnail min-w-[144px]"
        :class="{
          selected: selectedPhotos.has(photo.id!),
        }"
        :style="`aspect-ratio: ${getPhotoUrl(photo)?.dimensions?.width}/${getPhotoUrl(photo)?.dimensions?.height}`"
      >
        <label class="selection-ui">
          <span class="overlay" />
          <span class="action-bar">
            <input
              type="checkbox"
              class="selection-checkbox"
              :checked="selectedPhotos.has(photo.id!)"
              @change="
                selectedPhotos.has(photo.id!)
                  ? selectedPhotos.delete(photo.id!)
                  : selectedPhotos.add(photo.id!)
              "
            />
            <span class="selection-button">
              <font-awesome icon="check-circle" class="checkmark" />
              <font-awesome
                :icon="['far', 'circle-check']"
                class="checkmark-inverted"
              />
            </span>
          </span>
        </label>
        <nuxt-link :to="`/album/${albumId}/${photo.id}`" class="thumbnail-card">
          <img
            v-if="hasPhotoUrl(photo)"
            :src="getPhotoUrl(photo)?.url"
            loading="lazy"
            alt="Thumbnail image"
            class="thumbnail-image"
            @load="onPhotoLoaded"
          />
        </nuxt-link>
      </figure>
    </justified-infinite-grid>
  </article>
</template>

<style scoped lang="postcss">
.photo-preview-grid {
  @apply grid gap-[5px];
  grid-template-columns: repeat(auto-fill, minmax(250px, 1fr));

  .photo-preview {
    @apply aspect-[4/3] w-full;
  }
}

.photo-grid {
  .thumbnail {
    .thumbnail-card {
      @apply bg-gray-50 dark:bg-gray-950;

      &,
      .thumbnail-image {
        @apply block w-full h-full;
        @apply transition-all;
      }

      .thumbnail-image {
        @apply object-cover opacity-0 transition-opacity;

        &.loaded {
          @apply opacity-100;
        }
      }
    }

    &.selected .thumbnail-card {
      @apply p-2.5;

      .thumbnail-image {
        @apply rounded-xl;
      }
    }
  }

  .thumbnail {
    @apply relative;

    .selection-ui {
      .action-bar {
        @apply absolute;
        @apply w-full h-16;
        @apply opacity-0;
        @apply bg-gradient-to-b from-black/75;
        @apply pointer-events-none;
        @apply transition-opacity;

        .selection-checkbox {
          @apply sr-only;

          & + .selection-button {
            @apply m-2;
            @apply inline-flex text-white text-xl rounded-full;
            @apply pointer-events-auto cursor-pointer;
          }

          &:focus + .selection-button,
          &:focus-visible + .selection-button {
            @apply outline outline-4 outline-blue-300 dark:outline-blue-900;
          }
        }
      }

      .overlay {
        @apply absolute;
        @apply top-0 left-0 bottom-0 right-0;
      }

      .action-bar .selection-button .checkmark {
        @apply hidden;
      }

      .action-bar .selection-button:hover,
      .action-bar .selection-checkbox:focus + .selection-button,
      .overlay:hover + .action-bar .selection-button {
        .checkmark {
          @apply block;
        }

        .checkmark-inverted {
          @apply hidden;
        }
      }

      .action-bar .selection-checkbox:checked + .selection-button {
        .checkmark {
          @apply block text-blue-500 dark:text-blue-600;
        }

        .checkmark-inverted {
          @apply hidden;
        }
      }
    }
  }

  &.selection-mode .thumbnail,
  .thumbnail:hover {
    .selection-ui .action-bar {
      @apply opacity-100;
      @apply transition-none;
    }
  }

  .selection-ui .overlay {
    @apply pointer-events-none;
  }

  &.selection-mode .selection-ui .overlay {
    @apply pointer-events-auto cursor-pointer;
  }
}
</style>
