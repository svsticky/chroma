<template>
    <v-btn
        @click="$router.back()"
        title="Go back"
        icon
        small
        class="mr-1">
        <v-icon>mdi-arrow-left</v-icon>
    </v-btn>
</template>

<script lang="ts">
import Vue from 'vue';

export default Vue.extend({})
</script>

<style scoped>
.v-btn::before {
    background-color: transparent;
}
</style>