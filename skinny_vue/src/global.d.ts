import type { IStaticMethods } from 'preline/dist';

declare global {
  interface Window {
    // Optional third-party libraries
    _: typeof import('lodash');
    $: any;
    jQuery: any;
    DataTable: any;
    Dropzone: any;
    noUiSlider: any;
    VanillaCalendarPro: any;

    // Preline UI
    HSStaticMethods: IStaticMethods;
  }
}

export {};
