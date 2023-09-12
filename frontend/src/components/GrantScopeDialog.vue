<template>
    <v-dialog
        v-model="enabled"
        max-width="1000"
        persistent>
        <v-card flat>
            <v-card-title>Grant scope</v-card-title>
            <v-card-subtitle>Editing '{{ user.name }}'</v-card-subtitle>
            <v-card-text>
                <v-form v-model="valid">
                    <v-text-field
                        v-model="scope"
                        :rules="scopeRules"
                        counter="128"
                    ></v-text-field>
                </v-form>
            </v-card-text>
            <v-card-actions>
                <v-btn
                    @click="$emit('close', false)">
                    Cancel
                </v-btn>
                <v-spacer></v-spacer>
                <v-btn
                    @click="grantScope"
                    color="primary"
                    :disabled="!valid"
                    :loading="loading">
                    Grant scope
                </v-btn>
            </v-card-actions>
        </v-card>
    </v-dialog>
</template>

<script lang="ts">
import Vue, {PropType} from "vue";
import {getUserScopes, UserModel} from "@/views/user/user";
import {updateUserScopes} from "@/views/user/user";
import {InputValidationRules} from "vuetify";
import {errorText} from "@/api";

interface Data {
    snackbar: string | null,
    loading: boolean,
    scope: string | null,
    valid: boolean,
    scopeRules: InputValidationRules,
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
            this.scope = null;
            this.valid = true;
        }
    },
    data(): Data {
        return {
            snackbar: null,
            loading: false,
            scope: null,
            valid: true,
            scopeRules: [
                v => !!v || "Required"
            ]
        }
    },
    methods: {
        async grantScope() {
            if(this.user == undefined) return;

            this.loading = true;

            const currentScopes = await getUserScopes(this.user!.id);
            if(currentScopes == undefined) {
                this.loading = false;
                this.snackbar = errorText;
                return;
            }

            let newScopes = currentScopes.map(f => f.scope);
            newScopes.push(this.scope!);

            const result = await updateUserScopes(this.user!.id, newScopes);
            this.loading = false;

            if(result == undefined) {
                this.snackbar = errorText;
                return;
            }

            this.$emit('close', true);

        }
    }
})

</script>