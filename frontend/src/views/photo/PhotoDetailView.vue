<template>
    <v-dialog
        persistent
        v-model="enabled">
        <v-card flat :loading="loading.w1600">
            <v-card-title>View image</v-card-title>
            <v-card-text>
                <div v-if="!loading.w1600">
                    <v-img
                        width="87%"
                        aspect-ratio="1.7778"
                        class="mx-auto"
                        cover
                        :src="highQualitySrc">
                    </v-img>
                </div>
            </v-card-text>
            <v-card-actions>
                <v-spacer></v-spacer>
                <v-btn
                    color="primary"
                    :loading="loading.original"
                    @click="downloadOriginal">
                    Download
                </v-btn>
                <v-btn
                    color="primary"
                    @click="$emit('close')">
                    Close
                </v-btn>
            </v-card-actions>
        </v-card>
    </v-dialog>
</template>

<script lang="ts">
import Vue, {PropType} from 'vue';
import {getPhoto, PhotoModel, Quality} from "@/views/photo/photo";
import {errorText} from "@/api";

interface Data {
    loading: {
        w1600: boolean,
        original: boolean,
    },
    snackbar: string | null,
    highQualitySrc: string | null,
}

export default Vue.extend({
    data(): Data {
        return <Data> {
            snackbar: null,
            loading: {
                w1600: true,
                original: false,
            },
            highQualitySrc: null,
        }
    },
    props: {
        enabled: {
            type: Boolean,
            required: true,
        },
        photo: {
            type: Object as PropType<PhotoModel>,
            required: true,
        }
    },
    watch: {
        enabled() {
            this.loadHighQuality();
        }
    },
    mounted() {
        this.loadHighQuality();
    },
    methods: {
        async downloadOriginal() {
            this.loading.original = true;
            const result = await getPhoto(this.photo.id, Quality.ORIGINAL);

            if(result == undefined) {
                this.snackbar = errorText;
                this.loading.original = false;
            }

            result?.downloadOrNewTab();
            this.loading.original = false;
        },
        async loadHighQuality() {
            this.loading.w1600 = true;
            const result = await getPhoto(this.photo.id, Quality.W1600);

            if(result == undefined) {
                this.snackbar = errorText;
                this.loading.w1600 = false;
            }

            this.highQualitySrc = result!.getAsSrcUrl();
            this.loading.w1600 = false;
        }
    }
})
</script>
