<template>
    <v-dialog
        v-model="enabled"
        max-width="600">
        <v-card flat>
            <v-card-title>Upload photo</v-card-title>
            <v-card-subtitle>PNG or JPEG</v-card-subtitle>
            <v-card-text>
                <v-file-input
                    v-model="photos"
                    prepend-icon="mdi-image-outline"
                    accept=".png,.jpeg,.jpg"
                    multiple
                    chips
                    label="Photo"
                ></v-file-input>
            </v-card-text>
            <v-card-actions>
                <v-btn
                    @click="$emit('close', false)"
                    color="secondary">
                    Cancel
                </v-btn>
                <v-spacer></v-spacer>
                <v-btn
                    @click="upload"
                    color="primary"
                    :loading="loading">
                    Upload
                </v-btn>
            </v-card-actions>
        </v-card>
    </v-dialog>
</template>

<script lang="ts">
import Vue from 'vue';
import {uploadPhoto} from "@/views/album/album";

interface Data {
    snackbar: string | null,
    loading: boolean,
    photos: File[],
}

export default Vue.extend({
    props: {
        enabled: Boolean,
        albumId: String,
    },
    data(): Data {
        return {
            snackbar: null,
            loading: false,
            photos: [],
        }
    },
    methods: {
        async upload() {
            this.loading = true;
            const results = await Promise.all(this.photos.map(async photoFile => {
                const photoBytes = new Uint8Array(await photoFile.arrayBuffer());
                const result = await uploadPhoto(this.albumId, photoBytes);
                return result != undefined;
            }));
            this.loading = false;

            const failures = results.filter(result => result);
            if(failures.length == 0) {
                return;
            }

            this.snackbar = `Failed to upload ${failures.length} photos.`
        }
    }
})
</script>