<template>
    <v-btn
        @click="goBack"
        title="Go back"
        icon
        small
        class="mr-1">
        <v-icon>mdi-arrow-left</v-icon>
    </v-btn>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import { useRouter } from 'vue-router';

export default defineComponent({
    name: 'ReturnButtonComponent',
    setup() {
        const router = useRouter();

        const goBack = () => {
            router.back();
        };

        return {
            goBack
        };
    }
});
</script>

<style scoped>
.v-btn::before {
    background-color: transparent;
}
</style>
