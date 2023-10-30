import 'vuetify/styles'
import { createVuetify } from 'vuetify'
import * as components from 'vuetify/components'
import * as directives from 'vuetify/directives'

const lightTheme = {
    dark: false,
    colors:{
        primary: "#61518F",
        secondary: "#90EE02"
    }
}
const darkTheme = {
    dark: true,
    colors:{
        primary: "#61518F",
        secondary: "#90EE02"
    }
}

    
export default createVuetify({
    theme:{
        defaultTheme: 'dark',
        themes:{
            light: lightTheme,
            dark: darkTheme
        }
    },
    components,
    directives
});

