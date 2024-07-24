<template>
    <v-dialog
        persistent
        v-model="enabled"
        max-width="600">
        <v-card flat>
            <v-card-title>Upload photo(s)</v-card-title>
            <v-card-subtitle>PNG or JPEG</v-card-subtitle>
            <v-card-text>
                <v-file-input
                    v-model="photos"
                    prepend-icon="mdi-image-outline"
                    accept=".png,.jpeg,.jpg"
                    multiple
                    chips
                    :disabled="loading"
                    label="Photo"
                ></v-file-input>
            </v-card-text>
            <v-card-actions>
                <v-btn
                    :disabled="loading"
                    @click="close(false)">
                    Cancel
                </v-btn>
                <v-spacer></v-spacer>
                <div v-if="loading" class="mr-1">
                    <span class="primary--text">{{ uploadProgress }} / {{ uploadTotal}}</span>
                </div>
                <v-btn
                    @click="upload"
                    color="primary"
                    :disabled="loading"
                    :loading="loading">
                    Upload
                </v-btn>
            </v-card-actions>
        </v-card>
    </v-dialog>
</template>

<script lang="ts">
import Vue from 'vue';
import {createPhoto, TooManyRequests} from "@/views/photo/photo";

interface Data {
    snackbar: string | null,
    loading: boolean,
    photos: File[],
    uploadProgress: number,
    uploadTotal: number,
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
            uploadProgress: 0,
            uploadTotal: 0,
        }
    },
    methods: {
        async upload() {
            this.loading = true;

            this.uploadProgress = 0;
            this.uploadTotal = this.photos.length;

            let results = [];
            for(const photoFile of this.photos) {
                while(true) {
                    const photoBytes = new Uint8Array(await photoFile.arrayBuffer());
                    let result: boolean | undefined;
                    try {
                        result = await createPhoto(this.albumId, photoBytes);
                    } catch (e: any) {
                        if(e instanceof TooManyRequests) {
                            console.log(`Got HTTP 429. Waiting ${e.retryAfter} seconds`);
                            await new Promise(resolve => setTimeout(resolve, e.retryAfter));
                        }

                        continue;
                    }

                    if(result === true) {
                        break;
                    }

                    console.error("Got unknown error. Bailing");
                    return false;
                }

                this.uploadProgress++;

                results.push(photoFile);
            }

            this.loading = false;

            const failures = results.filter(result => !result);
            if(failures.length == 0) {
                // Success
                this.close(true);
                return;
            }

            this.snackbar = `Failed to upload ${failures.length} photos.`
        },
        close(success: boolean) {
            this.photos = [];
            this.$emit('close', success);
        }
    }
})
</script>