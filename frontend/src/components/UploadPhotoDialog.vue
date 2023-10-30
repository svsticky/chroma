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
                    :loading="loading">
                    Upload
                </v-btn>
            </v-card-actions>
        </v-card>
    </v-dialog>
</template>

<script lang="ts">
import { defineComponent, ref } from 'vue';
import { createPhoto } from "@/views/photo/photo";

export default defineComponent({
    name: 'UploadPhotoComponent',
    props: {
        enabled: Boolean,
        albumId: String,
    },
    setup(props, { emit }) {
        const snackbar = ref<string | null>(null);
        const loading = ref<boolean>(false);
        const photos = ref<File[]>([]);
        const uploadProgress = ref<number>(0);
        const uploadTotal = ref<number>(0);

        const upload = async () => {
            loading.value = true;

            uploadProgress.value = 0;
            uploadTotal.value = photos.value.length;
            if (props.albumId == null ){
                snackbar.value = 'Album ID is null';
                return;
            }


            const results = await Promise.all(photos.value.map(async photoFile => {
                const photoBytes = new Uint8Array(await photoFile.arrayBuffer());
                const result = await createPhoto(props.albumId!, photoBytes);

                uploadProgress.value++;

                return result !== undefined;
            }));
            loading.value = false;

            const failures = results.filter(result => !result);
            if(failures.length === 0) {
                // Success
                close(true);
                return;
            }

            snackbar.value = `Failed to upload ${failures.length} photos.`;
        };

        const close = (success: boolean) => {
            photos.value = [];
            emit('close', success);
        };

        return {
            snackbar,
            loading,
            photos,
            uploadProgress,
            uploadTotal,
            upload,
            close
        };
    }
});
</script>