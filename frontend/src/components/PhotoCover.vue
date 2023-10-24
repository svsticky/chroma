<template>
    <div>
        <PhotoDetailView
            :photo="photo"
            :enabled="detailViewEnabled"
            @close="detailViewEnabled = false"
        ></PhotoDetailView>

        <v-img
            class="mx-auto"
            width="80%"
            aspect-ratio="1.7778"
            cover
            lazy-src="@/assets/hoofd_outline_color.png"
            @click="detailViewEnabled = true"
            :src="coverPhotoUrl">
            <v-btn
                v-if="canSetThumbnail"
                style="position: absolute; right: 12%; top: 2%"
                color="primary"
                @click="$emit('select-cover')"
                title="Set as cover photo"
                fab
                small>
                <!-- Show a filled heart if the image is the cover, else show only an outline -->
                <v-icon>{{ isCover ?? false ? 'mdi-heart' : 'mdi-heart-outline'}}</v-icon>
            </v-btn>

            <v-btn
                v-if="canDelete"
                style="position: absolute; right: 2%; top: 2%"
                color="primary"
                @click="$emit('deleted')"
                title="Remove photo from album"
                fab
                small>
                <v-icon>mdi-trash-can-outline</v-icon>
            </v-btn>
        </v-img>
    </div>
</template>

<script lang="ts">
import Vue, {PropType} from 'vue';
import {PhotoDataKind, PhotoModel} from "@/views/photo/photo";
import PhotoDetailView from "@/views/photo/PhotoDetailView.vue";

interface Data {
    detailViewEnabled: boolean,
}

export default Vue.extend({
    components: {PhotoDetailView},
    props: {
        photo: {
            type: Object as PropType<PhotoModel>,
            required: true,
        },
        canSetThumbnail: Boolean,
        canDelete: Boolean,
        isCover: {
            type: Boolean,
            required: false,
        }
    },
    computed: {
        coverPhotoUrl(): string {
            return this.photo.getAsSrcUrl();
        }
    },
    data(): Data {
        return {
            detailViewEnabled: false,
        }
    }
})
</script>