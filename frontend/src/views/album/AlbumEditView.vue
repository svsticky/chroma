<template>
    <v-container>
        <UploadPhotoDialog
            :enabled="dialog.uploadPhoto"
            :album-id="getIdInner()"
            @close="handleUploadDialogClosed"
        ></UploadPhotoDialog>

        <v-card elevation="2" class="mt-3 pa-3" :loading="loading.get">
            <v-card-title>
                <ReturnButton></ReturnButton>
                Edit album
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
import {AlbumModel, getAlbum, saveEditedAlbum} from "@/views/album/album";
import {errorText, Storage} from "@/api";
import ReturnButton from "@/components/ReturnButton.vue";
import {InputValidationRules} from "vuetify";
import PhotoGrid from "@/components/PhotoGrid.vue";
import UploadPhotoDialog from "@/components/UploadPhotoDialog.vue";

interface Data {
    snackbar: string | null,
    loading: {
        get: boolean,
        save: boolean,
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
}

export default Vue.extend({
    components: {UploadPhotoDialog, PhotoGrid, ReturnButton},
    data(): Data {
        return {
            snackbar: null,
            loading: {
                get: true,
                save: false,
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
            }
        }
    },
    async mounted() {
        if(!Storage.isAdmin()) {
            await this.$router.back();
            return;
        }

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