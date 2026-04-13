<script setup lang="ts">
import { onMounted, ref } from 'vue';
import { apiService } from '../services/api';
import type { CertificateInfoResponse } from '../types/api';

const certificates = ref<CertificateInfoResponse[]>([]);
const nextPageToken = ref<string | null>(null);
const prevPageToken = ref<string | null>(null);
const isFirstPage = ref(true);
const limit = ref(5);
const loading = ref(true);
const error = ref<string | null>(null);

// Filter state
const filters = ref({
  commonName: '',
  organization: '',
  isSigned: null as boolean | null,
  isExpired: null as boolean | null,
  algorithmTypeName: '',
});

const fetchCertificates = async (pageToken?: string | null, direction?: string) => {
  loading.value = true;
  try {
    const params: Record<string, any> = {
      limit: limit.value
    };
    if (pageToken) params.pageToken = pageToken;
    if (direction) params.direction = direction;
    
    // Add filters to params
    if (filters.value.commonName) params.commonName = filters.value.commonName;
    if (filters.value.organization) params.organization = filters.value.organization;
    if (filters.value.isSigned !== null) params.isSigned = filters.value.isSigned;
    if (filters.value.isExpired !== null) params.isExpired = filters.value.isExpired;
    if (filters.value.algorithmTypeName) params.algorithmTypeName = filters.value.algorithmTypeName;
    
    const response = await apiService.getCertificates(params);
    certificates.value = response.items;
    nextPageToken.value = response.nextPageToken;
    prevPageToken.value = response.prevPageToken;
    
    // If we didn't provide a pageToken, we are on the first page
    if (!pageToken) {
      isFirstPage.value = true;
    } else if (direction === 'next') {
      isFirstPage.value = false;
    } else if (direction === 'prev' && !response.prevPageToken) {
      // If we went back and now there's no more prev token, we are likely back at the start
      isFirstPage.value = true;
    }
  } catch (err: any) {
    error.value = err.message;
  } finally {
    loading.value = false;
  }
};

const goToNextPage = () => {
  if (nextPageToken.value) {
    fetchCertificates(nextPageToken.value, 'next');
  }
};

const goToPrevPage = () => {
  if (prevPageToken.value) {
    fetchCertificates(prevPageToken.value, 'prev');
  }
};

const handleLimitChange = () => {
  fetchCertificates();
};

const handleFilter = () => {
  fetchCertificates();
};

const clearFilters = () => {
  filters.value = {
    commonName: '',
    organization: '',
    isSigned: null,
    isExpired: null,
    algorithmTypeName: '',
  };
  fetchCertificates();
};

const deleteCertificate = async (id: string) => {
  if (!confirm('Are you sure you want to delete this certificate?')) return;
  try {
    await apiService.deleteCertificate(id);
    certificates.value = certificates.value.filter(c => c.id !== id);
  } catch (err: any) {
    alert('Failed to delete: ' + err.message);
  }
};

const downloadCsr = (commonName: string | null, csrPem: string) => {
  const name = (commonName || 'certificate').replace(/\./g, '_');
  const filename = `${name}.csr`;
  const blob = new Blob([csrPem], { type: 'application/x-pem-file' });
  const url = URL.createObjectURL(blob);
  const link = document.createElement('a');
  link.href = url;
  link.download = filename;
  document.body.appendChild(link);
  link.click();
  document.body.removeChild(link);
  URL.revokeObjectURL(url);
};

onMounted(fetchCertificates);
</script>

<template>
  <div >
    <div class="flex flex-col">
      <div class="-m-1.5 overflow-x-auto">
        <div class="p-1.5 min-w-full inline-block align-middle">
          <div class="bg-white border border-gray-200 rounded-xl shadow-sm overflow-hidden dark:bg-neutral-900 dark:border-neutral-700">
            <!-- Header -->
            <div class="px-6 py-4 border-b border-gray-200 dark:border-neutral-700">
              <div class="grid gap-3 md:flex md:justify-between md:items-center">
                <div>
                  <h2 class="text-xl font-semibold text-gray-800 dark:text-neutral-200">Certificates</h2>
                  <p class="text-sm text-gray-600 dark:text-neutral-400">Manage your TLS certificates.</p>
                </div>
                <div class="flex items-center gap-x-2">
                  <div class="inline-flex items-center gap-x-1.5">
                    <label for="limit" class="text-sm text-gray-600 dark:text-neutral-400">Show:</label>
                    <select id="limit" v-model="limit" @change="handleLimitChange" class="py-2 px-3 pe-9 block w-full border-gray-200 shadow-sm text-sm rounded-lg focus:border-blue-500 focus:ring-blue-500 disabled:opacity-50 disabled:pointer-events-none dark:bg-neutral-900 dark:border-neutral-700 dark:text-neutral-400 dark:placeholder-neutral-500 dark:focus:ring-neutral-600">
                      <option :value="5">5</option>
                      <option :value="10">10</option>
                      <option :value="25">25</option>
                      <option :value="50">50</option>
                      <option :value="100">100</option>
                    </select>
                  </div>
                  <router-link to="/certificates/new" class="py-2 px-3 inline-flex items-center gap-x-2 text-sm font-semibold rounded-lg border border-transparent bg-blue-600 text-white hover:bg-blue-700 disabled:opacity-50 disabled:pointer-events-none focus:outline-none focus:ring-2 focus:ring-blue-500">
                    <svg class="size-4" xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M5 12h14"/><path d="M12 5v14"/></svg>
                    Create Certificate
                  </router-link>
                </div>
              </div>

              <!-- Filter Criteria -->
              <div class="mt-4 p-2 bg-gray-50 border border-gray-200 rounded-lg dark:bg-neutral-800 dark:border-neutral-700">
                <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-5 gap-2">
                  <div>
                    <label class="block text-sm font-medium mb-1 dark:text-neutral-200">Common Name</label>
                    <input v-model="filters.commonName" @keyup.enter="handleFilter" type="text" placeholder="e.g. example.com" class="py-2 px-3 block w-full border-gray-200 shadow-sm text-sm rounded-lg focus:border-blue-500 focus:ring-blue-500 dark:bg-neutral-900 dark:border-neutral-700 dark:text-neutral-400">
                  </div>
                  <div>
                    <label class="block text-sm font-medium mb-1 dark:text-neutral-200">Organization</label>
                    <input v-model="filters.organization" @keyup.enter="handleFilter" type="text" placeholder="e.g. Acme Inc" class="py-2 px-3 block w-full border-gray-200 shadow-sm text-sm rounded-lg focus:border-blue-500 focus:ring-blue-500 dark:bg-neutral-900 dark:border-neutral-700 dark:text-neutral-400">
                  </div>
                  <div>
                    <label class="block text-sm font-medium mb-1 dark:text-neutral-200">Signed Status</label>
                    <select v-model="filters.isSigned" @change="handleFilter" class="py-2 px-3 pe-9 block w-full border-gray-200 shadow-sm text-sm rounded-lg focus:border-blue-500 focus:ring-blue-500 dark:bg-neutral-900 dark:border-neutral-700 dark:text-neutral-400">
                      <option :value="null">All</option>
                      <option :value="true">Signed</option>
                      <option :value="false">CSR Only</option>
                    </select>
                  </div>
                  <div>
                    <label class="block text-sm font-medium mb-1 dark:text-neutral-200">Expiry Status</label>
                    <select v-model="filters.isExpired" @change="handleFilter" class="py-2 px-3 pe-9 block w-full border-gray-200 shadow-sm text-sm rounded-lg focus:border-blue-500 focus:ring-blue-500 dark:bg-neutral-900 dark:border-neutral-700 dark:text-neutral-400">
                      <option :value="null">All</option>
                      <option :value="true">Expired</option>
                      <option :value="false">Valid</option>
                    </select>
                  </div>
                  <div>
                    <label class="block text-sm font-medium mb-1 dark:text-neutral-200">Algorithm Type</label>
                    <input v-model="filters.algorithmTypeName" @keyup.enter="handleFilter" type="text" placeholder="e.g. RSA" class="py-2 px-3 block w-full border-gray-200 shadow-sm text-sm rounded-lg focus:border-blue-500 focus:ring-blue-500 dark:bg-neutral-900 dark:border-neutral-700 dark:text-neutral-400">
                  </div>
                </div>
                <div class="mt-4 flex justify-end gap-x-2">
                  <button @click="clearFilters" type="button" class="py-2 px-3 inline-flex items-center gap-x-2 text-sm font-medium rounded-lg border border-gray-200 bg-white text-gray-800 shadow-sm hover:bg-gray-50 dark:bg-neutral-900 dark:border-neutral-700 dark:text-white dark:hover:bg-neutral-800">
                    Clear Filters
                  </button>
                  <button @click="handleFilter" type="button" class="py-2 px-3 inline-flex items-center gap-x-2 text-sm font-semibold rounded-lg border border-transparent bg-blue-600 text-white hover:bg-blue-700">
                    Search
                  </button>
                </div>
              </div>
            </div>
            <!-- End Header -->

            <!-- Table -->
            <div class="flex flex-col">
              <div class="-m-1.5 overflow-x-auto">
                <div class="p-1.5 min-w-full inline-block align-middle">
                  <div class="overflow-hidden">
                    <table class="min-w-full divide-y divide-gray-200 dark:divide-neutral-700">
                      <thead class="bg-gray-50 dark:bg-neutral-800">
                        <tr>
                          <th scope="col" class="px-6 py-3 text-start text-xs font-semibold text-gray-800 uppercase tracking-wide dark:text-neutral-200">
                            Common Name
                          </th>
                          <th scope="col" class="px-6 py-3 text-start text-xs font-semibold text-gray-800 uppercase tracking-wide dark:text-neutral-200">
                            Algorithm
                          </th>
                          <th scope="col" class="px-6 py-3 text-start text-xs font-semibold text-gray-800 uppercase tracking-wide dark:text-neutral-200">
                            Type
                          </th>
                          <th scope="col" class="px-6 py-3 text-start text-xs font-semibold text-gray-800 uppercase tracking-wide dark:text-neutral-200">
                            Status
                          </th>
                          <th scope="col" class="px-6 py-3 text-start text-xs font-semibold text-gray-800 uppercase tracking-wide dark:text-neutral-200">
                            Created
                          </th>
                          <th scope="col" class="px-6 py-3 text-start text-xs font-semibold text-gray-800 uppercase tracking-wide dark:text-neutral-200">
                            CSR
                          </th>
                          <th scope="col" class="px-6 py-3 text-end text-xs font-semibold text-gray-800 uppercase tracking-wide dark:text-neutral-200"></th>
                        </tr>
                      </thead>
                      <tbody class="divide-y divide-gray-200 dark:divide-neutral-700">
                        <tr v-if="loading">
                          <td colspan="7" class="px-6 py-4 text-center text-sm text-gray-500">Loading certificates...</td>
                        </tr>
                        <tr v-else-if="error">
                          <td colspan="7" class="px-6 py-4 text-center text-sm text-red-500 font-medium">Error: {{ error }}</td>
                        </tr>
                        <tr v-else-if="certificates.length === 0">
                          <td colspan="7" class="px-6 py-4 text-center text-sm text-gray-500">No certificates found.</td>
                        </tr>
                        <tr v-for="cert in certificates" :key="cert.id" class="hover:bg-gray-50 dark:hover:bg-neutral-800 transition-colors">
                          <td class="px-6 py-4 whitespace-nowrap">
                            <router-link :to="{ name: 'CertificateDetail', params: { id: cert.id } }" class="text-sm font-medium text-blue-600 hover:text-blue-800 dark:text-blue-500 dark:hover:text-blue-400">
                              {{ cert.sans.commonName || 'N/A' }}
                            </router-link>
                          </td>
                          <td class="px-6 py-4 whitespace-nowrap">
                            <span class="text-sm text-gray-600 dark:text-neutral-400">
                              {{ cert.keyAlgorithm.displayName }}
                            </span>
                          </td>
                          <td class="px-6 py-4 whitespace-nowrap">
                            <span class="text-sm text-gray-600 dark:text-neutral-400">
                              {{ cert.keyAlgorithm.algorithmType.name }}
                            </span>
                          </td>
                          <td class="px-6 py-4 whitespace-nowrap">
                            <div class="flex items-center gap-1.5">
                              <span v-if="cert.isSigned" class="py-1 px-1.5 inline-flex items-center gap-x-1 text-xs font-medium bg-teal-100 text-teal-800 rounded-full dark:bg-teal-500/10 dark:text-teal-500">
                                Signed
                              </span>
                              <span v-else class="py-1 px-1.5 inline-flex items-center gap-x-1 text-xs font-medium bg-yellow-100 text-yellow-800 rounded-full dark:bg-yellow-500/10 dark:text-yellow-500">
                                CSR Only
                              </span>
                              <span v-if="cert.isExpired" class="py-1 px-1.5 inline-flex items-center gap-x-1 text-xs font-medium bg-red-100 text-red-800 rounded-full dark:bg-red-500/10 dark:text-red-500">
                                Expired
                              </span>
                            </div>
                          </td>
                          <td class="px-6 py-4 whitespace-nowrap">
                            <span class="text-sm text-gray-600 dark:text-neutral-400">
                              {{ new Date(cert.createdOn).toLocaleDateString() }}
                            </span>
                          </td>
                          <td class="px-6 py-4 whitespace-nowrap">
                            <button @click="downloadCsr(cert.sans.commonName, cert.pem.csrPem)" type="button" class="inline-flex items-center gap-x-2 text-sm font-medium text-blue-600 hover:text-blue-800 dark:text-blue-500 dark:hover:text-blue-400">
                              <svg class="shrink-0 size-4" xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/><polyline points="7 10 12 15 17 10"/><line x1="12" x2="12" y1="15" y2="3"/></svg>

                            </button>
                          </td>
                          <td class="px-6 py-4 whitespace-nowrap text-end text-sm font-medium">
                            <button @click="deleteCertificate(cert.id)" class="inline-flex items-center gap-x-2 text-sm font-semibold rounded-lg border border-transparent text-red-600 hover:text-red-800 focus:outline-none focus:text-red-800 disabled:opacity-50 disabled:pointer-events-none dark:text-red-500 dark:hover:text-red-400 dark:focus:text-red-400">
                              Delete
                            </button>
                          </td>
                        </tr>
                      </tbody>
                    </table>
                  </div>
                </div>
              </div>
            </div>
            <!-- End Table -->

            <!-- Footer -->
            <div class="px-6 py-4 grid gap-3 md:flex md:justify-between md:items-center border-t border-gray-200 dark:border-neutral-700">
              <div>
                <p class="text-sm text-gray-600 dark:text-neutral-400">
                  <span class="font-semibold text-gray-800 dark:text-neutral-200">{{ certificates.length }}</span> results
                </p>
              </div>

              <div>
                <div class="inline-flex gap-x-2">
                  <button type="button" 
                    @click="goToPrevPage"
                    :disabled="isFirstPage || !prevPageToken || loading"
                    class="py-2 px-3 inline-flex items-center gap-x-2 text-sm font-medium rounded-lg border border-gray-200 bg-white text-gray-800 shadow-sm hover:bg-gray-50 disabled:opacity-50 disabled:pointer-events-none focus:outline-none focus:bg-gray-50 dark:bg-neutral-900 dark:border-neutral-700 dark:text-white dark:hover:bg-neutral-800 dark:focus:bg-neutral-800">
                    <svg class="shrink-0 size-4" xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m15 18-6-6 6-6"/></svg>
                    Prev
                  </button>

                  <button type="button"
                    @click="goToNextPage"
                    :disabled="!nextPageToken || loading"
                    class="py-2 px-3 inline-flex items-center gap-x-2 text-sm font-medium rounded-lg border border-gray-200 bg-white text-gray-800 shadow-sm hover:bg-gray-50 disabled:opacity-50 disabled:pointer-events-none focus:outline-none focus:bg-gray-50 dark:bg-neutral-900 dark:border-neutral-700 dark:text-white dark:hover:bg-neutral-800 dark:focus:bg-neutral-800">
                    Next
                    <svg class="shrink-0 size-4" xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m9 18 6-6-6-6"/></svg>
                  </button>
                </div>
              </div>
            </div>
            <!-- End Footer -->
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
