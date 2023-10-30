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
import { defineComponent, ref, watch, onMounted, PropType} from 'vue';
import { getAvailableScopes, getUserScopes, UserModel, updateUserScopes } from "@/views/user/user";
import { errorText } from "@/api";

export default defineComponent({
    props: {
        enabled: Boolean,
        user: Object as PropType<UserModel | undefined>,
    },
    setup(props, context) {
        const snackbar = ref<string | null>(null);
        const loading = ref({
            availableScopes: true,
            updateScopes: false,
        });
        const scopes = ref<string[]>([]);
        const availableScopes = ref<string[]>([]);

        const loadAvailableScopes = async () => {
            loading.value.availableScopes = true;
            const available = await getAvailableScopes();
            loading.value.availableScopes = false;

            if (!available) {
                snackbar.value = errorText;
                return;
            }

            const grantedScopes = await getUserScopes(props.user!.id);
            if (!grantedScopes) {
                snackbar.value = errorText;
                return;
            }

            const grantedScopeNames = grantedScopes.map(f => f.scope);
            const scopesGrantable = available.filter(f => !grantedScopeNames.includes(f));
            availableScopes.value = scopesGrantable;
        };

        const grantScope = async () => {
            if (!props.user) return;

            loading.value.updateScopes = true;
            const currentScopes = await getUserScopes(props.user.id);
            if (!currentScopes) {
                loading.value.updateScopes = false;
                snackbar.value = errorText;
                return;
            }

            const newScopesList = [...currentScopes.map(f => f.scope), ...scopes.value];
            const result = await updateUserScopes(props.user.id, newScopesList);
            loading.value.updateScopes = false;

            if (!result) {
                snackbar.value = errorText;
                return;
            }

            context.emit('close', true);
        };

        watch(() => props.enabled, () => {
            scopes.value = [];
            loadAvailableScopes();
        });

        onMounted(loadAvailableScopes);

        return {
            snackbar,
            loading,
            scopes,
            availableScopes,
            loadAvailableScopes,
            grantScope
        };
    }
})
</script>