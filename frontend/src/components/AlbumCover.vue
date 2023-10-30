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
                @click="DeleteAlbum">
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
import { defineComponent, ref, computed, PropType } from 'vue';
import { AlbumModel, deleteAlbum } from "@/views/album/album";
import { errorText } from "@/api";
import { getPhoto, PhotoModel } from "@/views/photo/photo";
import { useRouter } from 'vue-router';

export default defineComponent({
    props: {
        album: {
            type: Object as PropType<AlbumModel>,
            required: true,
        },
        canEdit: Boolean,
        canDelete: Boolean,
    },
    setup(props, context) {
        const snackbar = ref<string | null>(null);
        const coverPhoto = ref<PhotoModel | null>(null);
        const loading = ref<boolean>(true);
        const router = useRouter();


        const coverPhotoUrl = computed(() => {
            if (loading.value || coverPhoto.value == null) {
                return null;
            }
            return coverPhoto.value.getAsSrcUrl();
        });

        const DeleteAlbum = async () => {
            const result = await deleteAlbum(props.album.id);
            if (result) {
                requestUpdate();
            } else {
                snackbar.value = errorText;
            }
        };

        const requestUpdate = () => {
            context.emit("update");
        };

        const openAlbum = () => {
            router.push(`/album/view?id=${props.album.id}`);
        };

        return {
            snackbar,
            coverPhoto,
            loading,
            coverPhotoUrl,
            DeleteAlbum,
            requestUpdate,
            openAlbum
        };
    }
})
</script>

<style scoped>
.v-card:hover {
    cursor: pointer;
}
</style>
