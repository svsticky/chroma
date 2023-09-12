<template>
    <v-container>
        <EditUserDialog :enabled="dialog.editUser.enabled" :user="dialog.editUser.user" @close="dialog.editUser.enabled = false"></EditUserDialog>
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
                            @click="openEditDialog(item)">
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
import EditUserDialog from "@/components/EditUserDialog.vue";

interface Data {
    snackbar: string | null,
    loading: boolean,
    users: UserModel[],
    headers: DataTableHeader[],
    dialog: {
        editUser: {
            user: UserModel | null,
            enabled: boolean,
        }
    }
}

export default Vue.extend({
    components: {EditUserDialog},
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
            ],
            dialog: {
                editUser: {
                    user: null,
                    enabled: false,
                }
            }
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
        openEditDialog(user: UserModel) {
            this.dialog.editUser.user = user;
            this.dialog.editUser.enabled = true;
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
