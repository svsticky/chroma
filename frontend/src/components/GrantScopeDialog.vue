<template>
    <v-dialog
        v-model="enabled"
        max-width="1000"
        persistent>
        <v-card flat>
            <v-card-title>Grant scope</v-card-title>
            <v-card-subtitle v-if="availableScopes.length > 0">Select available scopes to grant to '{{ user.name }}'</v-card-subtitle>
            <v-card-text>
                <v-select
                    multiple
                    v-if="availableScopes.length > 0"
                    :items="availableScopes"
                    v-model="scopes"
                ></v-select>
                <div v-else>
                    All available scopes have already been granted to '{{ user.name }}'
                </div>
            </v-card-text>
            <v-card-actions v-if="availableScopes.length > 0">
                <v-btn
                    @click="$emit('close', false)">
                    Cancel
                </v-btn>
                <v-spacer></v-spacer>
                <v-btn
                    @click="grantScope"
                    color="primary"
                    :disabled="scopes.length == 0"
                    :loading="loading.updateScopes">
                    Grant scope
                </v-btn>
            </v-card-actions>
            <v-card-actions v-else>
                <v-spacer></v-spacer>
                <v-btn
                    color="primary"
                    @click="$emit('close', false)">
                    Cancel
                </v-btn>
            </v-card-actions>
        </v-card>
    </v-dialog>
</template>

<script lang="ts">
import Vue, {PropType} from "vue";
import {getAvailableScopes, getUserScopes, UserModel} from "@/views/user/user";
import {updateUserScopes} from "@/views/user/user";
import {errorText} from "@/api";

interface Data {
    snackbar: string | null,
    loading: {
        availableScopes: boolean,
        updateScopes: boolean,
    },
    scopes: string[],
    availableScopes: string[]
}

export default Vue.extend({
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
        enabled() {
            this.scopes = [];
            this.loadAvailableScopes();
        }
    },
    mounted() {
        this.loadAvailableScopes();
    },
    data(): Data {
        return {
            snackbar: null,
            loading: {
                availableScopes: true,
                updateScopes: false,
            },
            scopes: [],
            availableScopes: [],
        }
    },
    methods: {
        async loadAvailableScopes() {
            this.loading.availableScopes = true;
            const availableScopes = await getAvailableScopes();
            this.loading.availableScopes = false;

            if(availableScopes == undefined) {
                this.snackbar = errorText;
                this.loading.availableScopes = false;
                return;
            }

            const grantedScopes = await getUserScopes(this.user!.id);
            this.loading.availableScopes = false;

            if(grantedScopes == undefined) {
                this.snackbar = errorText;
                return;
            }

            let grantedScopeNames = grantedScopes.map(f => f.scope)
            let scopesGrantable = availableScopes
                .filter(f => !grantedScopeNames.includes(f));

            this.availableScopes = scopesGrantable;
        },
        async grantScope() {
            if(this.user == undefined) return;

            this.loading.updateScopes = true;

            const currentScopes = await getUserScopes(this.user!.id);
            if(currentScopes == undefined) {
                this.loading.updateScopes = false;
                this.snackbar = errorText;
                return;
            }

            let newScopes = currentScopes.map(f => f.scope);
            newScopes = newScopes.concat(this.scopes);

            const result = await updateUserScopes(this.user!.id, newScopes);
            this.loading.updateScopes = false;

            if(result == undefined) {
                this.snackbar = errorText;
                return;
            }

            this.$emit('close', true);

        }
    }
})

</script>