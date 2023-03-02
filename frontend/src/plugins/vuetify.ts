import Vue from 'vue';
import Vuetify from 'vuetify/lib/framework';

Vue.use(Vuetify);

export default new Vuetify({
    theme: {
        themes: {
            light: {
                primary: "#29738f",
                secondary: "#90EE02"
            },
            dark: {
                primary: "#29738f",
                secondary: "#90EE02"
            }
        }
    }
});
