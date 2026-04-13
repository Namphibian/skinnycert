<script setup lang="ts">
import { ref, onMounted, computed } from 'vue';
import { useRouter } from 'vue-router';
import { apiService } from '../services/api';
import type { KeyAlgorithmResponse, CreateCertificateRequest, CertificateInfoResponse } from '../types/api';

const props = defineProps<{
  id?: string;
}>();

const router = useRouter();
const algorithms = ref<KeyAlgorithmResponse[]>([]);
const loading = ref(true);
const certificate = ref<CertificateInfoResponse | null>(null);

const isEditMode = computed(() => !!props.id);

const form = ref<CreateCertificateRequest>({
  key_algorithm_id: '',
  sans: [],
  subject: {
    country: '',
    email: '',
    locality: '',
    organization: '',
    organizational_unit: '',
    state_or_province: '',
  },
  validity_days: 365,
});

const sanInput = ref('');

const addSan = () => {
  if (sanInput.value && !form.value.sans.includes(sanInput.value)) {
    form.value.sans.push(sanInput.value);
    sanInput.value = '';
  }
};

const removeSan = (index: number) => {
  form.value.sans.splice(index, 1);
};

const handleSubmit = async () => {
  try {
    // If commonName is provided but not in SANS, the backend usually expects it handled or it's part of subject
    // In this API spec, SANS is a separate array.
    await apiService.createCertificate(form.value);
    router.push('/certificates');
  } catch (err: any) {
    alert('Error creating certificate: ' + err.message);
  }
};

onMounted(async () => {
  try {
    const algs = await apiService.getKeyAlgorithms();
    algorithms.value = algs;

    if (props.id) {
      const cert = await apiService.getCertificateById(props.id);
      certificate.value = cert;
      
      // Populate form for viewing
      form.value.key_algorithm_id = cert.keyAlgorithm.id;
      form.value.sans = cert.sans.sans || [];
      form.value.subject = {
        country: cert.subject?.country || '',
        email: cert.subject?.email || '',
        locality: cert.subject?.locality || '',
        organization: cert.subject?.organization || '',
        organizational_unit: cert.subject?.organizationalUnit || '',
        state_or_province: cert.subject?.stateOrProvince || '',
      };
      // validity_days is not in CertificateInfoResponse directly, it's used for creation
    } else if (algs.length > 0) {
      form.value.key_algorithm_id = algs[0].id;
    }
  } catch (err) {
    console.error('Failed to load data', err);
  } finally {
    loading.value = false;
  }
});
</script>

<template>
  <div>
    <!-- Breadcrumb -->
    <div class="mb-5">
      <router-link to="/certificates" class="inline-flex items-center gap-x-1 text-sm text-gray-600 hover:text-blue-600 dark:text-neutral-400 dark:hover:text-neutral-200">
        <svg class="shrink-0 size-4" xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m15 18-6-6 6-6"/></svg>
        Back to Certificates
      </router-link>
    </div>
    <div class="bg-white rounded-xl shadow p-4 sm:p-7 dark:bg-neutral-900">
      <div class="mb-8">
        <h2 class="text-xl font-bold text-gray-800 dark:text-neutral-200">
          {{ isEditMode ? 'Certificate Details' : 'Create New Certificate' }}
        </h2>
        <p class="text-sm text-gray-600 dark:text-neutral-400">
          {{ isEditMode ? 'View details of your certificate.' : 'Fill in the details to generate a new CSR and private key.' }}
        </p>
      </div>

      <div v-if="loading" class="text-center py-10">
        <div class="animate-spin inline-block size-6 border-[3px] border-current border-t-transparent text-blue-600 rounded-full dark:text-blue-500" role="status" aria-label="loading">
          <span class="sr-only">Loading...</span>
        </div>
      </div>

      <form v-else @submit.prevent="handleSubmit">
        <div class="grid grid-cols-12 gap-4 sm:gap-6">
          <!-- Algorithm -->
          <div class="col-span-12 sm:col-span-6">
            <label class="inline-block text-sm font-medium text-gray-800 mt-2.5 dark:text-neutral-200">Key Algorithm</label>
            <select v-model="form.key_algorithm_id" :disabled="isEditMode" class="py-2 px-3 pe-9 block w-full border-gray-200 shadow-sm text-sm rounded-lg focus:border-blue-500 focus:ring-blue-500 disabled:opacity-50 disabled:pointer-events-none dark:bg-neutral-900 dark:border-neutral-700 dark:text-neutral-400 dark:placeholder-neutral-500 dark:focus:ring-neutral-600">
              <option v-for="alg in algorithms" :key="alg.id" :value="alg.id">{{ alg.displayName }}</option>
            </select>
          </div>

          <!-- Validity -->
          <div v-if="!isEditMode" class="col-span-12 sm:col-span-6">
            <label class="inline-block text-sm font-medium text-gray-800 mt-2.5 dark:text-neutral-200">Validity (Days)</label>
            <input v-model.number="form.validity_days" type="number" class="py-2 px-3 block w-full border-gray-200 shadow-sm text-sm rounded-lg focus:border-blue-500 focus:ring-blue-500 dark:bg-neutral-900 dark:border-neutral-700 dark:text-neutral-400">
          </div>
          <div v-else class="col-span-12 sm:col-span-6">
            <label class="inline-block text-sm font-medium text-gray-800 mt-2.5 dark:text-neutral-200">Status</label>
            <div class="mt-2.5 flex items-center gap-1.5">
              <span v-if="certificate?.isSigned" class="py-1 px-1.5 inline-flex items-center gap-x-1 text-xs font-medium bg-teal-100 text-teal-800 rounded-full dark:bg-teal-500/10 dark:text-teal-500">
                Signed
              </span>
              <span v-else class="py-1 px-1.5 inline-flex items-center gap-x-1 text-xs font-medium bg-yellow-100 text-yellow-800 rounded-full dark:bg-yellow-500/10 dark:text-yellow-500">
                CSR Only
              </span>
              <span v-if="certificate?.isExpired" class="py-1 px-1.5 inline-flex items-center gap-x-1 text-xs font-medium bg-red-100 text-red-800 rounded-full dark:bg-red-500/10 dark:text-red-500">
                Expired
              </span>
            </div>
          </div>

          <hr class="col-span-12 my-4 border-gray-200 dark:border-neutral-700">

          <!-- Subject -->
          <div class="col-span-12">
            <h3 class="text-lg font-semibold text-gray-800 dark:text-neutral-200">Subject Information</h3>
          </div>

          <div class="col-span-12 sm:col-span-6">
            <label class="inline-block text-sm font-medium text-gray-800 mt-2.5 dark:text-neutral-200">Organization</label>
            <input v-model="form.subject.organization" :disabled="isEditMode" type="text" class="py-2 px-3 block w-full border-gray-200 shadow-sm text-sm rounded-lg focus:border-blue-500 focus:ring-blue-500 dark:bg-neutral-900 dark:border-neutral-700 dark:text-neutral-400 disabled:opacity-50">
          </div>

          <div class="col-span-12 sm:col-span-6">
            <label class="inline-block text-sm font-medium text-gray-800 mt-2.5 dark:text-neutral-200">Organizational Unit</label>
            <input v-model="form.subject.organizational_unit" :disabled="isEditMode" type="text" class="py-2 px-3 block w-full border-gray-200 shadow-sm text-sm rounded-lg focus:border-blue-500 focus:ring-blue-500 dark:bg-neutral-900 dark:border-neutral-700 dark:text-neutral-400 disabled:opacity-50">
          </div>

          <div class="col-span-12 sm:col-span-4">
            <label class="inline-block text-sm font-medium text-gray-800 mt-2.5 dark:text-neutral-200">Country (ISO)</label>
            <input v-model="form.subject.country" :disabled="isEditMode" type="text" maxlength="2" placeholder="US" class="py-2 px-3 block w-full border-gray-200 shadow-sm text-sm rounded-lg focus:border-blue-500 focus:ring-blue-500 dark:bg-neutral-900 dark:border-neutral-700 dark:text-neutral-400 disabled:opacity-50">
          </div>

          <div class="col-span-12 sm:col-span-4">
            <label class="inline-block text-sm font-medium text-gray-800 mt-2.5 dark:text-neutral-200">State/Province</label>
            <input v-model="form.subject.state_or_province" :disabled="isEditMode" type="text" class="py-2 px-3 block w-full border-gray-200 shadow-sm text-sm rounded-lg focus:border-blue-500 focus:ring-blue-500 dark:bg-neutral-900 dark:border-neutral-700 dark:text-neutral-400 disabled:opacity-50">
          </div>

          <div class="col-span-12 sm:col-span-4">
            <label class="inline-block text-sm font-medium text-gray-800 mt-2.5 dark:text-neutral-200">Locality</label>
            <input v-model="form.subject.locality" :disabled="isEditMode" type="text" class="py-2 px-3 block w-full border-gray-200 shadow-sm text-sm rounded-lg focus:border-blue-500 focus:ring-blue-500 dark:bg-neutral-900 dark:border-neutral-700 dark:text-neutral-400 disabled:opacity-50">
          </div>

          <hr class="col-span-12 my-4 border-gray-200 dark:border-neutral-700">

          <!-- SANS -->
          <div class="col-span-12">
            <h3 class="text-lg font-semibold text-gray-800 dark:text-neutral-200">Subject Alternative Names (SANs)</h3>
          </div>

          <div class="col-span-12">
            <div v-if="!isEditMode" class="flex gap-2">
              <input v-model="sanInput" @keyup.enter="addSan" type="text" placeholder="example.com" class="py-2 px-3 block w-full border-gray-200 shadow-sm text-sm rounded-lg focus:border-blue-500 focus:ring-blue-500 dark:bg-neutral-900 dark:border-neutral-700 dark:text-neutral-400">
              <button type="button" @click="addSan" class="py-2 px-3 inline-flex items-center gap-x-2 text-sm font-semibold rounded-lg border border-transparent bg-blue-600 text-white hover:bg-blue-700">Add</button>
            </div>
            <div class="mt-2 flex flex-wrap gap-2">
              <span v-for="(san, index) in form.sans" :key="index" class="inline-flex items-center gap-x-1.5 py-1.5 px-3 rounded-full text-xs font-medium bg-gray-100 text-gray-800 dark:bg-neutral-800 dark:text-neutral-200">
                {{ san }}
                <button v-if="!isEditMode" type="button" @click="removeSan(index)" class="shrink-0 size-4 inline-flex items-center justify-center rounded-full hover:bg-gray-200 dark:hover:bg-neutral-700">
                  <svg class="size-3" xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M18 6 6 18"/><path d="m6 6 12 12"/></svg>
                </button>
              </span>
            </div>
          </div>
        </div>

        <div class="mt-8 flex justify-end gap-x-2">
          <router-link to="/certificates" class="py-2 px-3 inline-flex items-center gap-x-2 text-sm font-medium rounded-lg border border-gray-200 bg-white text-gray-800 shadow-sm hover:bg-gray-50 dark:bg-neutral-900 dark:border-neutral-700 dark:text-white dark:hover:bg-neutral-800">
            {{ isEditMode ? 'Back' : 'Cancel' }}
          </router-link>
          <button v-if="!isEditMode" type="submit" class="py-2 px-3 inline-flex items-center gap-x-2 text-sm font-semibold rounded-lg border border-transparent bg-blue-600 text-white hover:bg-blue-700 disabled:opacity-50 disabled:pointer-events-none">
            Create Certificate
          </button>
        </div>
      </form>
    </div>
  </div>
</template>
