import Vue from 'vue';
import Vuetify from 'vuetify/lib/framework';

Vue.use(Vuetify);

export default new Vuetify({
    theme: {
        themes: {
            light: {
                primary: "#fa6b20",
                secondary: "#2070fa"
            },
            dark: {
                primary: "#fa6b20",
                secondary: "#2070fa"
            }
        }
    }
});
