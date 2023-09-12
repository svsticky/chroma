<template>
    <v-dialog
        persistent
        max-width="1200"
        v-model="enabled">
        <GrantScopeDialog :user="user" :enabled="dialog.grantScope" @close="grantScopeDialogClosed"></GrantScopeDialog>
        <v-card flat>
            <v-card-title>
                <v-btn
                    @click="$emit('close')"
                    title="Go back"
                    icon
                    small
                    class="mr-1">
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
                                <v-btn
                                    @click="dialog.grantScope = true"
                                    color="primary">
                                    Grant scope
                                </v-btn>
                            </div>
                            <v-data-table
                                :items="scopes"
                                :headers="scopeHeaders"
                                :loading="loading.getScopes">
                                <template v-slot:item.actions="{ item }">
                                    <v-btn
                                        fab
                                        small
                                        :loading="loading.removeScope"
                                        @click="removeUserScope(item)">
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
import Vue, {PropType} from "vue";
import {UserModel, getUserScopes, UserScopeModel, updateUserScopes} from "@/views/user/user";
import {errorText} from "@/api";
import ReturnButton from "@/components/ReturnButton.vue";
import {DataTableHeader} from "vuetify";
import GrantScopeDialog from "@/components/GrantScopeDialog.vue";

interface Data {
    snackbar: string | null,
    loading: {
        getScopes: boolean,
        removeScope: boolean,
    },
    scopes: UserScopeModel[],
    scopeHeaders: DataTableHeader[],
    dialog: {
        grantScope: boolean,
    }
}

export default Vue.extend({
    components: {GrantScopeDialog, ReturnButton},
    data(): Data {
        return {
            snackbar: null,
            loading: {
                getScopes: true,
                removeScope: false,
            },
            scopes: [],
            scopeHeaders: [
                {
                    text: "Scope",
                    value: 'scope'
                },
                {
                    text: "Actions",
                    value: 'actions'
                }
            ],
            dialog: {
                grantScope: false,
            }
        }
    },
    props: {
        enabled: {
            type: Boolean,
            required: true,
        },
        user: {
            type: Object as PropType<UserModel>,
            required: false,
        }
    },
    watch: {
        async enabled() {
            await this.loadUserScopes();
        }
    },
    async mounted() {
        await this.loadUserScopes();
    },
    methods: {
        async grantScopeDialogClosed(ok: boolean) {
            this.dialog.grantScope = false;
            if(ok) {
                await this.loadUserScopes();
            }
        },
        async loadUserScopes() {
            if(this.user == undefined) return;

            this.loading.getScopes = true;
            const scopes = await getUserScopes(this.user!.id);
            this.loading.getScopes = false;

            if(scopes == undefined) {
                this.snackbar = errorText;
                return;
            }

            this.scopes = scopes;
        },
        async removeUserScope(scope: UserScopeModel) {
            this.loading.removeScope = false;

            const newScopes = this.scopes
                .map(f => f.scope)
                .filter(f => f != scope.scope);

            const result = await updateUserScopes(this.user!.id, newScopes);
            this.loading.removeScope = false;

            if(result == undefined || !result) {
                this.snackbar = errorText;
                return;
            }

            await this.loadUserScopes();
        }
    }
})

</script>