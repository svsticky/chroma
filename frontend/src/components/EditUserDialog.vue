<template>
    <v-dialog
        persistent
        v-model="enabled">
        <v-card flat>
            <v-card-title>Edit user</v-card-title>
            <v-card-subtitle>Editing {{ user.name }}</v-card-subtitle>

            <v-card-text>
                
            </v-card-text>

            <v-card-actions>

            </v-card-actions>
        </v-card>
    </v-dialog>
</template>

<script lang="ts">
import Vue, {PropType} from "vue";
import {UserModel, getUserScopes, UserScopeModel} from "@/views/user/user";
import {errorText} from "@/api";

interface Data {
    snackbar: string | null,
    loading: {
        getScopes: boolean,
        updateScopes: boolean,
    },
    scopes: UserScopeModel[],
}

export default Vue.extend({
    data(): Data {
        return {
            snackbar: null,
            loading: {
                getScopes: true,
                updateScopes: false,
            },
            scopes: [],
        }
    },
    props: {
        enabled: {
            type: Boolean,
            required: true,
        },
        user: {
            type: Object as PropType<UserModel>,
            required: true,
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
        async loadUserScopes() {
            this.loading.getScopes = true;
            const scopes = await getUserScopes(this.user.id);
            this.loading.getScopes = false;

            if(scopes == undefined) {
                this.snackbar = errorText;
                return;
            }

            this.scopes = scopes;
        }
    }
})

</script>