<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { useRoute } from 'vue-router';
import { apiService } from '../services/api';
import type { KeyAlgorithmResponse, KeyPairResponse } from '../types/api';

const route = useRoute();
const id = route.params.id as string;

const algorithm = ref<KeyAlgorithmResponse | null>(null);
const loading = ref(true);
const error = ref<string | null>(null);
const generating = ref(false);
const keyPair = ref<KeyPairResponse | null>(null);

const fetchDetail = async () => {
  loading.value = true;
  try {
    algorithm.value = await apiService.getKeyAlgorithmById(id);
  } catch (err: any) {
    error.value = err.message;
  } finally {
    loading.value = false;
  }
};

const generateKeys = async () => {
  if (!algorithm.value) return;
  generating.value = true;
  try {
    keyPair.value = await apiService.generateKeyPair(algorithm.value.id);
  } catch (err: any) {
    alert('Failed to generate key pair: ' + err.message);
  } finally {
    generating.value = false;
  }
};

const copyToClipboard = async (text: string) => {
  try {
    await navigator.clipboard.writeText(text);
    alert('Copied to clipboard!');
  } catch (err) {
    console.error('Failed to copy: ', err);
  }
};

onMounted(fetchDetail);
</script>

<template>
  <div>
    <!-- Breadcrumb -->
    <div class="mb-5">
      <router-link to="/keys" class="inline-flex items-center gap-x-1 text-sm text-gray-600 hover:text-blue-600 dark:text-neutral-400 dark:hover:text-neutral-200">
        <svg class="shrink-0 size-4" xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m15 18-6-6 6-6"/></svg>
        Back to Algorithms
      </router-link>
    </div>

    <div v-if="loading" class="text-center py-10">
      <div class="animate-spin inline-block size-6 border-[3px] border-current border-t-transparent text-blue-600 rounded-full dark:text-blue-500" role="status" aria-label="loading">
        <span class="sr-only">Loading...</span>
      </div>
    </div>

    <div v-else-if="error" class="bg-red-50 border border-red-200 text-red-800 rounded-lg p-4 dark:bg-red-800/10 dark:border-red-900 dark:text-red-500">
      {{ error }}
    </div>

    <div v-else-if="algorithm" class="space-y-6">
      <!-- Algorithm Info -->
      <div class="bg-white border border-gray-200 rounded-xl shadow-sm overflow-hidden dark:bg-neutral-900 dark:border-neutral-700">
        <div class="px-6 py-4 border-b border-gray-200 dark:border-neutral-700">
          <h2 class="text-xl font-semibold text-gray-800 dark:text-neutral-200">
            {{ algorithm.displayName }}
          </h2>
          <p class="text-sm text-gray-600 dark:text-neutral-400">
            Detailed information about this key algorithm.
          </p>
        </div>

        <div class="p-6">
          <dl class="grid sm:grid-cols-2 gap-4 sm:gap-6">
            <div>
              <dt class="text-sm font-medium text-gray-500 dark:text-neutral-500">Algorithm Type</dt>
              <dd class="text-sm text-gray-800 dark:text-neutral-200">{{ algorithm.algorithmType.name }}</dd>
            </div>
            <div>
              <dt class="text-sm font-medium text-gray-500 dark:text-neutral-500">Status</dt>
              <dd class="text-sm text-gray-800 dark:text-neutral-200">{{ algorithm.algorithmStatus.name }}</dd>
            </div>
            <div v-if="algorithm.keyStrength">
              <dt class="text-sm font-medium text-gray-500 dark:text-neutral-500">Key Strength</dt>
              <dd class="text-sm text-gray-800 dark:text-neutral-200">{{ algorithm.keyStrength }}</dd>
            </div>
            <div v-if="algorithm.nidValue">
              <dt class="text-sm font-medium text-gray-500 dark:text-neutral-500">NID Value</dt>
              <dd class="text-sm text-gray-800 dark:text-neutral-200">{{ algorithm.nidValue }}</dd>
            </div>
            <div class="sm:col-span-2">
              <dt class="text-sm font-medium text-gray-500 dark:text-neutral-500">Description</dt>
              <dd class="text-sm text-gray-800 dark:text-neutral-200">{{ algorithm.algorithmType.description || 'No description available.' }}</dd>
            </div>
          </dl>
        </div>
      </div>

      <!-- Key Pair Generation -->
      <div class="bg-white border border-gray-200 rounded-xl shadow-sm overflow-hidden dark:bg-neutral-900 dark:border-neutral-700">
        <div class="px-6 py-4 border-b border-gray-200 dark:border-neutral-700 flex justify-between items-center">
          <div>
            <h2 class="text-xl font-semibold text-gray-800 dark:text-neutral-200">Key Pair Generation</h2>
            <p class="text-sm text-gray-600 dark:text-neutral-400">Generate a new crypto key pair using this algorithm.</p>
          </div>
          <button 
            @click="generateKeys" 
            :disabled="generating"
            type="button" 
            class="py-2 px-3 inline-flex items-center gap-x-2 text-sm font-semibold rounded-lg border border-transparent bg-blue-600 text-white hover:bg-blue-700 disabled:opacity-50 disabled:pointer-events-none focus:outline-none focus:ring-2 focus:ring-blue-500"
          >
            <span v-if="generating" class="animate-spin inline-block size-4 border-[2px] border-current border-t-transparent text-white rounded-full" role="status" aria-label="loading"></span>
            {{ generating ? 'Generating...' : 'Generate Key Pair' }}
          </button>
        </div>

        <div v-if="keyPair" class="p-6 space-y-4">
          <div>
            <div class="flex justify-between items-center mb-1">
              <label class="block text-sm font-medium dark:text-white">Public Key</label>
              <button @click="copyToClipboard(keyPair.publicKey)" class="text-xs text-blue-600 hover:text-blue-800 dark:text-blue-500">Copy</button>
            </div>
            <textarea readonly rows="5" class="py-2 px-3 block w-full border-gray-200 rounded-lg text-xs font-mono focus:border-blue-500 focus:ring-blue-500 dark:bg-neutral-900 dark:border-neutral-700 dark:text-neutral-400" :value="keyPair.publicKey"></textarea>
          </div>
          <div>
            <div class="flex justify-between items-center mb-1">
              <label class="block text-sm font-medium dark:text-white">Private Key</label>
              <button @click="copyToClipboard(keyPair.privateKey)" class="text-xs text-blue-600 hover:text-blue-800 dark:text-blue-500">Copy</button>
            </div>
            <textarea readonly rows="8" class="py-2 px-3 block w-full border-gray-200 rounded-lg text-xs font-mono focus:border-blue-500 focus:ring-blue-500 dark:bg-neutral-900 dark:border-neutral-700 dark:text-neutral-400" :value="keyPair.privateKey"></textarea>
            <p class="mt-2 text-xs text-red-500">Warning: Store your private key securely. It will not be shown again.</p>
          </div>
        </div>
        <div v-else class="p-12 text-center">
          <p class="text-sm text-gray-500 dark:text-neutral-500">No key pair generated yet.</p>
        </div>
      </div>
    </div>
  </div>
</template>
