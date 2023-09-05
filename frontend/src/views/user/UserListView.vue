<template>
    <v-container>
        <v-card
            elevation="2"
            class="mt-3 pa-3">

            <v-card-title>Users</v-card-title>
            <v-card-subtitle>Manage users registered within Chroma</v-card-subtitle>

            <v-card-text>
                <v-data-table
                    :items="users"
                    :loading="loading"
                    :headers="headers">

                    <template v-slot:item.actions="{ item }">
                        <v-btn
                            fab
                            small
                            @click="openEditDialog">
                            <v-icon>mdi-account-edit</v-icon>
                        </v-btn>
                    </template>
                </v-data-table>
            </v-card-text>
        </v-card>
    </v-container>
</template>

<script lang="ts">
import Vue from 'vue';
import {listUsers, UserModel} from "@/views/user/user";
import {errorText, Storage} from "@/api";
import {DataTableHeader} from "vuetify";

interface Data {
    snackbar: string | null,
    loading: boolean,
    users: UserModel[],
    headers: DataTableHeader[],
}

export default Vue.extend({
    data(): Data {
        return {
            snackbar: null,
            loading: true,
            users: [],
            headers: [
                {
                    text: "Naam",
                    value: "name"
                },
                {
                    text: "Acties",
                    value: "actions",
                }
            ]
        }
    },
    async mounted() {
        if(!Storage.isAdmin()) {
            await this.$router.back();
            return;
        }

        await this.loadUsers();
    },
    methods: {
        openEditDialog() {
            // TODO
        },
        async loadUsers() {
            this.loading = true;
            const users = await listUsers();
            this.loading = false;

            if(users == undefined) {
                this.snackbar = errorText;
                return;
            }

            this.users = users;
        }
    }
})
</script>
