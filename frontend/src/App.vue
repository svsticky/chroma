<template>
  <v-app>
    <v-app-bar
      app
      color="primary"
      dark>
        <v-card-title>Chroma</v-card-title>
    </v-app-bar>
    <v-main>
      <router-view/>
    </v-main>
  </v-app>
</template>

<script lang="ts">
import Vue from 'vue';
import {checkLoggedIn, KoalaLoginUrl, LoginCheckResult, setAdmin, setBeforeAuthUrl} from "@/api";

export default Vue.extend({
    async mounted() {
        this.$router.onReady(async () => {
            if(this.$router.currentRoute.path == '/logged_in') {
                return;
            }

            let loggedIn = await checkLoggedIn();
            if(loggedIn == null) {
                // TODO something went wrong
                return;
            }

            if(loggedIn instanceof KoalaLoginUrl) {
                setBeforeAuthUrl(this.$router.currentRoute.path);
                window.location.href = loggedIn.url;
                return;
            }

            if(!(loggedIn instanceof LoginCheckResult)) {
                // Future proofing
                console.error("Variable 'item' is not a valid type");
                return;
            }

            // This isnt used for access control. just to show or hide portions of the UI
            // Someone could set this manually, and it'll show the UI, but they still cant do anyhing useful.
            setAdmin(loggedIn.isAdmin);

            // User is logged in
        })
    }
});
</script>
