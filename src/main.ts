import { createApp } from "vue";
import "./styles.css";
import App from "./App.vue";

import { FontAwesomeIcon } from "@fortawesome/vue-fontawesome";
import { library } from "@fortawesome/fontawesome-svg-core";
import { faAsterisk, faHashtag, faTrash, faA, faB, faC, faD, fa0, fa1, fa2, fa3, fa4, fa5, fa6, fa7, fa8, fa9, faDeleteLeft, faFileArrowDown } from "@fortawesome/free-solid-svg-icons";

library.add(faAsterisk, faHashtag, faTrash, faA, faB, faC, faD, fa0, fa1, fa2, fa3, fa4, fa5, fa6, fa7, fa8, fa9, faDeleteLeft, faFileArrowDown);

createApp(App).mount("#app");
