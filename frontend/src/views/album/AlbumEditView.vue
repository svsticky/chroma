<template>
    <v-container>
      <UploadPhotoDialog
        :enabled="dialog.uploadPhoto"
        :album-id="getIdInner()"
        @close="handleUploadDialogClosed"
      ></UploadPhotoDialog>
  
      <v-card elevation="2" class="mt-3 pa-3" :loading="loading.get">
        <v-card-title v-if="album != null">
          <ReturnButton></ReturnButton>
          Edit {{ album.isDraft ? "draft" : null }} album
          <v-spacer></v-spacer>
  
          <div v-if="isAdmin">
            <v-btn
              v-if="album.isDraft"
              :loading="loading.changeDraftStatus"
              @click="setDraftStatus(false)"
              color="primary"
            >
              Publish album
            </v-btn>
            <v-btn
              v-else
              :loading="loading.changeDraftStatus"
              @click="setDraftStatus(true)"
              color="primary"
            >
              Unpublish album
            </v-btn>
          </div>
        </v-card-title>
        <v-card-text v-if="album != null">
          <v-form v-model="valid">
            <v-text-field
              v-model="album.name"
              counter="64"
              label="Name"
              :rules="rules.name"
            ></v-text-field>
          </v-form>
          <v-btn
            color="primary"
            :disabled="!valid"
            :loading="loading.save"
            @click="save"
          >
            Save
          </v-btn>
  
          <!-- Without the bottom margin, the plus button has shadow issues on the bottom -->
          <div class="d-flex flex-row mt-3 mb-3">
            <div class="text-h5"> Photos </div>
            <v-spacer></v-spacer>
            <v-btn
              v-if="canCreatePhotos"
              color="primary"
              fab
              small
              title="Add photo"
              @click="dialog.uploadPhoto = true"
            >
              <v-icon>mdi-plus</v-icon>
            </v-btn>
          </div>
          <PhotoGrid
            :update="photoGridUpdater"
            :album-id="getIdInner()"
            :edit="true"
          ></PhotoGrid>
        </v-card-text>
      </v-card>
    </v-container>
  </template>
  
  <script lang="ts">
  import { defineComponent, ref, computed, onMounted } from 'vue';
  import { useRouter } from 'vue-router';
  import { AlbumModel, getAlbum, saveEditedAlbum, setAlbumDraftStatus } from "@/views/album/album";
  import { checkScope, errorText, Storage } from "@/api";
  import ReturnButton from "@/components/ReturnButton.vue";
  import PhotoGrid from "@/components/PhotoGrid.vue";
  import UploadPhotoDialog from "@/components/UploadPhotoDialog.vue";
  
  export default defineComponent({
    components: { UploadPhotoDialog, PhotoGrid, ReturnButton },
    setup() {
      const snackbar = ref<string | null>(null);
      const loading = ref<{
        get: boolean;
        save: boolean;
        changeDraftStatus: boolean;
      }>({
        get: true,
        save: false,
        changeDraftStatus: false,
      });
      const router = useRouter();
      const rules = ref({
        name: [
          (v :string) => !!v || "This field is required",
          (v :string) => v.length > 1 || "This field is required",
        ],
      });
      const album = ref<AlbumModel | null>(null);
      const photoGridUpdater = ref<number>(0);
      const valid = ref<boolean>(true);
      const dialog = ref<{ uploadPhoto: boolean }>({
        uploadPhoto: false,
      });
      const canCreatePhotos = ref<boolean>(false);
  
  
      const isAdmin = computed(() => Storage.isAdmin());
  
      const getIdInner = (): string | undefined => {
        const paramsRaw = window.location.hash.split('?');
        if (paramsRaw.length != 2) {
          return undefined;
        }
  
        const params = new URLSearchParams(paramsRaw[1]);
        const id = params.get('id');
  
        if (id == null) {
          return undefined;
        } else {
          return id;
        }
      };
  
      const getId = async (): Promise<string> => {
        const id = getIdInner();
        if (id == null) {
          await router.back();
        }
  
        return id!;
      };
  
      const setDraftStatus = async (setDraft: boolean) => {
        loading.value.changeDraftStatus = true;
        const result = await setAlbumDraftStatus(album.value!, setDraft)
        loading.value.changeDraftStatus = false;
  
        if (result == undefined || !result) {
          snackbar.value = errorText;
          return;
        }
  
        if (setDraft) {
          snackbar.value = "Album unpublished";
        } else {
          snackbar.value = "Album published";
        }
  
        await loadAlbum();
      };
  
      const loadAlbum = async () => {
        loading.value.get = true;
        const result = await getAlbum(await getId(), true);
        loading.value.get = false;
  
        if (result == undefined && result != null) {
          snackbar.value = errorText;
          return;
        }
  
        if (result == null) {
          await router.back();
        }
  
        album.value = result!;
      };
  
      const save = async () => {
        loading.value.save = true;
        const result = await saveEditedAlbum(album.value!);
        loading.value.save = false;
  
        if (result != true) {
          snackbar.value = errorText;
        }
      };
  
      const handleUploadDialogClosed = (success: boolean) => {
        dialog.value.uploadPhoto = false;
  
        if (success) {
          photoGridUpdater.value++;
        }
      };
  
      onMounted(async () => {
        if (!Storage.isAdmin()) {
          const hasScopeUpdate = await checkScope("nl.svsticky.chroma.album.update") ?? false;
          if (!hasScopeUpdate) {
            await router.back();
            return;
          }
  
          canCreatePhotos.value = await checkScope("nl.svsticky.chroma.photo.create") ?? false;
        } else {
          canCreatePhotos.value = true;
        }
  
        await loadAlbum();
      });
  
      return {
        snackbar,
        loading,
        rules,
        album,
        photoGridUpdater,
        valid,
        dialog,
        canCreatePhotos,
        isAdmin,
        getIdInner,
        getId,
        setDraftStatus,
        loadAlbum,
        save,
        handleUploadDialogClosed,
      };
    },
  });
  </script>
  