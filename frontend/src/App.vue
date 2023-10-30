<template>
    <v-app>
        <v-app-bar
            app
            color="primary"
            dark>
            <v-card-title>Chroma</v-card-title>

            <v-spacer></v-spacer>

            <v-btn
                v-if="isAdmin && $router.currentRoute.value.fullPath !== '/user'"
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
import { defineComponent, watch } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import { checkLoggedIn, KoalaLoginUrl, LoginCheckResult, Storage } from "@/api";

export default defineComponent({
    setup() {
        const route = useRoute();
        const router = useRouter();

        const isAdmin = Storage.isAdmin();

        watch(() => route.fullPath, async () => {
            await performLoginCheck();
        });

        const toggleDarkMode = () => {
            if(Storage.getIsDarkMode()) {
                Storage.setIsDarkMode(false);
            } else {
                Storage.setIsDarkMode(true);
            }

            // Note: You'll need to adjust this part based on Vuetify's Vue 3 integration specifics
            // this.$vuetify.theme.dark = Storage.getIsDarkMode();
        };

        const navigateToUser = () => {
            if(!isAdmin.valueOf()) {
                return;
            }

            if(route.fullPath !== "/user") {
                router.push('/user');
            }
        };

        const performLoginCheck = async () => {
            if (route.path == '/logged_in') {
                return;
            }

            let loggedIn = await checkLoggedIn();
            if (loggedIn == null) {
                // TODO something went wrong
                return;
            }

            if (loggedIn instanceof KoalaLoginUrl) {
                Storage.setBeforeAuthUrl(route.path);
                window.location.href = loggedIn.url;
                return;
            }


            if (!(loggedIn instanceof LoginCheckResult)) {
                // Future proofing
                console.error("Variable 'item' is not a valid type");
                return;
            }

            // This isn't used for access control. Just to show or hide portions of the UI
            // Someone could set this manually, and it'll show the UI, but they still can't do anything useful.
            Storage.setAdmin(loggedIn.isAdmin);
        };

        return {
            route,
            isAdmin,
            toggleDarkMode,
            navigateToUser,
            performLoginCheck
        };
    }
});
</script>