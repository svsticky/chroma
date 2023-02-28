<template>
    <v-container>
        <div class="d-flex flex-row justify-center">
            <v-card flat elevation="2" class="mt-3">
                <v-card-title>Logged in</v-card-title>
                <v-card-subtitle>
                    You have successfully logged in.
                    You will be redirected automatically.
                </v-card-subtitle>
            </v-card>
        </div>
    </v-container>
</template>

<script lang="ts">
import Vue from 'vue';
import {checkLoggedIn, Storage, KoalaLoginUrl, LoginCheckResult} from "@/api";

export default Vue.extend({
    async mounted() {
        let paramsRaw = window.location.hash.split('?');
        let sessionId;

        if(paramsRaw.length == 2) {
            let params = new URLSearchParams(paramsRaw[1]);
            sessionId = params.get('session_id') ?? "DummyToken";
        } else {
            sessionId = "DummyToken";
        }

        let valid = await checkLoggedIn(sessionId!);
        if(valid == null) {
            // Something went wrong
            return;
        }

        if(valid instanceof KoalaLoginUrl) {
            console.log('Redirecting to Koala..');
            window.location.href = valid.url;
            return;
        }

        if(!(valid instanceof LoginCheckResult)) {
            // Future proofing
            console.error("Variable 'item' is not a valid type");
            return;
        }

        Storage.setSessionId(sessionId);

        // This isnt used for access control. just to show or hide portions of the UI
        // Someone could set this manually, and it'll show the UI, but they still cant do anyhing useful.
        Storage.setAdmin(valid.isAdmin);

        // Redirect the user to where they came from, or back to home
        let beforeAuthUrl = Storage.getBeforeAuthUrl();
        if(beforeAuthUrl != null) {
            await this.$router.push(beforeAuthUrl);
        } else {
            await this.$router.push('/');
        }
    },
})
</script>
