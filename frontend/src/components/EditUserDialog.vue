<template>
    <v-dialog persistent max-width="1200" v-model="enabled">
        <GrantScopeDialog :user="user" :enabled="dialog.grantScope" @close="grantScopeDialogClosed"></GrantScopeDialog>
        <v-card flat>
            <v-card-title>
                <v-btn @click="$emit('close')" title="Go back" icon small class="mr-1">
                    <v-icon>mdi-arrow-left</v-icon>
                </v-btn>
                Edit user
            </v-card-title>
            <v-card-subtitle v-if="user != undefined">Editing '{{ user.name }}'</v-card-subtitle>

            <v-card-text>
                <v-expansion-panels>
                    <v-expansion-panel>
                        <v-expansion-panel-header>Scopes</v-expansion-panel-header>
                        <v-expansion-panel-content>
                            <div class="d-flex flex-row justify-end">
                                <v-btn @click="dialog.grantScope = true" color="primary">
                                    Grant scope
                                </v-btn>
                            </div>
                            <v-data-table :items="scopes" :headers="scopeHeaders" item-key="scope" class="elevation-1">
                                <template v-slot:item.actions="{ item }">
                                    <v-btn fab small :loading="loading.removeScope" @click="removeUserScope(item)">
                                        <v-icon>mdi-delete-outline</v-icon>
                                    </v-btn>
                                </template>
                            </v-data-table>
                        </v-expansion-panel-content>
                    </v-expansion-panel>
                </v-expansion-panels>
            </v-card-text>
            <v-card-actions>
                <v-spacer></v-spacer>
                <v-btn color="primary" @click="$emit('close')">
                    Close
                </v-btn>
            </v-card-actions>
        </v-card>
    </v-dialog>
</template>


<script lang="ts">
import { defineComponent, ref, watch, onMounted, PropType } from 'vue';
import { UserModel, getUserScopes, UserScopeModel, updateUserScopes } from "@/views/user/user";
import { errorText } from "@/api";
import GrantScopeDialog from "@/components/GrantScopeDialog.vue";

export default defineComponent({
    components: { GrantScopeDialog },
    props: {
        enabled: Boolean,
        user: Object as PropType<UserModel | null>,
    },
    setup(props) {
        const snackbar = ref<string | null>(null);
        const loading = ref({
            getScopes: true,
            removeScope: false,
        });

        const users = ref<UserModel[]>([]);
        const headers = ref([
            { text: 'Name', align: 'start', value: 'name' },
            { text: 'Actions', value: 'actions' }
        ]);

        const scopes = ref<UserScopeModel[]>([]);
        const scopeHeaders = ref([
            { text: "Scope", value: 'scope' },
            { text: "Actions", value: 'actions' }
        ]);
        const dialog = ref({
            grantScope: false,
        });

        const loadUserScopes = async () => {
            if (!props.user) return;

            loading.value.getScopes = true;
            const userScopes = await getUserScopes(props.user.id);
            loading.value.getScopes = false;

            if (!userScopes) {
                snackbar.value = errorText;
                return;
            }

            scopes.value = userScopes;
        };

        const grantScopeDialogClosed = async (ok: boolean) => {
            dialog.value.grantScope = false;
            if (ok) {
                await loadUserScopes();
            }
        };

        const removeUserScope = async (scope: UserScopeModel) => {
            loading.value.removeScope = true;

            const newScopes = scopes.value.map(f => f.scope).filter(f => f !== scope.scope);
            const result = await updateUserScopes(props.user!.id, newScopes);
            loading.value.removeScope = false;

            if (!result) {
                snackbar.value = errorText;
                return;
            }

            await loadUserScopes();
        };

        watch(() => props.enabled, async () => {
            await loadUserScopes();
        });

        onMounted(async () => {
            await loadUserScopes();
        });

        return {
            snackbar,
            loading,
            scopes,
            scopeHeaders,
            dialog,
            loadUserScopes,
            grantScopeDialogClosed,
            removeUserScope
        };
    }
})
</script>
