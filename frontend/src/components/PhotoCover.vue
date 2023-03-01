<template>
    <v-img
        class="mx-auto"
        width="80%"
        aspect-ratio="1.7778"
        cover
        lazy-src="@/assets/hoofd_outline_kleur.svg"
        :src="coverPhotoUrl">
        <v-btn
            v-if="canDelete"
            style="position: absolute; right: 2%; top: 2%"
            color="primary"
            @click="$emit('deleted')"
            fab
            small>
            <v-icon>mdi-trash-can-outline</v-icon>
        </v-btn>
    </v-img>
</template>

<script lang="ts">
import Vue, {PropType} from 'vue';

export default Vue.extend({
    props: {
        bytes: {
            type: Uint8Array,
            required: true,
        },
        canDelete: {
            type: Boolean,
            required: false,
        },
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