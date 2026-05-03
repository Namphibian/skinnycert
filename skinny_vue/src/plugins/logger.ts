import type { App } from "vue";
import { logger } from "@/utils/logger";

export default {
    install(app: App) {
        app.config.globalProperties.$log = logger;
    }
};
