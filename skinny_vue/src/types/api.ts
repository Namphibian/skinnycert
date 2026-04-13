export interface CertificateInfoResponse {
  id: string;
  createdOn: string;
  updatedOn: string;
  isSigned: boolean;
  isExpired: boolean;
  pem: PemDataResponse;
  keyAlgorithm: KeyAlgorithmResponse;
  subject: SubjectDataResponse;
  sans: SansDataResponse;
  x509: X509MetadataResponse;
  certUploadedOn: string | null;
  deletedOn: string | null;
}

export interface CertificateListResponse {
  items: CertificateInfoResponse[];
  limit: number;
  nextPageToken: string | null;
  prevPageToken: string | null;
}

export interface CertificateSubject {
  country?: string | null;
  email?: string | null;
  locality?: string | null;
  organization?: string | null;
  organizational_unit?: string | null;
  state_or_province?: string | null;
}

export interface CreateCertificateRequest {
  key_algorithm_id: string;
  sans: string[];
  subject: CertificateSubject;
  validity_days?: number;
}

export interface ErrorResponse {
  error: string;
  details?: string | null;
}

export interface HealthCheckResponse {
  memoryInfo: MemoryInfo;
}

export interface KeyAlgorithmResponse {
  id: string;
  displayName: string;
  createdOn: string;
  algorithmStatus: KeyAlgorithmStatusResponse;
  algorithmType: KeyAlgorithmTypeResponse;
  keyStrength: number | null;
  nidValue: number | null;
  updatedOn: string | null;
}

export interface KeyAlgorithmStatusResponse {
  id: string;
  name: string;
  createdOn: string;
  description: string | null;
  updatedOn: string | null;
}

export interface KeyAlgorithmTlsStatusResponse {
  id: string;
  name: string;
  createdOn: string;
  description: string | null;
  updatedOn: string | null;
}

export interface KeyAlgorithmTypeResponse {
  id: string;
  name: string;
  requiresNid: boolean;
  requiresStrength: boolean;
  tlsStatus: KeyAlgorithmTlsStatusResponse;
  createdOn: string;
  description: string | null;
  updatedOn: string | null;
}

export interface KeyPairResponse {
  publicKey: string;
  privateKey: string;
}

export interface KeyStatusResponse {
  id: string;
  name: string;
  createdOn: string;
  description: string | null;
  updatedOn: string | null;
}

export interface MemoryInfo {
  availableMemoryKb: number;
  freeMemoryKb: number;
  processMemoryKb: number;
  totalMemoryKb: number;
}

export type PageDirection = 'next' | 'prev';

export interface PatchCertificateRequest {
  cert_pem: string;
  chain_pem?: string | null;
}

export interface PemDataResponse {
  csrPem: string;
  keyPem: string;
  publicKeyPem: string;
  certPem: string | null;
  chainPem: string | null;
}

export interface SansDataResponse {
  commonName: string | null;
  sans: string[];
}

export interface SubjectDataResponse {
  country: string | null;
  email: string | null;
  locality: string | null;
  organization: string | null;
  organizationalUnit: string | null;
  stateOrProvince: string | null;
}

export interface X509MetadataResponse {
  fingerprint: string | null;
  validFrom: string | null;
  validTo: string | null;
}
