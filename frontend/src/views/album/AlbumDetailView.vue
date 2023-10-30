<template>
    <v-container>
        <v-card v-if="loading" :loading="true">
            <v-card-title>
                Loading...
            </v-card-title>
        </v-card>
        <v-card v-else>
            <v-card-title>
                <ReturnButton></ReturnButton>
                Album {{ album?.name }}</v-card-title>
            <v-card-text>
                <PhotoGrid
                    :album-id="album?.id"
                ></PhotoGrid>
            </v-card-text>
        </v-card>
    </v-container>
</template>

<script lang="ts">
import { defineComponent, ref } from 'vue';
import { useRouter } from 'vue-router';
import { AlbumModel, getAlbum } from "@/views/album/album";
import PhotoGrid from "@/components/PhotoGrid.vue";
import ReturnButton from "@/components/ReturnButton.vue";

export default defineComponent({
    name: 'AlbumComponent',
    components: { ReturnButton, PhotoGrid },
    setup() {
        const router = useRouter();
        const snackbar = ref<string | null>(null);
        const album = ref<AlbumModel | null>(null);
        const loading = ref<boolean>(true);

        const getIdInner = (): string | null => {
            const paramsRaw = window.location.hash.split('?');
            if(paramsRaw.length != 2) {
                return null;
            }

            const params = new URLSearchParams(paramsRaw[1]);
            return params.get('id');
        };

        const getId = async (): Promise<string> => {
            const id = getIdInner();
            if(id == null) {
                await router.back();
            }

            return id!;
        };

        const loadAlbum = async () => {
            loading.value = true;
            const id = await getId();
            const result = await getAlbum(id, true);
            loading.value = false;

            if(result == undefined) {
                await router.back();
                return;
            }

            album.value = result;
        };

        loadAlbum();

        return {
            snackbar,
            album,
            loading,
            getIdInner,
            getId,
            loadAlbum
        };
    }
});
</script>
