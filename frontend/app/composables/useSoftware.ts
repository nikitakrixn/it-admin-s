export interface Software {
  id: number;
  name: string;
  manufacturer: string | null;
  category: string | null;
  description: string | null;
  website: string | null;
  is_system: boolean;
  requires_license: boolean;
  created_at: string;
  updated_at: string;
}

export interface SoftwareListResponse {
  software: Software[];
  total: number;
  page: number;
  per_page: number;
}

export interface CreateSoftwareData {
  name: string;
  manufacturer?: string;
  category?: string;
  description?: string;
  website?: string;
  is_system?: boolean;
  requires_license?: boolean;
}

export interface UpdateSoftwareData {
  name?: string;
  manufacturer?: string;
  category?: string;
  description?: string;
  website?: string;
  is_system?: boolean;
  requires_license?: boolean;
}

export const useSoftware = () => {
  const { $api } = useNuxtApp();

  const fetchSoftware = async (params?: {
    page?: number;
    per_page?: number;
    category?: string;
    search?: string;
  }) => {
    return await $api<SoftwareListResponse>("/software", {
      params,
    });
  };

  const fetchSoftwareById = async (id: number) => {
    return await $api<Software>(`/software/${id}`);
  };

  const createSoftware = async (data: CreateSoftwareData) => {
    return await $api<Software>("/software", {
      method: "POST",
      body: data,
    });
  };

  const updateSoftware = async (id: number, data: UpdateSoftwareData) => {
    return await $api<Software>(`/software/${id}`, {
      method: "PUT",
      body: data,
    });
  };

  const deleteSoftware = async (id: number) => {
    return await $api(`/software/${id}`, {
      method: "DELETE",
    });
  };

  return {
    fetchSoftware,
    fetchSoftwareById,
    createSoftware,
    updateSoftware,
    deleteSoftware,
  };
};
