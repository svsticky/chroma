<template>
    <v-img
        v-else
        class="mx-auto"
        width="30%"
        aspect-ratio="1"
        cover
        lazy-src="@/assets/hoofd_outline_kleur.svg"
        :src="coverPhotoUrl"
    ></v-img>
</template>

<script lang="ts">
import Vue, {PropType} from 'vue';

export default Vue.extend({
    props: {
        bytes: {
            type: Object as PropType<Uint8Array>,
            required: true,
        }
    },
    computed: {
        coverPhotoUrl(): string {
            return 'data:image/png;base64,' + btoa(
                this.bytes.reduce((data, byte) => data + String.fromCharCode(byte), '')
            );
        }
    }
})
</script>