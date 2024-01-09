import { createApp } from "vue";
import "./styles.css";
import App from "./App.vue";
import ElementPlus from 'element-plus'
import 'element-plus/dist/index.css'
import { library } from '@fortawesome/fontawesome-svg-core'
import { FontAwesomeIcon } from '@fortawesome/vue-fontawesome'
import { faCalculator,faRetweet,faObjectGroup,faBinoculars,faToolbox,faFileContract } from '@fortawesome/free-solid-svg-icons'


library.add(faCalculator,faRetweet,faObjectGroup,faBinoculars,faToolbox,faFileContract)




const app = createApp(App)
app.component('font-awesome-icon', FontAwesomeIcon)
app.use(ElementPlus)
app.mount("#app");