<template>
    <v-card elevation="pa-3">
        <v-card-title>
            {{ album.name }}
            <v-spacer></v-spacer>
            <v-btn
                v-if="isAdmin"
                color="primary"
                fab
                small
                :to="`/album/edit?id=${album.id}`">
                <v-icon>mdi-pencil-outline</v-icon>
            </v-btn>
        </v-card-title>
        <v-card-text>
            <v-img
                v-if="album.coverPhotoId == null"
                class="mx-auto"
                title="No cover image is available"
                width="30%"
                aspect-ratio="1"
                src="@/assets/camera-off-outline.svg">
            </v-img>
            <v-img
                v-else
                class="mx-auto"
                width="30%"
                aspect-ratio="1"
                cover
                lazy-src="@/assets/hoofd_outline_kleur.svg"
                :src="coverPhotoUrl">

                <template v-slot:placeholder>
                    <div class="d-flex align-center justify-center fill-height">
                        <v-progress-circular
                            color="grey-lighten-4"
                            indeterminate
                        ></v-progress-circular>
                    </div>
                </template>
            </v-img>
        </v-card-text>
    </v-card>
</template>

<script lang="ts">
import Vue, {PropType} from 'vue';
import {AlbumModel} from "@/views/album/album";
import {Storage} from "@/api";

interface Data {
    coverPhotoBytes: Uint8Array | null,
    loading: boolean,
}

export default Vue.extend({
    props: {
        album: {
            type: Object as PropType<AlbumModel>,
            required: true,
        }
    },
    data(): Data {
        return {
            coverPhotoBytes: null,
            loading: true,
        }
    },
    mounted() {
        this.loadCoverPhoto();
    },
    computed: {
        isAdmin: () => Storage.isAdmin(),
        coverPhotoUrl(): string | null {
            if(this.loading || this.coverPhotoBytes == null) {
                return null;
            }

            return 'data:image/png;base64,' + btoa(
                this.coverPhotoBytes.reduce((data, byte) => data + String.fromCharCode(byte), '')
            );
        }
    },
    methods: {
        async loadCoverPhoto() {

        }
    }
})
</script>

<style scoped>
.v-card:hover {
    cursor: pointer;
}
</style>