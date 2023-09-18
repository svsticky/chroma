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
                        color="primary">
                        Publish album
                    </v-btn>
                    <v-btn
                        v-else
                        :loading="loading.changeDraftStatus"
                        @click="setDraftStatus(true)"
                        color="primary">
                        Unpublish album
                    </v-btn>
                </div>
            </v-card-title>
            <v-card-text v-if="album != null">
                <v-form
                    v-model="valid">
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
                    @click="save">
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
                        @click="dialog.uploadPhoto = true">
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
import Vue from 'vue';
import {AlbumModel, getAlbum, saveEditedAlbum, setAlbumDraftStatus} from "@/views/album/album";
import {checkScope, errorText, Storage} from "@/api";
import ReturnButton from "@/components/ReturnButton.vue";
import {InputValidationRules} from "vuetify";
import PhotoGrid from "@/components/PhotoGrid.vue";
import UploadPhotoDialog from "@/components/UploadPhotoDialog.vue";

interface Data {
    snackbar: string | null,
    loading: {
        get: boolean,
        save: boolean,
        changeDraftStatus: boolean,
    },
    rules: {
        name: InputValidationRules,
    }
    album: AlbumModel | null,
    photoGridUpdater: number,
    valid: boolean,
    dialog: {
        uploadPhoto: boolean,
    }
    canCreatePhotos: boolean,
}

export default Vue.extend({
    components: {UploadPhotoDialog, PhotoGrid, ReturnButton},
    data(): Data {
        return {
            snackbar: null,
            loading: {
                get: true,
                save: false,
                changeDraftStatus: false,
            },
            rules: {
                name: [
                    v => !!v || "This field is required",
                    v => v.length > 1 || "This field is required",
                ]
            },
            album: null,
            photoGridUpdater: 0,
            valid: true,
            dialog: {
                uploadPhoto: false,
            },
            canCreatePhotos: false,
        }
    },
    computed: {
        isAdmin: () => Storage.isAdmin(),
    },
    async mounted() {
        if(!Storage.isAdmin()) {
            const hasScopeUpdate = await checkScope("nl.svsticky.chroma.album.update") ?? false;
            if(!hasScopeUpdate) {
                await this.$router.back();
                return;
            }

            this.canCreatePhotos = await checkScope("nl.svsticky.chroma.photo.create") ?? false;
        } else {
            this.canCreatePhotos = true;
        }

        await this.loadAlbum();
    },
    methods: {
        getIdInner(): string | undefined {
            const paramsRaw = window.location.hash.split('?');
            if(paramsRaw.length != 2) {
                return undefined;
            }

            const params = new URLSearchParams(paramsRaw[1]);
            const id = params.get('id');

            if(id == null) {
                return undefined;
            } else {
                return id;
            }
        },
        async getId(): Promise<string> {
            const id = this.getIdInner();
            if(id == null) {
                await this.$router.back();
            }

            return id!;
        },
        async setDraftStatus(setDraft: boolean) {
            this.loading.changeDraftStatus = true;
            const result = await setAlbumDraftStatus(this.album!, setDraft)
            this.loading.changeDraftStatus = false;

            if(result == undefined || !result) {
                this.snackbar = errorText;
                return;
            }

            if(setDraft) {
                this.snackbar = "Album unpublished";
            } else {
                this.snackbar = "Album published";
            }

            await this.loadAlbum();
        },
        async loadAlbum() {
            this.loading.get = true;
            const result = await getAlbum(await this.getId(), true);
            this.loading.get = false;

            if(result == undefined && result != null) {
                this.snackbar = errorText;
                return;
            }

            if(result == null) {
                await this.$router.back();
            }

            this.album = result!;
        },
        async save() {
            this.loading.save = true;
            const result = await saveEditedAlbum(this.album!);
            this.loading.save = false;

            if(result != true) {
                this.snackbar = errorText;
            }
        },
        handleUploadDialogClosed(success: boolean) {
            this.dialog.uploadPhoto = false;

            if(success) {
                this.photoGridUpdater++;
            }
        }
    }

})
</script>