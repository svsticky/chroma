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
                Album {{ album.name }}</v-card-title>
            <v-card-text>
                <PhotoGrid
                    :album-id="album.id"
                ></PhotoGrid>
            </v-card-text>
        </v-card>
    </v-container>
</template>

<script lang="ts">
import Vue from 'vue';
import {AlbumModel, getAlbum} from "@/views/album/album";
import {listPhotosInAlbum, PhotoModel} from "@/views/photo/photo";
import {errorText} from "@/api";
import PhotoGrid from "@/components/PhotoGrid.vue";
import ReturnButton from "@/components/ReturnButton.vue";

interface Data {
    snackbar: string | null,
    album: AlbumModel | null,
    loading: boolean
}

export default Vue.extend({
    components: {ReturnButton, PhotoGrid},
    data(): Data {
        return {
            snackbar: null,
            album: null,
            loading: true,
        }
    },
    async mounted() {
        await this.loadAlbum();
    },
    methods: {
        getIdInner(): string | null {
            const paramsRaw = window.location.hash.split('?');
            if(paramsRaw.length != 2) {
                return null;
            }

            const params = new URLSearchParams(paramsRaw[1]);
            return params.get('id');
        },
        async getId(): Promise<string> {
            const id = this.getIdInner();
            if(id == null) {
                await this.$router.back();
            }

            return id!;
        },
        async loadAlbum() {
            this.loading = true;
            const id = await this.getId();
            const result = await getAlbum(id);
            this.loading = false;

            if(result == undefined) {
                await this.$router.back();
                return;
            }

            this.album = result;
        },
    }
})
</script>