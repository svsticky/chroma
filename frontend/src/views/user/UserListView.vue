<template>
    <v-container>
        <EditUserDialog :enabled="dialog.editUser.enabled" :user="dialog.editUser.user"
            @close="dialog.editUser.enabled = false"></EditUserDialog>
        <v-card elevation="2" class="mt-3 pa-3">

            <v-card-title>
                <ReturnButton></ReturnButton>
                Users
            </v-card-title>
            <v-card-subtitle>Manage users registered within Chroma</v-card-subtitle>

            <v-card-text>
                <v-data-table
                    :items="users"
                    :loading="loading"
                    >

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
import { ref, onMounted, computed } from 'vue';
import { listUsers, UserModel } from "@/views/user/user";
import { errorText, Storage } from "@/api";
import { useRouter } from 'vue-router';
import EditUserDialog from "@/components/EditUserDialog.vue";
import ReturnButton from "@/components/ReturnButton.vue";
import { VDataTable } from 'vuetify/labs/VDataTable';
export default {
    components: { ReturnButton, EditUserDialog, VDataTable},
    props: {
        albumId: String,
    },
    setup() {
        const snackbar = ref<string | null>(null);
        const loading = ref<boolean>(true);

        

        const users = ref<UserModel[]>([]);

        const headers = ref([
            { title: 'Name', key: 'name', align: 'start' },
            { title: 'Actions', key: 'actions', sortable: false },
        ]);



        const dialog = ref({
            editUser: {
                user: null as UserModel | null,
                enabled: false,
            }
        });

        const openEditDialog = (user: UserModel) => {
            dialog.value.editUser.user = user;
            dialog.value.editUser.enabled = true;
        };

        const loadUsers = async () => {
            loading.value = true;
            const userList = await listUsers();
            loading.value = false;

            if (!userList) {
                snackbar.value = errorText;
                return;
            }

            users.value = userList;
        };

        onMounted(async () => {
            if (!Storage.isAdmin()) {
                const router = useRouter();
                router.back();
            }

            await loadUsers();
        });

        return {
            snackbar,
            loading,
            users,
            headers,
            dialog,
            openEditDialog,
            loadUsers
        };
    }
}
</script>