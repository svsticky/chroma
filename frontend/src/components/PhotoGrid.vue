<template>
    <v-card flat :loading="loading">
        <div v-if="photos.length > 0">
            <v-row
                v-for="(pair, idx) in chunkedPhotos"
                :key="idx">
                <v-col cols="12" sm="12" md="6">
                    <PhotoCover
                        :can-delete="edit && isAdmin"
                        :bytes="pair[0].photoBytes"
                        @deleted="deletePhoto(pair[0])"
                    ></PhotoCover>
                </v-col>
                <v-col v-if="pair.length === 2">
                    <PhotoCover
                        :can-delete="edit && isAdmin"
                        :bytes="pair[1].photoBytes"
                        @deleted="deletePhoto(pair[1])"
                    ></PhotoCover>
                </v-col>
            </v-row>
        </div>

        <div v-else>
            Album is empty..
        </div>
    </v-card>
</template>

<script lang="ts">
import Vue from 'vue';
import {deletePhoto, listPhotosInAlbum, PhotoModel} from "@/views/photo/photo";
import PhotoCover from "@/components/PhotoCover.vue";
import {errorText, Storage} from "@/api";

interface Data {
    snackbar: string | null,
    photos: PhotoModel[],
    loading: boolean,
}

export default Vue.extend({
    components: {PhotoCover},
    props: {
        albumId: String,
        update: {
            type: Number,
            required: false,
        },
        edit: {
            type: Boolean,
            required: false,
        }
    },
    data(): Data {
        return {
            snackbar: null,
            photos: [],
            loading: true,
        }
    },
    watch: {
        update() {
            this.loadPhotos();
        }
    },
    computed: {
        isAdmin: () => Storage.isAdmin(),
        chunkedPhotos(): PhotoModel[][] {
            const result = [];
            for(let i = 0; i < this.photos.length; i += 2) {
                result.push(this.photos.slice(i, i + 2))
            }

            return result
        }
    },
    async mounted() {
        await this.loadPhotos();
    },
    methods: {
        async loadPhotos() {
            this.loading = true;
            const result = await listPhotosInAlbum(this.albumId);
            this.loading = false;

            if(result == undefined) {
                this.snackbar = errorText;
                return;
            }

            this.photos = result;
        },
        async deletePhoto(photo: PhotoModel) {
            const result = await deletePhoto(photo.id);
            if(result) {
                await this.loadPhotos();
            } else {
                this.snackbar = errorText;
            }
        }
    }
})
</script>