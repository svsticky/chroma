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
                    v-if="canCreateAlbum"
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
                                :can-delete="canDeleteAlbums"
                                :can-edit="canEditAlbums"
                                @change="loadAlbums"
                                :album="pair[0]"
                            ></AlbumCover>
                        </v-col>
                        <v-col v-if="pair.length === 2">
                            <AlbumCover
                                :can-delete="canDeleteAlbums"
                                :can-edit="canEditAlbums"
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
import {checkScope, errorText, Storage} from "@/api";
import AlbumCover from "@/components/AlbumCover.vue";

interface Data {
    snackbar: string | null,
    loading: boolean,
    canCreateAlbum: boolean,
    albums: AlbumModel[]
    canEditAlbums: boolean,
    canDeleteAlbums: boolean,
}

export default Vue.extend({
    components: {AlbumCover},
    data(): Data {
        return {
            snackbar: null,
            loading: false,
            canCreateAlbum: false,
            albums: [],
            canEditAlbums: false,
            canDeleteAlbums: false,
        }
    },
    computed: {
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
        await this.loadCanCreateAlbum();

        if(Storage.isAdmin()) {
            this.canEditAlbums = true;
            this.canDeleteAlbums = true;
        } else {
            this.canEditAlbums = await checkScope("nl.svsticky.chroma.album.update") ?? false;
            this.canDeleteAlbums = await checkScope("nl.svsticky.chroma.album.delete") ?? false;
        }
    },
    methods: {
        async loadCanCreateAlbum() {
            if(Storage.isAdmin()) {
               this.canCreateAlbum = true;
            } else {
                const hasScope = await checkScope("nl.svsticky.chroma.album.create");
                if(hasScope == undefined) {
                    return;
                }

                this.canCreateAlbum = hasScope;
            }
        },
        async loadAlbums() {
            this.loading = true;
            const albums = await listAlbums();
            this.loading = false;

            if(albums == undefined) {
                this.snackbar = errorText;
                return;
            }

            albums.sort((a, b) => {
                if(a.publishedAt != null && b.publishedAt != null) {
                    return a.publishedAt - b.publishedAt;
                } else {
                    return a.createdAt - b.createdAt;
                }
            });

            this.albums = albums;
        }
    }
})
</script>