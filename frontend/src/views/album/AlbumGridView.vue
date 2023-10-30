<template>
    <v-container>
      <v-card elevation="2" class="mt-3 pa-3" :loading="loading">
        <v-card-title>
          Albums
          <v-spacer></v-spacer>
          <v-btn v-if="canCreateAlbum" color="primary" fab small title="Create" to="/album/create">
            <v-icon>mdi-plus</v-icon>
          </v-btn>
        </v-card-title>
        <v-card-text>
          <div v-if="albums.length > 0">
            <v-row v-for="(pair, idx) in chunkedAlbums" :key="idx">
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
            There are no albums yet...
          </div>
        </v-card-text>
      </v-card>
    </v-container>
  </template>
  
  <script lang="ts">
  import { defineComponent, ref, computed, onMounted } from 'vue';
  import { AlbumModel, listAlbums } from "@/views/album/album";
  import { checkScope, errorText, Storage } from "@/api";
  import AlbumCover from "@/components/AlbumCover.vue";
  
  export default defineComponent({
    components: { AlbumCover },
    setup() {
      const snackbar = ref<string | null>(null);
      const loading = ref<boolean>(false);
      const canCreateAlbum = ref<boolean>(false);
      const albums = ref<AlbumModel[]>([]);
      const canEditAlbums = ref<boolean>(false);
      const canDeleteAlbums = ref<boolean>(false);
  
      const chunkedAlbums = computed(() => {
        const result = [];
        for (let i = 0; i < albums.value.length; i += 2) {
          result.push(albums.value.slice(i, i + 2))
        }
        return result;
      });
  
      const loadCanCreateAlbum = async () => {
        if (Storage.isAdmin()) {
          canCreateAlbum.value = true;
        } else {
          const hasScope = await checkScope("nl.svsticky.chroma.album.create");
          if (hasScope === undefined) {
            return;
          }
          canCreateAlbum.value = hasScope;
        }
      };
  
      const loadAlbums = async () => {
        loading.value = true;
        const fetchedAlbums = await listAlbums();
        loading.value = false;
  
        if (fetchedAlbums === undefined) {
          snackbar.value = errorText;
          return;
        }
  
        albums.value = fetchedAlbums;
      };
  
      onMounted(async () => {
        await loadAlbums();
        await loadCanCreateAlbum();
  
        if (Storage.isAdmin()) {
          canEditAlbums.value = true;
          canDeleteAlbums.value = true;
        } else {
          canEditAlbums.value = await checkScope("nl.svsticky.chroma.album.update") ?? false;
          canDeleteAlbums.value = await checkScope("nl.svsticky.chroma.album.delete") ?? false;
        }
      });
  
      return {
        snackbar,
        loading,
        canCreateAlbum,
        albums,
        canEditAlbums,
        canDeleteAlbums,
        chunkedAlbums,
        loadCanCreateAlbum,
        loadAlbums
      };
    }
  });
  </script>
  