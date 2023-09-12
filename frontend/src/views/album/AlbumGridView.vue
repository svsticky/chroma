<template>
    <v-container>
        <v-card
            elevation="2"
            class="mt-3 pa-3"
            :loading="loading">
            <v-card-title>
                Albums
                <v-spacer></v-spacer>
                <v-btn
                    v-if="isAdmin"
                    color="primary"
                    fab
                    small
                    title="Create"
                    to="/album/create">
                    <v-icon>mdi-plus</v-icon>
                </v-btn>
            </v-card-title>
            <v-card-text>
                <div v-if="albums.length > 0">
                    <v-row
                        v-for="(pair, idx) in chunkedAlbums"
                        :key="idx">
                        <v-col cols="12" sm="12" md="6">
                            <AlbumCover
                                @change="loadAlbums"
                                :album="pair[0]"
                            ></AlbumCover>
                        </v-col>
                        <v-col v-if="pair.length === 2">
                            <AlbumCover
                                @change="loadAlbums"
                                :album="pair[1]"
                            ></AlbumCover>
                        </v-col>
                    </v-row>
                </div>

                <div v-else>
                    There are no albums yet..
                </div>
            </v-card-text>
        </v-card>
    </v-container>
</template>

<script lang="ts">
import Vue from 'vue';
import {AlbumModel, listAlbums} from "@/views/album/album";
import {errorText, Storage} from "@/api";
import AlbumCover from "@/components/AlbumCover.vue";

interface Data {
    snackbar: string | null,
    loading: boolean,

    albums: AlbumModel[]
}

export default Vue.extend({
    components: {AlbumCover},
    data(): Data {
        return {
            snackbar: null,
            loading: false,
            albums: [],
        }
    },
    computed: {
        isAdmin: () => Storage.isAdmin(),
        chunkedAlbums(): AlbumModel[][] {
            const result = [];
            for(let i = 0; i < this.albums.length; i += 2) {
                result.push(this.albums.slice(i, i + 2))
            }

            return result
        }
    },
    async mounted() {
        await this.loadAlbums();
    },
    methods: {
        async loadAlbums() {
            this.loading = true;
            const albums = await listAlbums();
            this.loading = false;

            if(albums == undefined) {
                this.snackbar = errorText;
                return;
            }

            this.albums = albums;
        }
    }
})
</script>