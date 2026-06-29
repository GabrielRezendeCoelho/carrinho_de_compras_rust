import api from './api';

export const produtoService = {
  async listar(filtros = {}) {
    const params = new URLSearchParams();
    if (filtros.pagina) params.append('pagina', filtros.pagina);
    if (filtros.por_pagina) params.append('por_pagina', filtros.por_pagina);
    if (filtros.nome) params.append('nome', filtros.nome);
    if (filtros.categoria_id) params.append('categoria_id', filtros.categoria_id);

    const response = await api.get(`/produtos?${params.toString()}`);
    return response.data;
  },

  async buscarPorId(id) {
    const response = await api.get(`/produtos/${id}`);
    return response.data;
  },

  async criar(dados) {
    const response = await api.post('/produtos', dados);
    return response.data;
  },

  async atualizar(id, dados) {
    const response = await api.put(`/produtos/${id}`, dados);
    return response.data;
  },

  async remover(id) {
    const response = await api.delete(`/produtos/${id}`);
    return response.data;
  },
};
