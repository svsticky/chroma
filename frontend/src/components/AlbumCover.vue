<template>
    <v-card elevation="pa-3">
        <v-card-title>
            {{ album.name }} {{ album.isDraft ? "(DRAFT)" : null }}
            <v-spacer></v-spacer>
            <v-btn
                v-if="canDeleteAlbum"
                class="mr-1"
                color="primary"
                fab
                small
                @click="deleteAlbum">
                <v-icon>mdi-trash-can-outline</v-icon>
            </v-btn>
            <v-btn
                v-if="canEditAlbum"
                color="primary"
                fab
                small
                :to="`/album/edit?id=${album.id}`">
                <v-icon>mdi-pencil-outline</v-icon>
            </v-btn>
        </v-card-title>
        <v-card-text @click="openAlbum">
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
                width="80%"
                aspect-ratio="1.7778"
                cover
                lazy-src="@/assets/hoofd_outline_color.png"
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
import {AlbumModel, deleteAlbum} from "@/views/album/album";
import {checkScope, errorText, Storage} from "@/api";
import {getPhoto} from "@/views/photo/photo";

interface Data {
    snackbar: string | null,
    coverPhotoBytes: Uint8Array | null,
    loading: boolean,
    canDeleteAlbum: boolean,
    canEditAlbum: boolean,
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
            snackbar: null,
            coverPhotoBytes: null,
            loading: true,
            canDeleteAlbum: false,
            canEditAlbum: false,
        }
    },
    mounted() {
        this.loadCoverPhoto();

        this.loadPermissions();
    },
    computed: {
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
        async loadPermissions() {
            if(Storage.isAdmin()) {
                this.canDeleteAlbum = true;
                this.canEditAlbum = true;
            } else {
                this.canEditAlbum = await checkScope("nl.svsticky.chroma.album.update") ?? false;
                this.canDeleteAlbum = await checkScope("nl.svsticky.chroma.album.delete") ?? false;
            }
        },
        async loadCoverPhoto() {
            if(this.album.coverPhotoId == null) {
                return;
            }

            this.loading = true;
            const result = await getPhoto(this.album.coverPhotoId, true);
            this.loading = false;
            if(result == undefined) {
                this.snackbar = errorText;
                return;
            }

            this.coverPhotoBytes = result.photoBytes;
        },
        async deleteAlbum() {
            const result = await deleteAlbum(this.album.id);
            if(result) {
                this.requestUpdate();
            } else {
                this.snackbar = errorText;
            }
        },
        requestUpdate() {
            this.$emit('change');
        },
        openAlbum() {
            this.$router.push(`/album/view?id=${this.album.id}`)
        }
    }
})
</script>

<style scoped>
.v-card:hover {
    cursor: pointer;
}
</style>