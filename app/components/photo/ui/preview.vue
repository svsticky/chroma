<script lang="ts">
import type { AsyncDataRequestStatus, NuxtError } from "#app";
import type { Photo, PhotoUrl } from "~/proto/entity/photo";

type QualityPreference = {
  [key: string]: number;
};

const qualityPreference: QualityPreference = {
  Preview: 0,
  Original: 1,
  Thumbnail: 2,
};

export default defineComponent({
  props: {
    photoId: { type: String, required: true },
    photos: Object as PropType<ReturnType<Photo["toObject"]>[]>,
  },
  emits: ["update:status", "update:error"],
  setup({ photoId, photos }) {
    const models = useModels();

    if (photos) {
      const photo = photos.find((photo) => photo.id === photoId);

      if (photo) {
        const photoStatus: AsyncDataRequestStatus = "success";
        const photoError: NuxtError | null = null;

        return { photo, photoStatus, photoError };
      }
    }

    const {
      data: photo,
      status: photoStatus,
      error: photoError,
    } = useAsyncData(photoId, () => models.photo.get(photoId));

    return { photo, photoStatus, photoError };
  },
  data() {
    return {
      cached: {
        photoSrc: null as ReturnType<PhotoUrl["toObject"]> | null,
      },
    };
  },
  computed: {
    photoSrc() {
      const bestMatch = {
        score: null as number | null,
        photoSrc: null as ReturnType<PhotoUrl["toObject"]> | null,
      };

      if (this.cached.photoSrc) {
        return this.cached.photoSrc;
      }

      this.photo?.media?.urls?.forEach(
        (photoSrc: ReturnType<PhotoUrl["toObject"]>) => {
          const size = photoSrc.size || "";

          if (!qualityPreference.hasOwnProperty(size)) return;

          if (
            bestMatch.score === null ||
            qualityPreference[size] < bestMatch.score
          ) {
            bestMatch.score = qualityPreference[size];
            bestMatch.photoSrc = photoSrc;
          }
        },
      );

      if (bestMatch.photoSrc) {
        this.cached.photoSrc = bestMatch.photoSrc;
      }

      return bestMatch.photoSrc;
    },
  },
  watch: {
    photoStatus: {
      handler(newValue) {
        this.$emit("update:status", newValue);
      },
      immediate: true,
    },
    photoError: {
      handler(newValue) {
        this.$emit("update:error", newValue);
      },
      immediate: true,
    },
  },
});
</script>

<template>
  <img
    :src="photoSrc?.url"
    :style="{
      aspectRatio: `${photoSrc?.dimensions?.width}/${photoSrc?.dimensions?.height}`,
    }"
    alt="Image preview"
  />
</template>

<style scoped lang="postcss"></style>
