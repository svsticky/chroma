<script lang="ts">
import type { Photo } from "~/proto/entity/photo";

export default defineComponent({
  setup() {
    definePageMeta({
      key: (route) => route.params["album"] as string,
    });

    const route = useRoute();
    const albumId = ref(route.params["album"] as string);
    const photoId = ref(route.params["photo"] as string | undefined);
    const hasPhotoId = !!photoId;

    return { photoId, albumId, hasPhotoId };
  },
  data() {
    return {
      photos: null as ReturnType<Photo["toObject"]>[] | null,
    };
  },
  watch: {
    $route(to) {
      this.albumId = to.params["album"] as string;
      this.photoId = (to.params["photo"] as string | undefined) || undefined;
    },
  },
});
</script>

<template>
  <album-page
    :key="albumId"
    :album-id="albumId"
    :photo-id="photoId"
    @update:photos="photos = $event"
  />

  <transition name="photo-page-ec18923c">
    <keep-alive>
      <photo-page
        key="photo-page"
        v-if="photoId"
        v-bind="photos && { photos }"
        :album-id="albumId"
        :photo-id="photoId"
      />
    </keep-alive>
  </transition>
</template>

<style scoped lang="postcss">
.photo-page-ec18923c-enter-active,
.photo-page-ec18923c-leave-active {
  @apply transition-opacity;
}

.photo-page-ec18923c-enter-from,
.photo-page-ec18923c-leave-to {
  @apply opacity-0;
}
</style>
