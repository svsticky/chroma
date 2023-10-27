import { createVuetify, ThemeDefinition} from 'vuetify';
import 'vuetify/dist/vuetify.min.css';


const lightTheme: ThemeDefinition = {
    dark: false,
    colors:{
        primary: "#61518F",
        secondary: "#90EE02"
    }
}
const darkTheme: ThemeDefinition = {
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
    }
});

