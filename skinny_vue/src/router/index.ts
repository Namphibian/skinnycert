import { createRouter, createWebHistory } from 'vue-router';
import CertificateList from '../views/CertificateList.vue';
import CertificateForm from '../views/CertificateForm.vue';
import KeyAlgorithmList from '../views/KeyAlgorithmList.vue';
import KeyAlgorithmDetail from '../views/KeyAlgorithmDetail.vue';
import HealthCheck from '../views/HealthCheck.vue';

const routes = [
  { path: '/', redirect: '/certificates' },
  { path: '/certificates', name: 'Certificates', component: CertificateList },
  { path: '/certificates/new', name: 'NewCertificate', component: CertificateForm },
  { path: '/certificates/:id', name: 'CertificateDetail', component: CertificateForm, props: true },
  { path: '/keys', name: 'Keys', component: KeyAlgorithmList },
  { path: '/keys/:id', name: 'KeyDetail', component: KeyAlgorithmDetail },
  { path: '/health', name: 'Health', component: HealthCheck },
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

export default router;
