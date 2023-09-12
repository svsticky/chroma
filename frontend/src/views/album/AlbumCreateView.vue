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
import Vue from 'vue';
import {InputValidationRules} from "vuetify";
import {errorText, Storage} from "@/api";
import {createAlbum} from "@/views/album/album";
import ReturnButton from "@/components/ReturnButton.vue";

interface Data {
    snackbar: string | null,
    loading: boolean,
    album: {
        name: string | null,
        isDraft: boolean,
    },
    valid: boolean,
    rules: {
        name: InputValidationRules
    }
}

export default Vue.extend({
    components: {ReturnButton},
    data(): Data {
        return {
            snackbar: null,
            loading: false,
            album: {
                name: null,
                isDraft: false,
            },
            valid: true,
            rules: {
                name: [
                    v => !!v || "Name is required",
                ]
            }
        }
    },
    mounted() {
        this.$router.onReady(() => {
            if(!Storage.isAdmin()) {
                this.$router.back();
            }
        })
    },
    methods: {
        async create() {
            this.loading = true;
            const result = await createAlbum(this.album.name!, this.album.isDraft!);
            this.loading = false;

            if(result == undefined) {
                this.snackbar = errorText;
                return;
            }

            this.snackbar = 'Album created';
            await this.$router.push(`/album/edit?id=${result}`);
        }
    }
})
</script>