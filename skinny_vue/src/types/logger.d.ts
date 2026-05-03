import { Logger } from "@/utils/logger";

declare module "@vue/runtime-core" {
    interface ComponentCustomProperties {
        $log: Logger;
    }
}
