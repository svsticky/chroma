<template>
    <v-app>
        <v-app-bar
            app
            color="primary"
            dark>
            <v-card-title>Chroma</v-card-title>

            <v-spacer></v-spacer>

            <v-btn
                v-if="isAdmin && $router.currentRoute.fullPath !== '/user'"
                icon
                small
                class="mr-3"
                @click="navigateToUser">
                <v-icon>mdi-cog-outline</v-icon>
            </v-btn>
            <v-btn
                icon
                small
                class="mr-3"
                @click="toggleDarkMode">
                <v-icon>mdi-theme-light-dark</v-icon>
            </v-btn>
        </v-app-bar>
        <v-main>
            <router-view/>
        </v-main>
    </v-app>
</template>

<script lang="ts">
import Vue from 'vue';
import {checkLoggedIn, KoalaLoginUrl, LoginCheckResult, Storage} from "@/api";

export default Vue.extend({
    watch: {
        async $route(to, from) {
            await this.performLoginCheck();
        }
    },
    computed: {
        isAdmin(): boolean {
            return Storage.isAdmin();
        }
    },
    async mounted() {
        this.$vuetify.theme.dark = Storage.getIsDarkMode();

        this.$router.onReady(async () => {
            await this.performLoginCheck();
        })
    },
    methods: {
        toggleDarkMode() {
            if(Storage.getIsDarkMode()) {
                Storage.setIsDarkMode(false);
            } else {
                Storage.setIsDarkMode(true);
            }

            this.$vuetify.theme.dark = Storage.getIsDarkMode();
        },
        navigateToUser() {
            if(!this.isAdmin) {
                return;
            }

            if(this.$router.currentRoute.fullPath !== "/user") {
                this.$router.push('/user');
            }
        },
        async performLoginCheck() {
            if (this.$router.currentRoute.path == '/logged_in') {
                return;
            }

            let loggedIn = await checkLoggedIn();
            if (loggedIn == null) {
                // TODO something went wrong
                return;
            }

            if (loggedIn instanceof KoalaLoginUrl) {
                Storage.setBeforeAuthUrl(this.$router.currentRoute.path);
                window.location.href = loggedIn.url;
                return;
            }

            if (!(loggedIn instanceof LoginCheckResult)) {
                // Future proofing
                console.error("Variable 'item' is not a valid type");
                return;
            }

            // This isnt used for access control. just to show or hide portions of the UI
            // Someone could set this manually, and it'll show the UI, but they still cant do anyhing useful.
            Storage.setAdmin(loggedIn.isAdmin);

            // User is logged in
        }
    }
});
</script>