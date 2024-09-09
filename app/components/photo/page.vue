<script lang="ts">
import type { Photo, PhotoUrl } from "~/proto/entity/photo";
import type { AsyncDataRequestStatus, NuxtError } from "#app";

export default defineComponent({
  props: {
    albumId: String,
    photoId: { type: String, required: true },
    photos: Object as PropType<ReturnType<Photo["toObject"]>[]>,
  },
  mounted() {
    this.$el.addEventListener("mousemove", () => {
      this.ui.show = true;

      if (this.timer) {
        clearTimeout(this.timer);
      }

      this.timer = setTimeout(() => {
        if (!this.ui.hovered) {
          this.ui.show = false;
        }
      }, 3000);
    });
  },
  data() {
    return {
      timer: null as ReturnType<typeof setTimeout> | null,
      ui: {
        show: true,
        hovered: false,
      },
      photo: {
        status: "idle" as AsyncDataRequestStatus,
        error: null as NuxtError | null,
      },
      cached: {
        photoIndex: null as number | null,
      },
    };
  },
  computed: {
    nextPhotoId() {
      if (!this.photos) {
        return null;
      }

      if (!this.cached.photoIndex) {
        this.cached.photoIndex = this.photos.findIndex(
          (photo) => photo.id === this.photoId,
        );
      }

      return this.cached.photoIndex < this.photos.length - 1
        ? this.photos.at(this.cached.photoIndex + 1)?.id || null
        : null;
    },
    previousPhotoId() {
      if (!this.photos) {
        return null;
      }

      if (!this.cached.photoIndex) {
        this.cached.photoIndex = this.photos.findIndex(
          (photo) => photo.id === this.photoId,
        );
      }

      return this.cached.photoIndex > 0
        ? this.photos.at(this.cached.photoIndex - 1)?.id || null
        : null;
    },
  },
  watch: {
    photoId() {
      this.cached.photoIndex = null;
    },
  },
});
</script>

<template>
  <article class="lightbox" :class="{ 'photo-error': photo.error }">
    <template v-if="photo.status === 'pending'"></template>
    <template v-else-if="!photo.error">
      <transition name="show-ui-8fd1b1b0">
        <header
          v-show="ui.show"
          @mouseenter="ui.hovered = true"
          @mouseleave="ui.hovered = false"
        >
          <nuxt-link
            :to="albumId ? `/album/${albumId}` : '/'"
            class="back-button"
          >
            <font-awesome icon="arrow-left" />
          </nuxt-link>
        </header>
      </transition>
      <main class="photo-preview">
        <transition name="show-ui-8fd1b1b0">
          <nuxt-link
            :to="
              albumId
                ? `/album/${albumId}/${previousPhotoId}`
                : `/photo/${previousPhotoId}`
            "
            v-if="previousPhotoId"
            v-show="ui.show"
            class="navigation-button left"
            replace
          >
            <div
              @mouseenter="ui.hovered = true"
              @mouseleave="ui.hovered = false"
              class="navigation-icon"
            >
              <font-awesome icon="chevron-left" />
            </div>
          </nuxt-link>
        </transition>
        <keep-alive>
          <photo-ui-preview
            :key="photoId"
            :photo-id="photoId"
            :photos="photos"
            @update:status="photo.status = $event"
            @update:error="photo.error = $event"
          />
        </keep-alive>
        <transition name="show-ui-8fd1b1b0">
          <nuxt-link
            :to="
              albumId
                ? `/album/${albumId}/${nextPhotoId}`
                : `/photo/${nextPhotoId}`
            "
            v-if="nextPhotoId"
            v-show="ui.show"
            class="navigation-button right"
            replace
          >
            <div
              @mouseenter="ui.hovered = true"
              @mouseleave="ui.hovered = false"
              class="navigation-icon"
            >
              <font-awesome icon="chevron-right" />
            </div>
          </nuxt-link>
        </transition>
      </main>
    </template>
    <template v-else>
      <div class="text-white">Photo not found!</div>
    </template>
  </article>
</template>

<style scoped lang="postcss">
.lightbox {
  @apply fixed top-0 left-0 bottom-0 right-0 z-50;
  @apply bg-black;

  header {
    @apply absolute top-0 left-0 right-0 z-10;
    @apply p-2.5;
    @apply flex;
    @apply bg-gradient-to-b from-black/50;

    .back-button {
      @apply w-10 h-10 flex justify-center items-center;
      @apply text-xl text-white;
      @apply rounded-full;
      @apply transition-[background];

      &:hover,
      &:focus {
        @apply bg-white/15;
        @apply transition-none;
      }
    }
  }

  main.photo-preview {
    @apply flex;
    @apply w-full h-full;

    img {
      @apply m-auto max-w-full max-h-full;
    }

    .navigation-button {
      @apply absolute;
      @apply top-0 bottom-0 w-1/3;
      @apply flex items-center;
      @apply text-white;
      @apply opacity-0;

      &.left {
        @apply left-2.5 justify-start;
      }

      &.right {
        @apply right-2.5 justify-end;
      }

      &:hover {
        @apply opacity-100;
      }

      &:focus {
        @apply outline-none;

        & > .navigation-icon {
          @apply outline outline-4 outline-blue-300 dark:outline-blue-900;
        }
      }

      & > .navigation-icon {
        @apply w-10 h-10 flex justify-center items-center;
        @apply text-xl text-white;
        @apply rounded-full;
        @apply bg-black/15;
        @apply transition-[background];
      }
    }
  }
}

.show-ui-8fd1b1b0-leave-active {
  @apply transition-opacity duration-500;
}

.show-ui-8fd1b1b0-enter-from,
.show-ui-8fd1b1b0-leave-to {
  @apply opacity-0;
}
</style>
