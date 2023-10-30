<template>
    <v-card flat :loading="loading">
        <div v-if="photos.length > 0">
            <v-row
                v-for="(pair, idx) in chunkedPhotos"
                :key="idx">
                <v-col cols="12" sm="12" md="6">
                    <PhotoCover
                        :can-delete="edit && canDeletePhoto"
                        :can-set-thumbnail="edit && canEdit"
                        :is-cover="albumModel?.coverPhotoId === pair[0].id"
                        :photo="pair[0]"
                        @select-cover="selectCover(pair[0])"
                        @deleted="deletePhotoFn(pair[0])"
                    ></PhotoCover>
                </v-col>
                <v-col v-if="pair.length === 2">
                    <PhotoCover
                        :can-delete="edit && canDeletePhoto"
                        :can-set-thumbnail="edit && canEdit"
                        :is-cover="albumModel?.coverPhotoId === pair[1].id"
                        :photo="pair[1]"
                        @select-cover="selectCover(pair[1])"
                        @deleted="deletePhotoFn(pair[1])"
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
import { defineComponent, ref, watch, onMounted, computed } from 'vue';
import { deletePhoto, listPhotosInAlbum, PhotoModel } from "@/views/photo/photo";
import PhotoCover from "@/components/PhotoCover.vue";
import { checkScope, errorText, Storage } from "@/api";
import { AlbumModel, getAlbum, saveEditedAlbum } from "@/views/album/album";

export default defineComponent({
    components: { PhotoCover },
    props: {
        albumId: String,
        update: Number,
        edit: Boolean
    },
    setup(props) {
        const snackbar = ref<string | null>(null);
        const photos = ref<PhotoModel[]>([]);
        const loading = ref<boolean>(true);
        const albumModel = ref<AlbumModel | null>(null);
        const canDeletePhoto = ref<boolean>(false);
        const canEdit = ref<boolean>(false);

        const chunkedPhotos = computed(() => {
            const result: PhotoModel[][] = [];
            for(let i = 0; i < photos.value.length; i += 2) {
                result.push(photos.value.slice(i, i + 2));
            }
            return result;
        });

        const loadPhotos = async () => {
            loading.value = true;
            if (!props.albumId) {
                snackbar.value = errorText;
                return;
            }
            const result = await listPhotosInAlbum(props.albumId, true);
            loading.value = false;

            if(!result) {
                snackbar.value = errorText;
                return;
            }

            photos.value = result;
        };

        const loadCoverData = async () => {
            if (!props.albumId) {
                snackbar.value = errorText;
                return;
            }
            const result = await getAlbum(props.albumId, true);
            
            if(!result) {
                snackbar.value = errorText;
                return;
            }

            albumModel.value = result;
        };

        const deletePhotoFn = async (photo: PhotoModel) => {
            const result = await deletePhoto(photo.id);
            if(result) {
                await loadPhotos();
            } else {
                snackbar.value = errorText;
            }
        };

        const selectCover = async (photo: PhotoModel) => {
            if (albumModel.value) {
                albumModel.value.coverPhotoId = photo.id;
                const result = await saveEditedAlbum(albumModel.value);

                snackbar.value = result ? "Cover updated" : errorText;
            }
        };

        watch(() => props.update, async () => {
            await loadPhotos();
        });

        onMounted(async () => {
            await loadPhotos();
            await loadCoverData();

            if(Storage.isAdmin()) {
                canDeletePhoto.value = true;
                canEdit.value = true;
            } else if(props.edit) {
                canDeletePhoto.value = await checkScope("nl.svsticky.chroma.photo.delete") ?? false;
                canEdit.value = await checkScope("nl.svsticky.chroma.album.update") ?? false;
            }
        });

        return {
            snackbar,
            photos,
            loading,
            albumModel,
            canDeletePhoto,
            canEdit,
            chunkedPhotos,
            loadPhotos,
            loadCoverData,
            deletePhotoFn,
            selectCover
        };
    }
})
</script>