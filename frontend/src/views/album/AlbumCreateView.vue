<template>
    <v-container>
        <v-card elevation="2" class="mt-3 pa-3">
            <v-card-title>
                <ReturnButton></ReturnButton>
                Create album
            </v-card-title>
            <v-card-text>
                <v-form v-model="valid">
                    <v-text-field
                        v-model="album.name"
                        label="Name"
                        :rules="rules.name"
                        counter="64"
                    ></v-text-field>
                    <v-checkbox
                        v-model="album.isDraft"
                        :disabled="forceDraft"
                        label="Draft"
                    ></v-checkbox>
                </v-form>
            </v-card-text>
            <v-card-actions>
                <v-btn
                    color="primary"
                    :disabled="!valid"
                    :loading="loading"
                    @click="create">
                    Create
                </v-btn>
            </v-card-actions>
        </v-card>
    </v-container>
</template>

<script lang="ts">
import { defineComponent, ref } from 'vue';
import { useRouter } from 'vue-router';
import { checkScope, errorText, Storage } from "@/api";
import { createAlbum } from "@/views/album/album";
import ReturnButton from "@/components/ReturnButton.vue";

export default defineComponent({
    name: 'CreateAlbumComponent',
    components: { ReturnButton },
    setup() {
        const router = useRouter();
        const snackbar = ref<string | null>(null);
        const loading = ref<boolean>(false);
        const album = ref({
            name: null as string | null,
            isDraft: false
        });
        const valid = ref<boolean>(true);
        const rules = ref({
            name: [
                (v: string) => !!v || "Name is required",
            ]
        });
        const forceDraft = ref<boolean>(true);

        const loadForceDraft = async () => {
            forceDraft.value = !Storage.isAdmin();
            if(forceDraft.value) {
                album.value.isDraft = true;
            }
        };

        const create = async () => {
            loading.value = true;
            const result = await createAlbum(album.value.name!, album.value.isDraft!);
            loading.value = false;

            if(result == undefined) {
                snackbar.value = errorText;
                return;
            }

            snackbar.value = 'Album created';
            await router.push(`/album/edit?id=${result}`);
        };
        if(!Storage.isAdmin()) {
            checkScope("nl.svsticky.chroma.album.create").then(hasScope => {
                if(!hasScope) {
                    router.back();
                    return;
                }
            });
        }

        loadForceDraft();


        return {
            snackbar,
            loading,
            album,
            valid,
            rules,
            forceDraft,
            loadForceDraft,
            create
        };
    }
});
</script>