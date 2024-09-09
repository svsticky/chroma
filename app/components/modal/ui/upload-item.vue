<script lang="ts">
export type Upload = {
  file: File;
  status: string;
  progress: number;
};

export default defineComponent({
  props: {
    upload: { type: Object as PropType<Upload>, required: true },
  },
  data() {
    return {
      observer: null as IntersectionObserver | null,
    };
  },
  mounted() {
    this.observer = new IntersectionObserver(
      (entries) => {
        entries.forEach((entry) => {
          const target = this.$refs.image as HTMLImageElement;

          if (entry.isIntersecting) {
            target.src = this.createObjectUrl(this.upload.file);
            target.onload = () => {
              this.destroyObjectUrl(target.src);
            };
          } else {
            target.src = "";
          }
        });
      },
      {
        rootMargin: "25px 0px 25px 0px",
      },
    );

    this.observer.observe(this.$refs.observed as HTMLElement);
  },
  beforeUnmount() {
    this.observer?.disconnect();
  },
  methods: {
    createObjectUrl(file: File) {
      return URL.createObjectURL(file);
    },
    destroyObjectUrl(url: string) {
      URL.revokeObjectURL(url);
    },
  },
});
</script>

<template>
  <li ref="observed">
    <client-only>
      <img
        ref="image"
        :alt="upload.file.name"
        class="upload-preview"
        loading="lazy"
      />
    </client-only>
    {{ upload.file.name }} ({{ upload.status }}:
    {{ upload.progress.toFixed(2) }}%)
  </li>
</template>

<style scoped lang="postcss">
.upload-preview {
  @apply w-12 h-12;
  @apply object-cover;
  @apply rounded-sm;
}
</style>
