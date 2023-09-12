<template>
    <v-img
        class="mx-auto"
        width="80%"
        aspect-ratio="1.7778"
        cover
        lazy-src="@/assets/hoofd_outline_color.png"
        :src="coverPhotoUrl">
        <v-btn
            v-if="canSetThumbnail"
            style="position: absolute; right: 12%; top: 2%"
            color="primary"
            @click="$emit('select-cover')"
            title="Set as cover photo"
            fab
            small>
            <!-- Show a filled heart if the image is the cover, else show only an outline -->
            <v-icon>{{ isCover ?? false ? 'mdi-heart' : 'mdi-heart-outline'}}</v-icon>
        </v-btn>

        <v-btn
            v-if="canDelete"
            style="position: absolute; right: 2%; top: 2%"
            color="primary"
            @click="$emit('deleted')"
            title="Remove photo from album"
            fab
            small>
            <v-icon>mdi-trash-can-outline</v-icon>
        </v-btn>
    </v-img>
</template>

<script lang="ts">
import Vue from 'vue';
import {checkScope} from "@/api";

interface Data {
    canDelete: boolean,
    canSetThumbnail: boolean,
}

export default Vue.extend({
    props: {
        bytes: {
            type: Uint8Array,
            required: true,
        },
        isCover: {
            type: Boolean,
            required: false,
        }
    },
    data(): Data {
        return {
            canDelete: false,
            canSetThumbnail: false,
        }
    },
    async mounted() {
        this.canDelete = await checkScope("nl.svsticky.chroma.photo.delete") ?? false;
        this.canSetThumbnail = await checkScope("nl.svsticky.chroma.album.update") ?? false;
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