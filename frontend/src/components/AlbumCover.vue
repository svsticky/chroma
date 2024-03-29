<template>
    <v-card elevation="pa-3">
        <v-card-title>
            {{ album.name }} {{ album.isDraft ? "(DRAFT)" : null }}
            <v-spacer></v-spacer>
            <v-btn
                v-if="canDelete"
                class="mr-1"
                color="primary"
                fab
                small
                @click="deleteAlbum">
                <v-icon>mdi-trash-can-outline</v-icon>
            </v-btn>
            <v-btn
                v-if="canEdit"
                color="primary"
                fab
                small
                :to="`/album/edit?id=${album.id}`">
                <v-icon>mdi-pencil-outline</v-icon>
            </v-btn>
        </v-card-title>
        <v-card-text @click="openAlbum">
            <v-img
                v-if="album.coverPhoto == null"
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
                :src="album.coverPhoto.getAsSrcUrl()">

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
import {errorText} from "@/api";
import {getPhoto, PhotoModel} from "@/views/photo/photo";

interface Data {
    snackbar: string | null,
    coverPhoto: PhotoModel | null,
    loading: boolean,
}

export default Vue.extend({
    props: {
        album: {
            type: Object as PropType<AlbumModel>,
            required: true,
        },
        canEdit: Boolean,
        canDelete: Boolean,
    },
    data(): Data {
        return {
            snackbar: null,
            coverPhoto: null,
            loading: true,
        }
    },
    computed: {
        coverPhotoUrl(): string | null {
            if(this.loading || this.coverPhoto == null) {
                return null;
            }

            return this.coverPhoto.getAsSrcUrl();
        }
    },
    methods: {
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