<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { apiService } from '../services/api';
import type { KeyAlgorithmResponse } from '../types/api';

const algorithms = ref<KeyAlgorithmResponse[]>([]);
const loading = ref(true);

onMounted(async () => {
  try {
    algorithms.value = await apiService.getKeyAlgorithms();
  } catch (err) {
    console.error(err);
  } finally {
    loading.value = false;
  }
});
</script>

<template>
  <div >
    <!-- Card -->
    <div class="flex flex-col">
      <div class="-m-1.5 overflow-x-auto">
        <div class="p-1.5 min-w-full inline-block align-middle">
          <div class="bg-white border border-gray-200 rounded-xl shadow-sm overflow-hidden dark:bg-neutral-900 dark:border-neutral-700">
            <!-- Header -->
            <div class="px-6 py-4 border-b border-gray-200 dark:border-neutral-700">
              <h2 class="text-xl font-semibold text-gray-800 dark:text-neutral-200">Key Algorithms</h2>
              <p class="text-sm text-gray-600 dark:text-neutral-400">Available algorithms for certificate generation.</p>
            </div>
            <!-- End Header -->

            <!-- Table -->
            <table class="min-w-full divide-y divide-gray-200 dark:divide-neutral-700">
              <thead class="bg-gray-50 dark:bg-neutral-800">
                <tr>
                  <th scope="col" class="px-6 py-3 text-start text-xs font-semibold text-gray-800 uppercase tracking-wide dark:text-neutral-200">
                    Display Name
                  </th>
                  <th scope="col" class="px-6 py-3 text-start text-xs font-semibold text-gray-800 uppercase tracking-wide dark:text-neutral-200">
                    Type
                  </th>
                  <th scope="col" class="px-6 py-3 text-start text-xs font-semibold text-gray-800 uppercase tracking-wide dark:text-neutral-200">
                    Strength / NID
                  </th>
                  <th scope="col" class="px-6 py-3 text-start text-xs font-semibold text-gray-800 uppercase tracking-wide dark:text-neutral-200">
                    Status
                  </th>
                </tr>
              </thead>

              <tbody class="divide-y divide-gray-200 dark:divide-neutral-700">
                <tr v-if="loading">
                  <td colspan="4" class="px-6 py-4 text-center text-sm text-gray-500">Loading algorithms...</td>
                </tr>
                <tr v-else v-for="alg in algorithms" :key="alg.id" class="hover:bg-gray-50 dark:hover:bg-neutral-800 transition-colors">
                  <td class="px-6 py-4 whitespace-nowrap">
                    <router-link :to="{ name: 'KeyDetail', params: { id: alg.id } }" class="text-sm font-semibold text-blue-600 hover:text-blue-800 dark:text-blue-500 dark:hover:text-blue-400">
                      {{ alg.displayName }}
                    </router-link>
                  </td>
                  <td class="px-6 py-4 whitespace-nowrap">
                    <span class="text-sm text-gray-600 dark:text-neutral-400">{{ alg.algorithmType.name }}</span>
                  </td>
                  <td class="px-6 py-4 whitespace-nowrap">
                    <div class="flex items-center gap-x-1">
                      <span v-if="alg.keyStrength" class="inline-flex items-center gap-x-1.5 py-1 px-2 rounded-full text-xs font-medium bg-blue-100 text-blue-800 dark:bg-blue-800/30 dark:text-blue-500">
                        Strength: {{ alg.keyStrength }}
                      </span>
                      <span v-if="alg.nidValue" class="inline-flex items-center gap-x-1.5 py-1 px-2 rounded-full text-xs font-medium bg-gray-100 text-gray-800 dark:bg-neutral-800 dark:text-neutral-200">
                        NID: {{ alg.nidValue }}
                      </span>
                    </div>
                  </td>
                  <td class="px-6 py-4 whitespace-nowrap">
                    <span class="text-xs text-gray-500 dark:text-neutral-500">{{ alg.algorithmStatus.name }}</span>
                  </td>
                </tr>
              </tbody>
            </table>
            <!-- End Table -->
          </div>
        </div>
      </div>
    </div>
    <!-- End Card -->
  </div>
</template>
