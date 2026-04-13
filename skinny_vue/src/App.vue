<script setup lang="ts">
import { onMounted, watch } from 'vue';
import { useRoute } from 'vue-router';
import { HSStaticMethods } from 'preline';

const route = useRoute();

onMounted(() => {
  HSStaticMethods.autoInit();
});

watch(
  () => route.path,
  () => {
    setTimeout(() => {
      HSStaticMethods.autoInit();
    }, 100);
  }
);
</script>

<template>
  <div class="bg-gray-50 dark:bg-neutral-950 min-h-screen">
    <!-- Sidebar -->
    <div id="application-sidebar" class="hs-overlay [--auto-close:lg] hs-overlay-open:translate-x-0 -translate-x-full transition-all duration-300 transform w-[260px] fixed inset-y-0 start-0 z-[60] bg-white border-e border-gray-200 lg:block lg:translate-x-0 lg:end-auto lg:bottom-0 dark:bg-neutral-800 dark:border-neutral-700">
      <div class="px-8 pt-4">
        <!-- Logo -->
        <router-link to="/" class="flex-none flex items-center gap-x-2 rounded-xl text-xl font-semibold focus:outline-none focus:opacity-80 dark:text-white" aria-label="Brand">
          <img class=32 src="/src/assets/skinnycert.png" alt="Logo">
        </router-link>
        <!-- End Logo -->
      </div>

      <nav class="hs-accordion-group p-6 w-full flex flex-col flex-wrap" data-hs-accordion-always-open>
        <ul class="space-y-1.5">
          <li>
            <router-link to="/certificates" class="flex items-center gap-x-3.5 py-2 px-2.5 text-sm text-gray-700 rounded-lg hover:bg-gray-100 focus:outline-none focus:bg-gray-100 dark:bg-neutral-800 dark:text-neutral-400 dark:hover:text-neutral-300 dark:hover:bg-neutral-700 dark:focus:bg-neutral-700" :class="{ 'bg-gray-100 dark:bg-neutral-700 dark:text-white': route.path.startsWith('/certificates') }">
              <svg class="shrink-0 size-4" xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z"/></svg>
              Certificates
            </router-link>
          </li>

          <li>
            <router-link to="/keys" class="flex items-center gap-x-3.5 py-2 px-2.5 text-sm text-gray-700 rounded-lg hover:bg-gray-100 focus:outline-none focus:bg-gray-100 dark:bg-neutral-800 dark:text-neutral-400 dark:hover:text-neutral-300 dark:hover:bg-neutral-700 dark:focus:bg-neutral-700" :class="{ 'bg-gray-100 dark:bg-neutral-700 dark:text-white': route.path === '/keys' }">
              <svg class="shrink-0 size-4" xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m15.5 7.5 2.3 2.3a1 1 0 0 0 1.4 0l2.1-2.1a1 1 0 0 0 0-1.4L19 4"/><path d="m10.5 12.5 2.3 2.3a1 1 0 0 0 1.4 0l2.1-2.1a1 1 0 0 0 0-1.4L14 9"/><path d="m3.5 19.5 2.3 2.3a1 1 0 0 0 1.4 0l2.1-2.1a1 1 0 0 0 0-1.4L7 16"/><path d="m5 13 6-6"/><path d="m2 16 6-6"/></svg>
              Keys
            </router-link>
          </li>

          <li>
            <router-link to="/health" class="flex items-center gap-x-3.5 py-2 px-2.5 text-sm text-gray-700 rounded-lg hover:bg-gray-100 focus:outline-none focus:bg-gray-100 dark:bg-neutral-800 dark:text-neutral-400 dark:hover:text-neutral-300 dark:hover:bg-neutral-700 dark:focus:bg-neutral-700" :class="{ 'bg-gray-100 dark:bg-neutral-700 dark:text-white': route.path === '/health' }">
              <svg class="shrink-0 size-4" xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M22 12h-4l-3 9L9 3l-3 9H2"/></svg>
              Health
            </router-link>
          </li>
        </ul>
      </nav>
    </div>
    <!-- End Sidebar -->

    <!-- Content -->
    <div class="w-full lg:ps-64">
      <div class="p-4 sm:p-6 space-y-4 sm:space-y-6">
        <router-view></router-view>
      </div>
    </div>
    <!-- End Content -->
  </div>
</template>
