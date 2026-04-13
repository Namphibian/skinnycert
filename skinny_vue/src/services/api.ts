import type {
  CertificateInfoResponse,
  CertificateListResponse,
  CreateCertificateRequest,
  HealthCheckResponse,
  KeyAlgorithmResponse,
  KeyAlgorithmTypeResponse,
  KeyAlgorithmTlsStatusResponse,
  KeyStatusResponse,
  KeyPairResponse,
  PatchCertificateRequest
} from '../types/api';

const API_BASE_URL = '/api'; // Adjusted via vite proxy if needed

async function request<T>(path: string, options?: RequestInit): Promise<T> {
  const response = await fetch(`${API_BASE_URL}${path}`, {
    ...options,
    headers: {
      'Content-Type': 'application/json',
      ...options?.headers,
    },
  });

  if (!response.ok) {
    const errorData = await response.json().catch(() => ({ error: 'Unknown error' }));
    throw new Error(errorData.error || `HTTP error! status: ${response.status}`);
  }

  if (response.status === 204) {
    return {} as T;
  }

  return response.json();
}

export const apiService = {
  // Certificates
  getCertificates(params?: Record<string, any>): Promise<CertificateListResponse> {
    const query = params ? '?' + new URLSearchParams(params).toString() : '';
    return request<CertificateListResponse>(`/certificates${query}`);
  },

  createCertificate(data: CreateCertificateRequest): Promise<CertificateInfoResponse> {
    return request<CertificateInfoResponse>('/certificates', {
      method: 'POST',
      body: JSON.stringify(data),
    });
  },

  getCertificateById(id: string): Promise<CertificateInfoResponse> {
    return request<CertificateInfoResponse>(`/certificates/${id}`);
  },

  deleteCertificate(id: string): Promise<void> {
    return request<void>(`/certificates/${id}`, { method: 'DELETE' });
  },

  patchCertificate(id: string, data: PatchCertificateRequest): Promise<CertificateInfoResponse> {
    return request<CertificateInfoResponse>(`/certificates/${id}`, {
      method: 'PATCH',
      body: JSON.stringify(data),
    });
  },

  // Keys
  getKeyAlgorithms(params?: Record<string, any>): Promise<KeyAlgorithmResponse[]> {
    const query = params ? '?' + new URLSearchParams(params).toString() : '';
    return request<KeyAlgorithmResponse[]>(`/keys${query}`);
  },

  getKeyAlgorithmById(id: string): Promise<KeyAlgorithmResponse> {
    return request<KeyAlgorithmResponse>(`/keys/${id}`);
  },

  generateKeyPair(id: string): Promise<KeyPairResponse> {
    return request<KeyPairResponse>(`/keys/${id}/keypair`);
  },

  // Metadata/Statuses
  getKeyAlgorithmTypes(): Promise<KeyAlgorithmTypeResponse[]> {
    return request<KeyAlgorithmTypeResponse[]>('/key_types');
  },

  getKeyAlgorithmStatuses(): Promise<KeyStatusResponse[]> {
    return request<KeyStatusResponse[]>('/key_statuses');
  },

  getKeyAlgorithmTypeTlsStatuses(): Promise<KeyAlgorithmTlsStatusResponse[]> {
    return request<KeyAlgorithmTlsStatusResponse[]>('/key_type_tls_statuses');
  },

  // Health
  getHealth(): Promise<HealthCheckResponse> {
    return request<HealthCheckResponse>('/health');
  },
};
