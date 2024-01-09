import { createApp } from "vue";
// import "./styles.css";
import App from "./App.vue";
import ElementPlus from 'element-plus'
import 'element-plus/dist/index.css'
/* import the fontawesome core */
import { library } from '@fortawesome/fontawesome-svg-core'
/* import font awesome icon component */
import { FontAwesomeIcon } from '@fortawesome/vue-fontawesome'
/* import specific icons */
import { faCalculator,faRetweet,faObjectGroup,faBinoculars,faToolbox } from '@fortawesome/free-solid-svg-icons'

/* add icons to the library */
library.add(faCalculator,faRetweet,faObjectGroup,faBinoculars,faToolbox)




const app = createApp(App)
app.component('font-awesome-icon', FontAwesomeIcon)
app.use(ElementPlus)
app.mount("#app");