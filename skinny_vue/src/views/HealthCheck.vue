<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { apiService } from '../services/api';
import type { HealthCheckResponse } from '../types/api';

const health = ref<HealthCheckResponse | null>(null);
const loading = ref(true);

onMounted(async () => {
  try {
    health.value = await apiService.getHealth();
  } catch (err) {
    console.error(err);
  } finally {
    loading.value = false;
  }
});

const formatKb = (kb: number) => {
  if (kb > 1024 * 1024) return `${(kb / (1024 * 1024)).toFixed(2)} GB`;
  if (kb > 1024) return `${(kb / 1024).toFixed(2)} MB`;
  return `${kb} KB`;
};
</script>

<template>
  <div >
    <div class="bg-white border border-gray-200 rounded-xl shadow-sm overflow-hidden dark:bg-neutral-900 dark:border-neutral-700">
      <div class="px-6 py-4 border-b border-gray-200 dark:border-neutral-700">
        <h2 class="text-xl font-semibold text-gray-800 dark:text-neutral-200">System Health</h2>
      </div>

      <div class="p-6">
        <div v-if="loading" class="text-center py-10">Loading health info...</div>
        <div v-else-if="health" class="grid sm:grid-cols-2 gap-6">
          <div class="flex flex-col bg-white border border-gray-200 rounded-xl p-4 md:p-5 dark:bg-neutral-900 dark:border-neutral-700 dark:text-neutral-400">
            <h3 class="text-lg font-bold text-gray-800 dark:text-white">Total System Memory</h3>
            <p class="mt-2 text-gray-500 dark:text-neutral-500">{{ formatKb(health.memoryInfo.totalMemoryKb) }}</p>
          </div>
          <div class="flex flex-col bg-white border border-gray-200 rounded-xl p-4 md:p-5 dark:bg-neutral-900 dark:border-neutral-700 dark:text-neutral-400">
            <h3 class="text-lg font-bold text-gray-800 dark:text-white">Available System Memory</h3>
            <p class="mt-2 text-gray-500 dark:text-neutral-500">{{ formatKb(health.memoryInfo.availableMemoryKb) }}</p>
          </div>
          <div class="flex flex-col bg-white border border-gray-200 rounded-xl p-4 md:p-5 dark:bg-neutral-900 dark:border-neutral-700 dark:text-neutral-400">
            <h3 class="text-lg font-bold text-gray-800 dark:text-white">Free System Memory</h3>
            <p class="mt-2 text-gray-500 dark:text-neutral-500">{{ formatKb(health.memoryInfo.freeMemoryKb) }}</p>
          </div>
          <div class="flex flex-col bg-white border border-gray-200 rounded-xl p-4 md:p-5 dark:bg-neutral-900 dark:border-neutral-700 dark:text-neutral-400">
            <h3 class="text-lg font-bold text-gray-800 dark:text-white">Process Memory Usage</h3>
            <p class="mt-2 text-gray-500 dark:text-neutral-500">{{ formatKb(health.memoryInfo.processMemoryKb) }}</p>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
